use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct State {
    pub id: u64,
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Samples {
    pub num_samples: u64,
    pub samples: HashMap<u64, State>,
}

impl Default for Samples{
    fn default() -> Samples {
        Samples {
            num_samples: 0,
            samples: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Environment {
    pub x_max: i32,
    pub y_max: i32,
    pub start: State,
    pub goal: State, 
    pub obs_map: Vec<Vec<u8>>,
    
}
