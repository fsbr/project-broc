// Routines for Checking Collisions in 2D (and hopefully more advanced) motion planning problems.
// 

pub fn obs_map_idx_from_coords(x: f64, y: f64, y_max: i32) -> (u64, u64) {
    // This function should take in the x,y coordinates of a given state
    // and output where in the obstacle map it would be

    let x_idx = x.floor() as u64;
    let y_idx = y_max as u64 - y.floor() as u64 - 1; 
    return (x_idx, y_idx);
}

pub fn check_idx_in_collision(x_idx: u64, y_idx: u64, obs_map: &Vec<Vec<u8>> ) -> bool {
    // returns true if the index is in collision
    let value = obs_map[y_idx as usize][x_idx as usize];
    let mut returnval: bool = false;
    if value == 0 {
        returnval = false;
    } else if value == 1 {
        returnval = true; 
    }
    return returnval;
}

pub fn euclidean_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // Not yet implemented
    // i wish i started using this programming language back then, 
    // but idk if i had the skills/courage 
    return x1;
}
