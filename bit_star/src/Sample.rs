// This function is taking care of sampling
use rand::{thread_rng, Rng};
use crate::Data;

pub fn Sample(batch_size: u64 , gt_x_goal: f64, mut x_samples: Data::Samples)->Data::Samples
{ 

    let mut start_id = x_samples.num_samples+1; 
    let mut finish_id = start_id + batch_size;


    // this is a very basic sampling function
    for i in start_id..finish_id{
        let mut rng = thread_rng();
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();

        let mut state = Data::State {
            id: i,
            x: x,
            y: y,
        };
        x_samples.samples.insert(state.id, state);
        
    }
    x_samples.num_samples = finish_id;
    return x_samples;
}

