use std::collections::HashMap;

// Keep nodes of the motion tree, and states separate
// Like a node in the motion tree should be a state plus some added info like fHat, gHat etc
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

// This is the thing that we will actually push into the V, E 
#[derive(Debug)]
pub struct Node {
    pub id: u64,
    pub state: State,
    parent: Box<Node>,
}

#[derive(Debug)]
pub struct Edge{
    pub id: u64,
    pub source_state: State,
    pub target_state: State,
}
impl PartialEq for Edge{ 
    fn eq(&self, other: &Edge) -> bool {
        self.source_state.x == other.source_state.x &&
        self.source_state.y == other.source_state.y &&
        self.target_state.x == other.target_state.x &&
        self.target_state.y == other.target_state.y
    }
}
impl Eq for Edge{

}

#[derive(Debug)]
pub struct Triplet{
    pub sign: u8,
    pub exponent: i32, 
    pub mantissa: u64,
}


#[derive(Debug)]
pub struct Environment {
    pub x_max: i32,
    pub y_max: i32,
    pub start: State,
    pub goal: State, 
    pub obs_map: Vec<Vec<u8>>,
}



