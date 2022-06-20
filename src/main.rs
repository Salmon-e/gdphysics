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
// Add help message - done
// Add documentation - done
// Add more random parameters - done
fn main() {    
    let args: Vec<String> = std::env::args().collect();    
    if args.len() == 1 {
        println!("Usage instructions can be found at https://github.com/Salmon-e/gdphysics"); 
        return
    }
    let config_path = args.get(1).unwrap();
    let result = Config::new(config_path.clone());
    let config = if let Ok(config) = result {
        config
    }
    else {
        println!("Failed to load config file: {0}", result.unwrap_err());
        return
    };
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
    let result = File::open(config.path.clone());
    let mut save = if let Ok(save) = result {
        save
    }
    else {
        println!("Failed to load save file: {0}", result.unwrap_err());
        return
    };
    let mut data = Vec::new();
    let level_name = config.level_name;
    if let Err(e) = save.read_to_end(&mut data) {
        println!("Something went wrong while reading the save file: {0}", e)
    }
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
        for layer in config.simulations {
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
                if let Err(e) = encrypt_level_string(
                    ls,
                    if overwrite {
                        String::new()
                    }
                    else {
                        old_ls
                    }, 
                    path, 
                    Some(level_name)
                ) {
                    println!("Failed to write to save: {0}", e)
                }
            }
            else {
                println!("Failed to make backup: {0}", result.unwrap_err())
            }
        }
    }
    else {
        println!("The save file failed to decrypt: {0}", level.unwrap_err());
    }
}