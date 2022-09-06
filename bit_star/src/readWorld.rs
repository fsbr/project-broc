// The idea of this file is to read in grid world environments that I've made, and return the relevant data structures
use std::env;
use std::fs;
use crate::Data;

pub fn readWorld()->Data::Environment {
    // readWorld() should function as some kind of a main function
    // You provide it the environment files, and then it outputs the relevant data structures
    // all the helper functions can be kept in this module

    println!("Reading the World File");
    let args: Vec<String> = env::args().collect();
    let root_dir = "../../test_environments/grid_envs/environment".to_string();
    let instance_number = &args[1].to_string();
    let extension = ".txt".to_string();
    let environment_file = root_dir+instance_number+&extension;
   
    println!("{:?}", environment_file); 
    let contents = fs::read_to_string(environment_file).expect("Should have been able to read the file");
    let mut obstacle_map: Vec< Vec<u8> > = Vec::new();

    // eventually I want to be able to do different types of files but for now lets just get something working
    println!("Printing CONTENTS");
    println!("{}",contents);
    let content_vec: Vec<&str> = contents.lines().collect();


    
    // We are going to do this a better way later
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;

    let mut x_start: f64 = 0.0;
    let mut y_start: f64 = 0.0;
    let mut x_goal: f64 = 0.0;
    let mut y_goal: f64 = 0.0;

    for (idx, val) in content_vec.iter().enumerate(){
        if idx == 0 {
            x_max = content_vec[idx].parse().unwrap();
        } else if idx == 1 {
            y_max = content_vec[idx].parse().unwrap();
            println!(" 2+ymax == {:#?}", 2+(y_max as usize));
        } else if idx > 1 && idx < 2+(y_max as usize){
            //println!("{:#?}", content_vec[idx]);

            let mut obs_row:Vec<u8> = Vec::new();

            for letter in content_vec[idx].chars(){
                //println!(" THE CHARACTER IM TESTING IS {}", letter);
                match letter {
                    '-' => obs_row.push(0),
                    '#' => obs_row.push(1),
                    _ => println!("Invalid Character found in input data"),
                }
            } // for letter in content_vec
            obstacle_map.push(obs_row);
        } else if idx == 2+y_max as usize {
            x_start = content_vec[idx].parse().unwrap();
        } else if idx == 3+y_max as usize {
            y_start = content_vec[idx].parse().unwrap();
        } else if idx == 4+y_max as usize{
            x_goal = content_vec[idx].parse().unwrap();
        } else if idx == 5+y_max as usize{
            y_goal = content_vec[idx].parse().unwrap();
        } else {
            // theres probably something better i can do here 
        }
    } // content_vec iteration

    println!("Summary of the input");
    println!("{:#?}", content_vec);
    println!("x_start = {}", x_start);
    println!("y_start = {}", y_start);
    println!("x_goal = {}", x_goal);
    println!("y_goal = {}", y_goal);
    
    println!("{:#?}", obstacle_map);

    // Intention is to assemble the start and goal states
    let start_state = Data::State{
        id:0, 
        x: x_start,
        y: y_start,
    };
    
    let goal_state = Data::State{
        id: 1,
        x: x_goal,
        y: y_goal,
    };
    // Place Everything into the Environment Structure
    // We definitely never want this thing to change
    let Environment = Data::Environment{
        x_max: x_max,
        y_max: y_max,
        start: start_state,
        goal: goal_state,
        obs_map: obstacle_map,
    };

    return Environment;
} //readWorld

