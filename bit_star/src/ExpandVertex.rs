use priority_queue::{DoublePriorityQueue};
use crate::Data;
use crate::Collision;

// Implementing the Expand Vertex function as in the BIT* paper
pub fn ExpandVertex( Qv: &mut DoublePriorityQueue<&Data::Node, (u8, i32, u64)>,
                    x_samples: &mut Data::Samples ) ->() {
    println!("HELLO WORLD!");
    // A2.1 Qv <- {v}
    let mut best_vertex_queue_pair = Qv.pop_min();
    println!("best_vertex in queue = {:#?}", best_vertex_queue_pair);
    println!("End");
    // A2.2 X_near <-Nearest Neighbors 
    // need some Some() None() stuff
    match best_vertex_queue_pair {
        Some(best_vertex_queue_pair) => {
            //let x_near = nearest_neighbors(10, x_samples, best_vertex);
            let best_vertex = best_vertex_queue_pair.0;
            println!("expanded from Some() wrapper {:#?}", best_vertex);
            println!("End");
            let nn = nearest_neighbors(10, x_samples, best_vertex);
            for neighbor in nn {
                println!(" wont u b my neighbor {:#?}", neighbor);
            }





        }
        None => {
            println!("No vertex found in Expand Vertex");
        }
    }
    
    // A2.3 Qe <-+ {(v,x) in v x X_near | ghat(v) + cHat(v,x) + hHat(x) < gT(xGoal)
}

fn nearest_neighbors(k_near: i32, x_samples: &mut Data::Samples, best_vertex : &Data::Node)->Vec<Data::State>{
    // What I want this to do
    // Should only find the nearest ten states
    // 1. Iterate through every sample in X_samples
    // 2. Return a vector Data::Nodes of the k-nearest with appropriate gHat, hHat etc 
   
    let mut distance_queue: DoublePriorityQueue<&Data::State, (u8, i32, u64)> = DoublePriorityQueue::new();
    let mut nearest_neighbor_list: Vec<Data::State> = Vec::new();
    // From now on in this program, I think I have to tell my computer that this is a reference
    for (key,v) in &x_samples.samples {
        // iterate through every sample 
        // place them into a priority queue with the priority of the distance 
        println!("k = {}, v = {:#?}", key,v);
        let d = Collision::euclidean_distance(v.x, v.y, best_vertex.state.x, best_vertex.state.y);
        println!("d = {}", d);
        distance_queue.push(v, Data::float_to_triplet(d));
        // 
    }
    println!("{:#?}", distance_queue);
    for i in 0..k_near{
        println!("popped distances {:#?}", distance_queue.pop_min());
        let nn = distance_queue.pop_min();
        match nn {
            Some(nn)=> {
                // what we actually want to do with the nearest neighbors  
                println!("Doing Something With The nearest Neighbors");
                // dereference?
                nearest_neighbor_list.push(*nn.0);
            }
            None=>{
                println!("Nothing found in nearest_neighbors");
            }
        } 
    } // for i in 0..k_near

    // im terrified because stuff is just working 
    // but i kind of wonder if i've transfered ownership somehow and taken these neighbors out of scope
    // or out of my program for lack of a better term
    println!("nearest_neighbor_list = {:#?}", nearest_neighbor_list); 
    println!("nearest_neighbor_list length = {:#?}", nearest_neighbor_list.len()); 
    //
    return nearest_neighbor_list;
}
