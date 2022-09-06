// This function is taking care of sampling
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use crate::Data;

pub fn Sample( batch_size: u64 , mut x_samples: Data::Samples)->Data::Samples
{ 
      
    for i in 1..batch_size{
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
    return x_samples;
}

