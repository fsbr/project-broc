use std::collections::HashMap;
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
    for _i in 0..99{
        x_samples = Sample::Sample(50, x_samples);
    }

    

}
