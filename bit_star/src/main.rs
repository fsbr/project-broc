//          BIT* in Rust
//          Thomas Fuller 2022

// imports for algorithms
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use minimal_lexical::Float;

// imports for outputting results
use std::fs::File;
use std::io::prelude::*;
use json;

mod Prune;
mod Sample;
mod Data;
mod readWorld;
mod Collision;

fn main() -> std::io::Result<()>{
    // File writing
    let mut file = File::create("samples.txt")?;

    // Algorithm Setup
    let inf: f64 = f64::INFINITY;

    let mut num_samples:u64 = 0;
    let mut x_samples =  Data::Samples{
        num_samples: num_samples,
        samples: HashMap::new(),
    };

    let Environment = readWorld::readWorld();

    println!("BIT* Implementation");

    // A1.1 V <-{x_start}
    let x_start = Environment.start;
    let x_goal = Environment.goal;
        
    // TODO: Update this step to use NODE instead of state, the difference is important
    let mut V: HashMap<u64, Data::State> = HashMap::new();

    // Need a function in here that computes the gHat, fHat, all that stuff (tomorrow)
    V.insert(x_start.id, x_start);
    println!("V = {:#?}", V); 

    // A1.1 E <- emptyset
    let mut E: HashMap<u64, Data::Edge> = HashMap::new();

    // A1.1 x_samples <- x_goal
    x_samples.num_samples+=1;
    x_samples.samples.insert(x_goal.id, x_goal);
    println!("x_samples = {:#?}", x_samples); 

    // Qe <- emptyset
    let mut Qe: PriorityQueue<Data::Edge, (u8, i32, u64)> = PriorityQueue::new();
    let mut Qv: PriorityQueue<Data::Node, (u8, i32, u64)> = PriorityQueue::new();

    let mut loop_counter: u8 = 0; 
    // A1.3
    while loop_counter < 2 {
        println!("JUST SEE THIS ONCE"); 
        loop_counter+=1;
        
        // A1.4
        let mut Qe_len = Qe.len();
        let mut Qv_len = Qv.len();
        println!("Qe len = {}, Qv len = {}", Qe_len, Qv_len);
        if Qe_len == 0 && Qv_len == 0 {
            // A1.5, PRUNE
            Prune::Prune();

            // A1.6 x_samples <-+ Sample( m, gT(xgoal) )
            x_samples = Sample::Sample(50, inf, x_samples, &Environment);

            // A1.7 V_old <- V;
            let mut V_old = &V;
            println!("V_old {:#?}", V_old); 
            println!("V {:#?}", &V); 

            // A1.8 Qv <- V
            


        }// Q_len == 0 and Qv_len == 0 
        
    }

    // Checking the euclidean function for fun
    let d  = Collision::euclidean_distance(1.0, 0.0, 0.0, 1.0);
    println!("d == {}", d);

    // File writing stuff
    for (k,v) in &x_samples.samples{
        write!(file, "{}, {},{}, \n", v.id, v.x, v.y);
    }
    Ok(())
} // END MAIN
