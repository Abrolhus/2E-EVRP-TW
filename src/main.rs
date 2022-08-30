#![feature(map_first_last)]
mod instance;
mod ev_route;
mod satelite;
mod solution;
mod truck;
mod aux_structures;
mod construtivo;
mod route;
mod plot;

use std::{env, process};
use crate::instance::Instance;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() == 0 {
        println!("usage: main <instance_path>");
        println!("You can also: main <instance1_path> <instance2_path> <instace3_path>...");
        process::exit(1);
    }
    for (i,arg) in arguments.iter().enumerate(){
        if i == 0{
            continue;
        }
        let filename = arg.as_str();
        let inst = Instance::instance_from_file(filename);
        println!("{:#?}", inst);
        match construtivo::construtivo_rota_unica(&inst, format!("plot{}", i).as_str()){
            Ok(_solution) => println!("FOUND SOLUTION!"),
            Err(erro) => println!("{}", erro),
        }
    }
}
