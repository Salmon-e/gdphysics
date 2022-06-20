# gdphysics
A tool for simulating objects in Geometry Dash levels. 

# Usage

The program requires only a JSON file as input where all other parameters are specified. An example configuration file is
```JSON
{
    "level_name": "Example",
    "path" : "C:/path/to/CCLocalLevels.dat", 
    "backup_path" : "C:/path/to/CCLocalLevels Backup.dat",
    "simulations" : [
        {
            "layer" : 1,
            "gravity" : [0.0, -100.0],
            "objects" : [
                {
                    "group" : 5,
                    "initial_velocity" : [200.0, 300.0]
                }
            ]
        }
    ]
}
```
First, there are the top level parameters for the entire level. These are the level name, the path to `CCLocalLevels.dat`, and the path to store the backup that is generated.
Next there is a list of simulations to run. Each simulation operates on one editor layer of the level, all triggers generated will be placed there and any other objects
not required by the program may cause undefined behaviour if placed there.

Objects to be simulated are made by placing at least 3 objects of any kind on the target layer in the same group. The convex hull of the positions of these objects will
be used for the hitbox of the simulated object. Because only the object's position matters, it's best to use a small, centered object for forming the shape.
(I like to use the X object near the end of the chain tab) Once you've made the shape, give all the points a group, (and **only** one group, any others will make things break)
and place one final point inside the object and give it a second group to be the rotation center. Don't worry about centering it perfectly, that will be done automatically, just
make sure it's inside the shape. 


![image](https://user-images.githubusercontent.com/77418890/174506242-ff798aa1-8213-4c9a-9d16-19cfa33e6590.png)

For example, here is a square with corners all with group 5, and center point with group 5 and **then** 6, the order is important. Going back to the example JSON file,
```JSON
"objects" : [
    {
       "group" : 5,
       "initial_velocity" : [200.0, 300.0]
    }
]
```
Will add an intial velocity of 200 units per second to the right and 300 units per second up to our square, since it was put on group 5.
To run this simulation, run
`gdphysics example_config.json`
and the movements will be added.

Subsequent calls will automatically remove previous triggers.
If something goes wrong and you wish to revert to before, run
`gdphysics example_config.json restore`
and the backup will be restored.
# Parameters

Top level parameters
-------

| Parameter     | Description                     | Default |
|---------------|---------------------------------|---------|
| `level_name`  | The name of the target level    | none    |
| `path`        | The path to `CCLocalLevels.dat` | none    |
| `backup_path` | The path to store the backup    | none    |
| `simulations` | The list of simulations to run  | none    |

Simulation level parameters
-------
| Parameter                    | Description                                          | Type           | Default |
|------------------------------|------------------------------------------------------|----------------|---------|
| `layer`                      | The target layer of the  simulation                  | integer        | none    |
| `height`                     | The height triggers are placed                       | float          | 2100    |
| `fps`                        | How many frames per second the simulation runs       | float          | 60      |
| `keyframe_interval`          | How many frames between each keyframe                | integer        | 6       |
| `gravity`                    | The gravity vector                                   | [float, float] | [0, -294.3]  |
| `rotation_duration_modifier`* | The multiplier for the duration of rotation triggers | float          | 0.9     |
| `sim_time`                   | Length of simulation in seconds                      | float          | 5.0     |
| `anchor_id`**                  | Object ID of the anchor                              | integer        | 41      |
| `ground`                     | Whether ground is there                              | bool           | true    |
| `objects`                    | List of per object parameters                        | list           | empty   |

Object level parameters
---------
| Parameter          | Description                                | Type           | Default |
|--------------------|--------------------------------------------|----------------|---------|
| `group`            | The main group of the object               | integer        | none    |
| `velocity`         | The initial velocity of the object         | [float, float] | [0, 0]  |
| `angular_velocity` | The initial angular velocity of the object | float          | 0       |
| `density`          | The density of the object                  | float          | 1       |
| `dynamic`          | Whether the object is dynamic              | bool           | true    |
| `restitution`      | The coefficient of restitution             | float          | 0       |
| `linear_damping`   | The damping of linear motion               | float          | 0.1     |
| `angular_damping`  | The damping of rotation                    | float          | 0.1     |
| `friction`         | The friction coefficient                   | float          | 1.0     |
| `positon_fixed`    | Whether the object's position is fixed     | bool           | false   |
| `rotation_fixed`   | Whether the object's rotation is fixed     | bool           | false   |

*rotation triggers do not overlap, so the rotation duration is slightly decreased to account for little overlaps. If rotation breaks, try decreasing this value, and if it is too choppy, try increasing it a little. 

**If you want to change when the simulation starts, place an anchor object on the target layer at the desired position. By default the first chain in the chain tab will be viewed as an anchor, but you can change it to any object by setting this parameter.