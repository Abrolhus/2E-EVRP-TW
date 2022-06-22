#![feature(map_first_last)]
mod instance;
mod ev_route;
mod satelite;
mod solution;
mod truck;
mod auxStructures;
mod construtivo;
mod route;

use crate::instance::Instance;

fn main() {
    println!("Hello, world!");
    let filename = "./instances/Customer_5/C101_C5x.txt";
    let inst = Instance::instance_from_file(filename);
    println!("{:#?}", inst);
    construtivo::construtivo_sem_recarga(&inst);
    // stores each 
    // println!("With text:\n{}", contents);
}
