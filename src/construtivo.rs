use std::collections::BTreeSet;
use crate::instance::Instance;
use crate::plot::plot_solution;
use crate::solution::{Solution, check_solution, solution_to_file};
use crate::instance::Node;
use crate::ev_route::EvRoute;
use crate::truck::TruckRoute;
use crate::aux_structures::{Insertion, SingleRouteInsertion};

pub fn get_first_echelon_hint(solution:&mut Solution, instance: &Instance){
    if !solution.trucks.is_empty(){
        panic!("Solution truck routes should be empty");
    }
    // let mut truck_routes: Vec<TruckRoute> = Vec::new();
    // add empty (0>0) truck route for each truck vehicle avaliable
    for _ in 0..instance.get_n_trucks(){
        solution.trucks.push(TruckRoute::new(instance, instance.get_vehicle(0)));
    }
    let satelites = instance.get_sats();
    let depot = instance.get_depot();
    for (sat_index, sat_node) in satelites.iter().enumerate(){
        for (truck_id, truck) in solution.trucks.iter_mut().enumerate(){
            let can_insert = truck.can_insert_node(instance, sat_node.node_id, truck.route.len()-1, truck_id, sat_index);
            match can_insert{ 
                Some(insertion) => {
                    truck.route.insert(instance, &insertion, true);
                    solution.satelites[insertion.origin_id].set_tempo_de_chegada(insertion.t_chegada); //TODO: wrap this in solution.insert() function
                    println!("inseriu, {:#?}", insertion);
                    break;
                }
                None => ()
            }
        }
    }
}

pub fn construtivo_sem_recarga(instance: &Instance, filename: &str) -> Result<Solution, String> {
    let mut solution = Solution::new(instance);
    println!("solution: {:#?}", solution);
    get_first_echelon_hint(&mut solution, instance);
    println!("solution: {:#?}", solution);
    let mut lista_restrita: BTreeSet<Insertion> = BTreeSet::new();
    let mut unvisited_clients: Vec<usize> = Vec::new();

    for sat in &mut solution.satelites{
        sat.add_vehicle(instance.get_vehicle(1), instance); //TODO: pensar numa maneira de nao
                                                            //adicionar o veiculo 1 sempre kk
    }
    println!("solution: {:#?}", solution);

    for client in instance.get_clients(){
        unvisited_clients.push(client.node_id);
    }
    while !unvisited_clients.is_empty() {
        println!("{:#?}", unvisited_clients);
        let mut n_evs_in_solution = 0;
        for client_id in &unvisited_clients {
            // para cada satelite,
            for (sat_id, satelite) in solution.get_satelites().iter().enumerate() {
                // para cada rota de EV desse satelite,
                for (ev_id, ev_route) in satelite.get_ev_routes().iter().enumerate() {
                    n_evs_in_solution += 1;
                    // para cada posicao nessa rota
                    for (position, node) in ev_route.get_nodes().iter().enumerate() {
                        // veh se consegui inserir cliente na posicao
                        if position != 0 && position < ev_route.len() {
                            print!(".");
                            match ev_route.can_insert_node(instance, *client_id, position, ev_id, sat_id){
                                Some(insertion) => { lista_restrita.insert(insertion); },
                                None => ()
                            }
                        }
                    }
                }
            }
        }
        println!("{:#?}", lista_restrita);
        if lista_restrita.is_empty(){
            println!("{:#?}", solution);
            check_solution(&solution, instance);
            match solution_to_file(&solution, "solution.txt"){
                Ok(()) => (),
                _ => ()
            }
            plot_solution(instance, &solution, filename).unwrap();
            return Err(String::from("Lista restrita vazia"));
        }
        let insertion = lista_restrita.pop_first().unwrap();
        println!("{:#?}", lista_restrita);
        println!("{:#?}", solution);
        n_evs_in_solution /= unvisited_clients.len();
        n_evs_in_solution -= solution.satelites.len();
        println!("{:?}", unvisited_clients);
        println!("{:?}", n_evs_in_solution);
        let add_vehicle = n_evs_in_solution < instance.get_n_evs();
        solution.insert(instance, &insertion, add_vehicle);
        let index = unvisited_clients.binary_search(&insertion.node_id).unwrap();
        unvisited_clients.remove(index);
        lista_restrita.clear();
    }
    println!("foi");
    println!("{:#?}", solution);
    Ok(solution)
}

pub fn construtivo_rota_unica(instance: &Instance, filename: &str) -> Result<Solution, String> {
    let mut solution = Solution::new(instance);
    get_first_echelon_hint(&mut solution, instance);
    println!("solution soh com primeiro nivel: {:#?}", solution);
    let mut lista_restrita: BTreeSet<SingleRouteInsertion> = BTreeSet::new();
    let mut unvisited_clients: Vec<usize> = Vec::new();

    println!("solution com satelites vazios: {:#?}", solution);

    for client in instance.get_clients(){
        unvisited_clients.push(client.node_id);
    }
    while !unvisited_clients.is_empty() {
        println!("unvisited clients: {:#?}", unvisited_clients);
        let mut n_evs_in_solution = 0;
        for client_id in &unvisited_clients {
            // para cada satelite,
            for (sat_id, satelite) in solution.get_satelites().iter().enumerate() {
                // para cada rota de EV desse satelite,
                match satelite.can_insert_in_single_route(instance, *client_id){
                    Some(insertion) => { lista_restrita.insert(insertion); },
                    None => ()
                }
            }
        }
        println!("{:#?}", lista_restrita);
        if lista_restrita.is_empty(){
            println!("{:#?}", solution);
            check_solution(&solution, instance);
            match solution_to_file(&solution, "solution.txt"){
                Ok(()) => (),
                _ => ()
            }
            plot_solution(instance, &solution, filename).unwrap();
            return Err(String::from("Lista restrita vazia"));
        }
        let insertion = lista_restrita.pop_first().unwrap();
        println!("{:#?}", lista_restrita);
        println!("{:#?}", solution);
        println!("{:?}", unvisited_clients);
        solution.insert_single_route(instance, &insertion);
        let index = unvisited_clients.binary_search(&insertion.node_id).unwrap();
        unvisited_clients.remove(index);
        lista_restrita.clear();
    }
            plot_solution(instance, &solution, filename).unwrap();
    println!("foi");
    println!("{:#?}", solution);
    Ok(solution)
}
