mod instance;
mod ev_route;
// mod solution;

use crate::instance::Instance;

fn main() {
    println!("Hello, world!");
    let filename = "./instances/Customer_5/C101_C5x.txt";
    let inst = Instance::instance_from_file(filename);
    println!("{:#?}", inst);
    // stores each 
    // println!("With text:\n{}", contents);
}
