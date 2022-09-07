// This function is taking care of sampling
use rand::{thread_rng, Rng};
use crate::Data;
use crate::Collision;

pub fn Sample(batch_size: u64 , gt_x_goal: f64, mut x_samples: Data::Samples, environment: &Data::Environment)->Data::Samples
{ 

    let mut start_id = x_samples.num_samples+1; 
    let mut finish_id = start_id + batch_size;
    let x_max = environment.x_max;
    let y_max = environment.y_max;
    let obs_map  = &environment.obs_map;

    println!("OBS MAP IN SAMPLES{:#?}", obs_map);

    // this is a very basic sampling function
    // need to get 50 VALID samples
    for i in start_id..finish_id{
        let mut rng = thread_rng();
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let test_x = x*(x_max as f64); 
        let test_y = y*(y_max as f64);

        println!("test_x = {}, test_y = {}", test_x, test_y);
        let output = Collision::obs_map_idx_from_coords(test_x,test_y, y_max);
        let in_collision = Collision::check_idx_in_collision(output.0, output.1, obs_map);
        println!("in collision = {:#?}", in_collision);
        println!("output (indices) = {:#?}", output);
        let mut state = Data::State {
            id: i,
            x: test_x,
            y: test_y,
        };
        x_samples.samples.insert(state.id, state);
        
    }
    x_samples.num_samples = finish_id;
    return x_samples;
}

