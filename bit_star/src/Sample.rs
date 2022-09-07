// This function is taking care of sampling
use rand::{thread_rng, Rng};
use crate::Data;
use crate::Collision;

pub fn Sample(batch_size: u64 , gt_x_goal: f64, mut x_samples: Data::Samples, environment: &Data::Environment)->Data::Samples
{ 

    let start_id = x_samples.num_samples+1; 
    let finish_id = start_id + batch_size;
    let x_max = environment.x_max;
    let y_max = environment.y_max;
    let obs_map  = &environment.obs_map;
    let mut valid_sample_counter = 0;


    while valid_sample_counter < batch_size {
        let mut rng = thread_rng();
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let test_x = x*(x_max as f64);
        let test_y = y*(y_max as f64);
        let test_indices = Collision::obs_map_idx_from_coords(test_x, test_y, y_max);
        let in_collision = Collision::check_idx_in_collision(test_indices.0, test_indices.1, obs_map);
        if  !in_collision  {
            // a valid sample is found
            let mut state = Data::State{
                id: valid_sample_counter+start_id,
                x: test_x,
                y: test_y,
            };

            x_samples.samples.insert(state.id, state);
            valid_sample_counter+=1;
        } // if !in_collision
        x_samples.num_samples = finish_id;
    }
    return x_samples;
}

