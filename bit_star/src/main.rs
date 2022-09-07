use std::collections::HashMap;
use priority_queue::PriorityQueue;
use minimal_lexical::Float;
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

            // 

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
    

    // I'm going to try a lexicographical ordering fo the sign, mantissa and exponent
    // SIGN EXPONENT MANTISSA for 
    //let test_number: f64 = 31.5;
    //let mantissa: u64 = test_number.mantissa();
    //let exponent: i32 = test_number.exponent();
    //let is_sign_positive = test_number.is_sign_positive() as u8;
    //println!("mantissa {}", mantissa );
    //println!("exponent {}", exponent );
    //println!("is sign positive {}", is_sign_positive);
    //println!("exponent {}", 3.0 < 4.12);
    ////let triplet = Data::Triplet{
    ////    sign: is_sign_positive,
    ////    exponent: exponent,
    ////    mantissa: mantissa,
    ////};
    //let triplet = (is_sign_positive, exponent, mantissa);
    //let output = Data::float_to_triplet(31.49999);
    //println!("triplet {:#?}", triplet);
    //let t1 = vec!([8, 10, 10]);
    //let t2 = vec!([8, 10, 12]);
    ////println!("t1 > t2? {}", t1>t2);
    //println!("Data::float_to_triplet = {:#?}", output);
    //println!("f2t[0] {}", output.0);
    //println!("Triplet > Output? = {}", triplet> output);
    

}
