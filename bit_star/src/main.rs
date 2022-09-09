//          BIT* in Rust
//          Thomas Fuller 2022

// imports for algorithms
use std::collections::HashMap;
use priority_queue::{PriorityQueue, DoublePriorityQueue};
use minimal_lexical::Float;
use std::process::exit;

// imports for outputting results
use std::fs::File;
use std::io::prelude::*;
use json;

mod Prune;
mod ExpandVertex;
mod Sample;
mod Data;
mod readWorld;
mod Collision;

// TODO: combine BestQueueValueQe and BestQueueValueQv into one function
fn BestQueueValueQv(queue: &DoublePriorityQueue<&Data::Node, (u8, i32, u64)>) -> (u8,i32,u64) {
    let mut min_couple = queue.peek_min();
    
    println!(" IN BEST QUEUE VALUE Qv {:#?}", min_couple);
    match min_couple {
        Some(min_couple) =>{
            println!("something was there!");
            println!("{:#?}", min_couple.1);
            return *min_couple.1;
        }
        None => {
            println!("Nothing found in Queue, so returning maximum values");
            return (1, i32::MAX, u64::MAX);
        }
    }
}

fn BestQueueValueQe(queue: &DoublePriorityQueue<&Data::Edge, (u8, i32, u64)>) -> (u8,i32,u64) {
    let mut min_couple = queue.peek_min();
    
    println!(" IN BEST QUEUE VALUE Qe, {:#?}", min_couple);
    match min_couple {
        Some(min_couple) =>{
            println!("something was there!");
            println!("{:#?}", min_couple.1);
            return *min_couple.1;
        }
        None => {
            println!("Nothing found in Queue, so returning maximum values");
            return (1, i32::MAX, u64::MAX);
        }
    }
}
// i really want to use Some() here
fn main() -> std::io::Result<()>{
    // File writing
    let mut sample_file = File::create("samples.txt")?;

    // Reading in the environment
    let Environment = readWorld::readWorld();

    // Algorithm Setup
    let inf: f64 = f64::INFINITY;

    // Book-keeping for the number of samples, nodes and edges we create
    let mut num_samples:u64 = 0;
    let mut num_nodes:u64 = 0;
    let mut num_edges: u64 = 0;

    let mut x_samples =  Data::Samples{
        num_samples: num_samples,
        samples: HashMap::new(),
    };
    num_samples+=1;



    println!("BIT* Implementation");

    // A1.1 V <-{x_start}
    let x_start = Environment.start;
    let x_goal = Environment.goal;
        
    let mut V: HashMap<u64, Data::Node> = HashMap::new();
    let mut start_node = Data::Node{
        id: 0,
        state: x_start,
        gT: 0.0,
        hHat: Collision::euclidean_distance(x_start.x, x_start.y, x_goal.x, x_goal.y),
        gHat: Collision::euclidean_distance(x_start.x, x_start.y, x_start.x, x_start.y),
        parent: None,
    };
    num_nodes+=1;

    // Need a function in here that computes the gHat, fHat, all that stuff (tomorrow)
    V.insert(start_node.id, start_node);
    println!("V = {:#?}", V); 

    // A1.1 E <- emptyset
    let mut E: HashMap<u64, Data::Edge> = HashMap::new();

    // A1.1 x_samples <- x_goal
    x_samples.num_samples+=1;
    x_samples.samples.insert(x_goal.id, x_goal);
    println!("x_samples = {:#?}", x_samples); 

    // Qe <- emptyset
    let mut Qe: DoublePriorityQueue<&Data::Edge, (u8, i32, u64)> = DoublePriorityQueue::new();

    // what i think is happening is im using a reference to the nodes in V, and ``borrowing" them
    let mut Qv: DoublePriorityQueue<&Data::Node, (u8, i32, u64)> = DoublePriorityQueue::new();

    let mut loop_counter: u8 = 0; 
    // A1.3
    while loop_counter < 1 {
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
            for (k,v) in &V {
                println!("Line 1.8");
                let Qv_score = Data::float_to_triplet( V[&k].gT + V[&k].hHat); 
                Qv.push(&V[&k], Qv_score);
            }  
            println!("Qv = {:#?}", Qv);
            // A1.9 we will do k nearest instead of radius checking
        }// Q_len == 0 and Qv_len == 0 

        println!("BEST GUEUUE VALUE");
        let z = BestQueueValueQv(&Qv);
        println!("best queue value qv = {:#?}", z);
        // A1.10 while bestQueueValue(Qv) <= BestQueueValue(Qe) do ExpandVertex(BestInQueue(Qv))
        while BestQueueValueQv(&Qv) <= BestQueueValueQe(&Qe){
            println!("should be expanding vertices here"); 
            // I don't think I want to pass in the entire queue, just the best vertex from the queue
            ExpandVertex::ExpandVertex(&mut Qv, &mut x_samples);
            std::process::exit(1);
        }
        
    }

    // Checking the euclidean function for fun
    let d  = Collision::euclidean_distance(1.0, 0.0, 0.0, 1.0);
    println!("d == {}", d);

    // File writing stuff
    for (k,v) in &x_samples.samples{
        write!(sample_file, "{}, {},{}, \n", v.id, v.x, v.y);
    }

    Ok(())
} // END MAIN
