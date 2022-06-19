mod levelstring;
mod object;
mod config;
mod speed;
use object::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
// https://github.com/Spu7Nix/SPWN-language/blob/master/levelstring/src/lib.rs
use levelstring::*;

use crate::config::Config;
mod physics;
// TODO
// Clean up main - done
// Account for speed portals - almost done
// Config loading - done
// Auto-backup - done
// Rotation center rework - done
// Properly remove old triggers - done
// Trigger grouping - done
// Add CLI stuff - done
// Add help message
// Add documentation
// Add more random parameters - done
fn main() {    
    let args: Vec<String> = std::env::args().collect();    
    if args.len() == 1 {
        println!("Todo: help message"); 
        return
    }
    let config = Config::new(args.get(1).unwrap().clone()).unwrap();
    if args.len() == 3 && args[2] == "restore"{
        let result = std::fs::copy(config.backup_path.clone(), config.path.clone());
        if result.is_ok() {
            println!("Backup restored");
        }
        else {
            println!("Failed to restore backup: {0}", result.unwrap_err())
        }
        return
    }
    let mut save = File::open(config.path.clone()).unwrap();
    let mut data = Vec::new();
    let level_name = config.level_name;
    save.read_to_end(&mut data).unwrap();
    let level = get_level_string(data, Some(&level_name));
    
    if let Ok(old_ls) = level {       
        let mut object_strings: Vec<String> = old_ls.split(';').map(|s|s.to_string()).collect();
        let header = object_strings[0].clone();
        // Remove header
        object_strings.remove(0);
        // Remove trailing semi-colon
        object_strings.pop();
        let mut objects: Vec<Obj> = object_strings.iter().map(|s|Obj::from(s.clone())).collect();    
        let tracker = speed::SpeedTracker::new(&objects);
        for layer in config.layers {
            physics::simulate(&mut objects, layer, &tracker)
        }
        println!("Writing {0} objects", objects.len());
        
        let mut ls = header + ";";        

        for obj in objects.iter() {         
            let obj_str = obj.as_str() + ";";            
            ls.push_str(&obj_str);
        }
        let path = PathBuf::from(config.path.clone());
        let write = true;
        let overwrite = true;
        if write {
            let result = std::fs::copy(config.path.clone(), config.backup_path.clone());
            if result.is_ok() {
                println!("Created backup at {0}", config.backup_path);
                encrypt_level_string(
                    ls,
                    if overwrite {
                        String::new()
                    }
                    else {
                        old_ls
                    }, 
                    path, 
                    Some(level_name)
                ).unwrap();
            }
            else {
                println!("Failed to make backup: {0}", result.unwrap_err())
            }
        }
    }
    else {
        println!("{:?}", level.unwrap_err());
    }
}