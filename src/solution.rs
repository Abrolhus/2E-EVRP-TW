use std::collections::HashMap;

use crate::auxStructures::Insertion;
use crate::instance::{Instance, NodeType, Node};
use crate::satelite::Satelite;
use crate::truck::TruckRoute;

#[derive(Debug)]
pub struct Solution{
    pub satelites: Vec<Satelite>,
    pub trucks: Vec<TruckRoute>,
    cost: f64,
    n_trucks: u32,
    n_evs: u32,
    //truck_routes: Vec<TruckRute>,
}
impl Solution{
    pub fn new(instance:&Instance) -> Self {
        let mut sol = Solution { 
            satelites: Vec::new(), 
            trucks: Vec::new(),
            cost: 0.0,
            n_trucks: 0,
            n_evs: 0 
        };
        sol.add_empty_satelites(instance.get_sats());
        sol
    }
    pub fn get_satelites(&self) -> &Vec<Satelite> {
        &self.satelites
    }
    pub fn get_truck_routes(&self) -> &Vec<TruckRoute>{
        &self.trucks
    }
    fn add_empty_satelites(&mut self, satelites_info: &[Node]){
        if !self.satelites.is_empty(){
            panic!("Solution already with satelites")
        }
        for node in satelites_info {
        self.satelites.push(Satelite::new(node.node_id))
        }
    }
    pub fn insert(&mut self, instance: &Instance, insertion: &Insertion, add_vehicle: bool){
        let origin_index = insertion.origin_id;
        let origin_type = insertion.origin_type;
        let route_index = insertion.route_id;
        match origin_type{
            NodeType::Sat => {
                let satelite = &mut self.satelites[origin_index];
                let ev_route = &mut satelite.routes[route_index];
                ev_route.insert(instance, insertion);
                if add_vehicle {
                    satelite.add_vehicle(instance.get_vehicle(1), instance); // TODO: tira isso pelo
                }
            }
            NodeType::Depot => {
                let truck_route = &mut self.trucks[route_index];
                truck_route.insert(instance, insertion);
                panic!("truck route insertion not implemented yet!!!")
            }
            _ => panic!("Origin type is not Sat nor Depot. Is {:?}", origin_type),
        }
    }
}
impl Default for Solution {
    fn default() -> Self {
        Solution { 
            satelites: Vec::new(), 
            trucks: Vec::new(),
            cost: 0.0,
            n_trucks: 0,
            n_evs: 0 
        }

    }
}
pub fn check_solution(solution: &Solution, instance: &Instance) -> bool{
    // TODO move each "for" to separate function that returns or Ok or Err(String), than you can
    // put the error string in the errors vector

    let is_feasible = true;
    let mut errors: Vec<String> = vec![String::from("INVALID SOLUTION!")];
    let mut was_visited = vec![];
    let first_client_index = instance.get_clients()[0].node_id;
    for i in 0..instance.get_n_clients() {
        was_visited.push(false);
    }
    // check if all clients were visited (ONLY ONCE)
    for sat in solution.get_satelites(){
        for ev_route in sat.get_ev_routes(){
            for route_node in ev_route.get_nodes(){
                if route_node.node_type == NodeType::Client{
                    if was_visited[route_node.node_id - first_client_index] {
                        let error = format!("Client {} visited twice", route_node.node_id);
                        errors.push(error);
                    }
                    else{
                        was_visited[route_node.node_id - first_client_index] = true;
                    }
                }
            }
        }
    }
    for (i, client_visited) in was_visited.iter().enumerate(){
        if *client_visited == false{
            let error = format!("Client {} was not visited!", i + first_client_index);
            errors.push(error);
        }
    }
    // check if satelite demand == sum of its clients demands
    for sat in solution.get_satelites(){
        let mut sat_clients_demand = 0f64;
        for ev_route in sat.get_ev_routes(){
            for client in ev_route.get_nodes(){
                sat_clients_demand += client.get_demand(instance);
            }
        }
        if sat_clients_demand != sat.get_demand(){
            let error = format!("Satelite {} with incorrect demand anotation. is {}, should be {}", sat.sat_id, sat.demand, sat_clients_demand);
            errors.push(error);
        }
    }
    // check if all (used) satelites with clients were visited
    let mut sat_ids = Vec::new();
    let mut was_sat_visited = Vec::new();
    for sat in solution.get_satelites(){
        sat_ids.push(sat.sat_id);
        was_sat_visited.push(false);
    }
    for route in solution.get_truck_routes(){
        for node in route.get_nodes(){
            match sat_ids.binary_search(&node.node_id){
                Ok(pos) => was_sat_visited[pos] = true,
                Err(_) => ()
            }
        }
    }
    if was_sat_visited.contains(&false){
        let error = format!("some satelite(s) were not visited"); // TODO: witch satelites were not
                                                                  // visited?
        errors.push(error);
    }
    // check if no time window was violated
    for route in solution.get_truck_routes(){
        let current_time = 0f64;
        let mut last_node = &route.get_nodes()[0];
        for (i, node) in route.get_nodes().iter().enumerate(){
            // TODO use ev_route and route methods!!!! To avoid future incongruencias
            let t_chegada = last_node.tempo_saida + instance.get_distance(last_node.node_id, node.node_id)*route.vehicle.time_per_distance;
            if t_chegada != node.tempo_chegada{
                let error = format!("No noh {} da rota {}: Tempo de chegada diferente do anotado no noh da rota. Tempo de chegada real: {}, tempo anotado: {}", i, node.node_id, t_chegada, node.tempo_chegada); // TODO: witch satelites were not
                errors.push(error);
            }
            if t_chegada > instance.get_node(node.node_id).end_time_window{
                let error = format!("No noh {} da rota {}: tempo de chegada maior que o fim da janela de tempo. Tempo de chegada: {}, fim da janela: {}", i, node.node_id, t_chegada, node.tempo_chegada); // TODO: witch satelites were not
                errors.push(error);
            }
            last_node = node;
        }
    }
    // check client time windowsss
    let mut sats_arrival_time: HashMap<usize, f64> = HashMap::new();
    for route in solution.get_truck_routes(){
        for node in route.get_nodes(){
            let arrives_later = match sats_arrival_time.get(&node.node_id){
                Some(time) => node.tempo_chegada > *time,
                None => true
            };
            if arrives_later{
                sats_arrival_time.insert(node.node_id, node.tempo_chegada);
            }
        }
    }
    for sat in solution.get_satelites(){
        let starting_time = sats_arrival_time.get(&sat.sat_id).unwrap();
        for (i,ev_route) in sat.get_ev_routes().iter().enumerate(){
            let anotado_starting_time = instance.get_node(ev_route.get_nodes()[0].node_id).end_time_window;
            if *starting_time != anotado_starting_time {
                    let error = format!("Tempo de inicio da rota {} de Ev diferente do tempo de chegada no satelite. tempo real de chegada: {}, tempo de chegada anotado: {}", i, starting_time, anotado_starting_time); // TODO: witch satelites were not
                    errors.push(error);
            }
            let mut last_node = &ev_route.get_nodes()[0];
            for (i, node) in ev_route.get_nodes().iter().enumerate(){
                let t_chegada = last_node.tempo_saida + instance.get_distance(last_node.node_id, node.node_id)*ev_route.vehicle.time_per_distance;
                if t_chegada != node.tempo_chegada{
                    let error = format!("No noh {} da rota {}: Tempo de chegada diferente do anotado no noh da rota. Tempo de chegada real: {}, tempo anotado: {}", i, node.node_id, t_chegada, node.tempo_chegada); // TODO: witch satelites were not
                    errors.push(error);
                }
                if t_chegada > instance.get_node(node.node_id).end_time_window{
                    let error = format!("No noh {} da rota {}: tempo de chegada maior que o fim da janela de tempo. Tempo de chegada: {}, fim da janela: {}", i, node.node_id, t_chegada, node.tempo_chegada); // TODO: witch satelites were not
                    errors.push(error);
                }
                last_node = node;
            }
        }
    }
    // check EV battery. TODO: consider recharging stations
    for sat in solution.get_satelites(){
        for (ev_route_index,ev_route) in sat.get_ev_routes().iter().enumerate(){
            let mut ev_battery = ev_route.vehicle.battery.unwrap();
            let mut last_node = &ev_route.get_nodes()[0];
            for (node_index, node) in ev_route.get_nodes().iter().enumerate(){
                let gasto_bateria = instance.get_distance(last_node.node_id, node.node_id)*ev_route.vehicle.battery_per_distance.unwrap(); // these unwraps should always work, otherwise the solution is REALLY invalid
                if ev_battery < gasto_bateria {
                    let error = format!("No noh {} da rota {}: Bateria insuficiente!, atual: {}, gasto do noh: {}", ev_route_index, node.node_id, ev_battery, gasto_bateria); // TODO: witch satelites were not
                    errors.push(error);
                    break;
                }
                ev_battery -= gasto_bateria;
                last_node = node;
            }
        }
    }
    // check if all ev_routes start and end with satelite:
    for sat in solution.get_satelites(){
        for (i, ev_route)  in sat.get_ev_routes().iter().enumerate(){
            if ev_route.len() <= 2{
                let error = format!("route {} has length equals {}, should be >2", i, ev_route.len()); // TODO: witch satelites were not
                errors.push(error);
            }
            if ev_route.get_node(0).node_type != NodeType::Sat || ev_route.get_node(ev_route.len()-1).node_type != NodeType::Sat{
                let error = format!("ev_route doesnt start and end with satelite, start: {:?}, end: {:?}", ev_route.get_node(0).node_type, ev_route.get_node(ev_route.len()-1).node_type); // TODO: witch satelites were not
                errors.push(error);

            }
        }
    }
    if errors.len() >= 2{
        return false
    }
    else{
        return true
    }
    // TODO: check if satelite or depot in the mid of the route;
}

