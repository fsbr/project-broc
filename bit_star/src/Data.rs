use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use minimal_lexical::Float;

// Keep nodes of the motion tree, and states separate
// Like a node in the motion tree should be a state plus some added info like fHat, gHat etc
// I am not sure if I actually want to derive copy or clone for this one
#[derive(Copy, Clone, Debug)]
pub struct State {
    pub id: u64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for State{
    fn eq(&self, other: &State) -> bool {
        self.id == other.id &&
        self.x == other.x &&
        self.y == other.y
    }

}
impl Eq for State{

}

// this is running but I'm not actually sure what it's doing
// What I HOPE is happening is that the thing is hashed by the id number lmfao
impl Hash for State{
    fn hash<H: Hasher>(&self, item: &mut H) {
        self.id.hash(item);
    }
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
#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Node {
    pub id: u64,
    pub state: State,
    // gT, fHat, etc.
    parent: Box<Node>,
}

#[derive(Debug, Hash)]
pub struct Edge{
    pub id: u64,
    pub source_node: Node,
    pub target_node: Node,
}
impl PartialEq for Edge{ 
    fn eq(&self, other: &Edge) -> bool {
        self.source_node.state.x == other.source_node.state.x &&
        self.source_node.state.y == other.source_node.state.y &&
        self.target_node.state.x == other.target_node.state.x &&
        self.target_node.state.y == other.target_node.state.y
    }
}
impl Eq for Edge{

}

#[derive(Debug)]
pub struct Environment {
    pub x_max: i32,
    pub y_max: i32,
    pub start: State,
    pub goal: State, 
    pub obs_map: Vec<Vec<u8>>,
}


// Stuff for the Priority Queue
// Goals for organization: have some generic mathematical utilities crate stored away for future use
pub fn float_to_triplet(input: f64)->( u8, i32, u64 ){
    // this takes an f64 and outputs the sign, exponent, mantissa triplet 
    // needed for comparison in edge queues
    let sign: u8 = input.is_sign_positive() as u8;
    let exponent: i32 = input.exponent();
    let mantissa: u64 = input.mantissa();
    let output: (u8, i32, u64) = (sign, exponent, mantissa);
    return output;
}
pub fn triplet_to_float(input: (u8,i32,u64) )-> f64{
    println!("This function is not implemented yet");
    let output: f64 = 3.141592;
    return output;

}
