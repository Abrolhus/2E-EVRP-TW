use crate::instance::{Instance, Node};
use crate::instance::NodeType;
use crate::auxStructures::{RouteNode, Insertion};
use crate::instance::VehicleType;
use crate::route::Route;
use crate::instance::Vehicle;

#[derive(Debug)]
pub struct TruckRoute{
    pub route: Route,
    pub vehicle: Vehicle,
}
impl TruckRoute{
    pub fn new(instance: &Instance, vehicle: &Vehicle) -> TruckRoute{
        if vehicle.vehicle_type != VehicleType::Truck{
            panic!("Vehicle should be Truck, not {:?}", vehicle.vehicle_type);
        }
        let depot_id = instance.get_depot().node_id;
        TruckRoute {
            route: Route::new(NodeType::Depot, depot_id, instance.get_n_sats()+2, vehicle, 0f64), 
            vehicle: *vehicle
        }
    }
    pub fn push(&mut self, instance: &Instance, node_id: usize){
        self.route.push(instance, node_id);
    }
    pub fn get_nodes(&self) -> &Vec<RouteNode>{
        &self.route.nodes
    }
    pub fn get_node(&self, position: usize) -> &RouteNode{
        &self.route.nodes[position]
    }
    pub fn can_insert_node(&self, instance: &Instance, node_id: usize, position: usize, this_route_id: usize, this_sat_id: usize) -> Option<Insertion>{
        // demand, VV
        // next time windows, VV
        // this node time window, VV
        // route max length (DEBUG) VV
        //
        let node_info = instance.get_node(node_id);
        let prev_node = self.get_node(position-1); //(usize) note that position is always > 0
        let next_node = self.get_node(position);
        let distance_to_node = instance.get_distance(prev_node.node_id, node_id);
        let distance_to_next_node = instance.get_distance(node_id, next_node.node_id);
        let mut distance = 0.0;
        distance += distance_to_node;
        distance += distance_to_next_node;
        distance -= instance.get_distance(prev_node.node_id, next_node.node_id);
        let time_from_prev_to_node = distance_to_node*self.vehicle.time_per_distance;
        let time_to_next_node = distance_to_next_node*self.vehicle.time_per_distance;
        let t_chegada = prev_node.tempo_saida + time_from_prev_to_node;
        let t_saida =  t_chegada + node_info.service_time; // ele chega no noh, fica o tempo de
                                                           // servico e dps vai embora
        let cost = distance*self.vehicle.cost_per_distance;
        // let total_time_spent = t_saida + distance_to_next_node*self.vehicle.time_per_distance;
        let total_time_spent = t_saida + time_to_next_node - next_node.tempo_chegada;

        if match self.route.menor_folga{
            None => false,
            Some(menor_folga) => {
                if total_time_spent > menor_folga{
                    true
                } else{
                    false
                }
            }
        }{
            println!("Quebra menor folga!");
            return None;

        }
        if position == 0 || position >= self.route.max_size{ // if position == 0, its trying to
                                                             // insert before the satelite
            println!("Quebra route max size");
            return None;
        }
        else if node_info.demand > self.route.get_remaining_capacity(){
            println!("Quebra route demand");
            return None;
        } // TODO: deixar split de demanda
        if check_time_window(t_chegada, node_info) == false {
            println!("quebra janela de tempo do cliente");
            return None;
        }
        else{
            // TODO add cost if its recharging station
            // TODO also, add time if its recharging station. Actually, add different insert function
            // for RSs 
            let insertion = Insertion::new(position, this_sat_id, this_route_id, node_id, cost, t_chegada, t_saida, total_time_spent, distance, NodeType::Depot);
            return Some(insertion);
        }
    }

    pub fn insert(&self, instance: &Instance, insertion: &Insertion){
        todo!()
    }
}
fn check_time_window(t_chegada: f64, node_info: &Node) -> bool {
    t_chegada < node_info.end_time_window
}

/*
pub struct TruckRoute {
    depot: Option<u32>,
    id: Option<u32>,
    max_size: Option<usize>,
    current_distance: f64,
    current_demand: f64,
    route: Vec<RouteNode>,
    capacity: Option<f64>,
    cost_per_distance: f64,
    // pub time_per_distance: f64,
    vehicle_cost: f64,
}

impl Default for TruckRoute {
    fn default() -> Self {
        TruckRoute { 
            depot: Some(0), 
            id: None,
            max_size: None,
            current_distance: 0.0,
            current_demand: 0.0,
            route: Vec::new(),
            vehicle_cost: 0.0,
            capacity: None,
            cost_per_distance: 1.0,
        }
    }
}

impl TruckRoute {
    pub fn new(instance: &Instance) -> TruckRoute {
        let route = vec![RouteNode::default_truck_node(), RouteNode::default_truck_node()]; 
        TruckRoute{
            max_size: Some(instance.get_n_sats() +2),
            route,
            ..Default::default()
        }
    }
    pub fn check_route(&self) -> bool {
        false
    }
    pub fn can_push_node(&self, instance: Instance, node_id: usize) -> bool {
        let position = self.route.len();
        let prev_node: &RouteNode = &self.route[position];
        let distance_from_route: f64 = instance.get_distance(prev_node.node_id, node_id);
        let tempo_chegada = prev_node.tempo_chegada + prev_node.tempo_servico + distance_from_route*self.cost_per_distance;
        let tempo_servico = instance.get_node(node_id).service_time;
        let node_type = &instance.get_node(node_id).node_type;
        if *node_type == NodeType::Client{
            panic!("shouldnt be client");
        }
        // checa se estoura alguma janela de tempo dos nós seguintes
        let mut last_node_id = node_id;
        let mut t_saida = tempo_chegada + tempo_servico;
        for node in &self.route[position..]{
            let tw_end = instance.get_node(node_id).end_time_window;
            let t_chegada = t_saida + instance.get_distance(last_node_id, node.node_id);
            let t_servico = instance.get_node(node_id).service_time;
            if tw_end < t_chegada {
                return false;
            }
            last_node_id = node.node_id;
            t_saida = t_chegada + t_servico;
        }
        //self.route.insert(
        return true;
    }
    pub fn push_node(&self, instance: Instance, node_id: usize) -> bool {
        let position = self.route.len();
        let prev_node: &RouteNode = &self.route[position];
        let distance_from_route: f64 = instance.get_distance(prev_node.node_id, node_id);
        let tempo_chegada = prev_node.tempo_chegada + prev_node.tempo_servico + distance_from_route*self.cost_per_distance;
        let tempo_servico = instance.get_node(node_id).service_time;
        let node_type = &instance.get_node(node_id).node_type;
        if *node_type == NodeType::Client  || *node_type == NodeType::Depot {
            panic!("node type should be satelite");
        }
        // checa se estoura alguma janela de tempo dos nós seguintes
        let mut last_node_id = node_id;
        let mut t_saida = tempo_chegada + tempo_servico;
        for node in &self.route[position..]{
            let tw_end = instance.get_node(node_id).end_time_window;
            let t_chegada = t_saida + instance.get_distance(last_node_id, node.node_id);
            let t_servico = instance.get_node(node_id).service_time;
            if tw_end < t_chegada {
                return false;
            }
            last_node_id = node.node_id;
            t_saida = t_chegada + t_servico;
        }
        //self.route.insert(
        return true;
    }
}
*/
