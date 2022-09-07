use std::collections::HashMap;
use priority_queue::PriorityQueue;
use minimal_lexical::Float;
mod Prune;
mod Sample;
mod Data;
mod readWorld;

fn main() {
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
        
    let mut V: HashMap<u64, Data::State> = HashMap::new();
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
            // READ this value from a configuration file someday
            x_samples = Sample::Sample(50, 420.0, x_samples);
            println!("{:#?}", x_samples);
            println!("{}", x_samples.samples.len());
            

        }// Q_len == 0 and Qv_len == 0 

        
    }

    // EXAMPLE USAGE
    //let mut source_state = Data::State{
    //    id: 2,
    //    x: 6.9,
    //    y: 42.0,
    //};
    //let mut target_state = Data::State{
    //    id: 3,
    //    x: 1006.9,
    //    y: 10042.0,
    //};
    //let mut EdgeToAdd =  Data::Edge{
    //    id: 1,
    //    source_state: source_state,
    //    target_state: target_state,
    //};
    //let score: f64 = 69.69;
    //
    //Qe.push(EdgeToAdd, Data::float_to_triplet(score));
    //println!("Qe = {:#?}", Qe);

}
