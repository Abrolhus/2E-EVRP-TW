use crate::instance::Instance;
use crate::instance::Node;
use crate::auxStructures::RouteNode;
use crate::route::Route;
use crate::instance::Vehicle;
use crate::auxStructures::Insertion;


#[derive(Debug)]
pub struct EvRoute{
    route: Route,
    pub vehicle: Vehicle,
    current_battery: f64, //  TODO: battery differs depending on where you are on the route
}
/*
pub struct EvRoute {
    satelite: Option<usize>,
    max_size: Option<usize>,
    current_distance: f64,
    current_demand: f64,
    route: Vec<RouteNode>,
}
*/
impl EvRoute {
    pub fn new(instance: &Instance, satelite_id: usize, vehicle: &Vehicle, satelite_starting_time: f64) -> EvRoute{
        let battery = match vehicle.battery{
            Some(battery) => battery,
            None => panic!("In EvRoute, vehicle should be Electric but is truck"),
        };
        EvRoute {
            current_battery: battery,
            route: Route::new(crate::instance::NodeType::Sat, satelite_id, instance.get_n_clients() + instance.get_n_stations() + 2, vehicle, satelite_starting_time), 
            vehicle: *vehicle
        }
    }
    /*
    pub fn push(&self, instance: &Instance, node_id: usize){
        self.current_battery -= self.vehicle.battery_per_distance*distance;
        self.route.push(instance, node_id);
    }
    */
    // returns a  insertin object if able to insert, otherwise returns None
    /*
    pub fn can_push(&self, instance: &Instance, node_id: usize) -> Option<Insertion>{
        match self.route.max_size{
            Some(length) => {
                if self.route.len() > length{
                    panic!("route size greater than its maximum size");
                } else if self.route.len() == length{
                    panic!("route size equals it max size, cant insert");
                }
            }
            None => {} // TODO: use better syntax, more sugar
        }
        let last_node = self.route.nodes[self.route.len()-2];
        let origin_id = self.route.nodes[self.route.len()-1].node_id;
        let t_chegada = last_node.tempo_chegada + last_node.tempo_saida;
        let node_info = instance.get_node(node_id);
        if self.route.current_demand + node_info.demand > self.vehicle.capacity {
            return None;
        }
        let distance = instance.get_distance(last_node.node_id, node_info.node_id )
                     + instance.get_distance(origin_id, node_info.node_id )
                     - instance.get_distance(last_node.node_id, origin_id);
        let total_time_spent = (distance + self.route.current_distance) * self.vehicle.time_per_distance;
        let total_distance = (distance + self.route.current_distance) * self.vehicle.time_per_distance;
        let total_distance_to_node = self.route.current_distance + distance - instance.get_distance(origin_id, node_info.node_id);
        let time_til_node = total_distance_to_node*self.vehicle.time_per_distance;
        if time_til_node > instance.get_node(node_id).end_time_window{
            return None;
        }
        if total_distance * self.vehicle.battery_per_distance? > self.vehicle.battery?{
            return None;
        }
        let cost = distance*self.vehicle.cost_per_distance;
        let time_spent = distance*self.vehicle.time_per_distance;
        let insertion: Insertion = Insertion::new(self.route.len()-1, None, node_id, None, cost, time_spent, distance); 
        Some(insertion)
    }
    */
    pub fn insert(&mut self, instance: &Instance, insertion: &Insertion){
        let battery_cost = insertion.distance*self.vehicle.battery_per_distance.unwrap();
        if self.current_battery < battery_cost{
            panic!("Not enough battery to insert node {} in evRoute {}. current bat: {}. Cost: {}", insertion.node_id, insertion.route_id, self.current_battery, battery_cost);
        }
        self.current_battery -= battery_cost;
        self.route.insert(instance, insertion, true);
    }
    pub fn can_insert_node(&self, instance: &Instance, node_id: usize, position: usize, this_route_id: usize, this_sat_id: usize) -> Option<Insertion>{
        // demand, VV
        // next time windows, VV
        // this node time window, VV
        // vehicle battery VV
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
        //let battery_cost = distance_to_next_node*self.vehicle.battery_per_distance.unwrap();
        let battery_cost = distance*self.vehicle.battery_per_distance.unwrap();
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
        }
        if check_time_window(t_chegada, node_info) == false {
            println!("quebra janela de tempo do cliente");
            return None;
        }
        else if battery_cost > self.current_battery{
            println!("quebra bateria do veiculo");
            return None;
        }
        else{
            // TODO add cost if its recharging station
            // TODO also, add time if its recharging station. Actually, add different insert function
            // for RSs 
            let insertion = Insertion::new(position, this_sat_id, this_route_id, node_id, cost, t_chegada, t_saida, total_time_spent, distance, crate::instance::NodeType::Sat);
            return Some(insertion);
        }
    }
    pub fn get_node(&self, position: usize) -> &RouteNode{
        &self.route.nodes[position]
    }
    pub fn get_nodes(&self) -> &Vec<RouteNode>{
        &self.route.nodes
    }
    pub fn len(&self) -> usize{
        self.route.len()
    }
    pub fn get_satelite_id(&self) -> usize {
        self.route.origin_id
    }
}
fn check_time_window(t_chegada: f64, node_info: &Node) -> bool {
    t_chegada < node_info.end_time_window
}
/*
impl EvRoute {
    pub fn new(satelite: usize, max_size: usize) -> EvRoute {
        EvRoute{
            satelite: Some(satelite),
            max_size: Some(max_size),
            ..Default::default()
        }
    }
    pub fn new_empty_route(sat_id: usize) -> EvRoute{ // empty = starting at the satelite and ending at the satelite

        EvRoute{
            satelite: Some(sat_id),
            max_size: None,
            current_demand: 0.0,
            current_distance: 0.0,
            route: vec![RouteNode::default_ev_node(sat_id), 
                        RouteNode::default_ev_node(sat_id)],
        }
    }
    pub fn check_route(&self) -> bool {
        false
    }
    pub fn size(&self) -> usize {
        self.route.len()
    }
    pub fn get_nodes(&self) -> &Vec<RouteNode> {
        &self.route
    }
    pub fn get_insertion_cost(&self, instance:Instance, position:i32, node_id:usize) -> f64{
        0.0
    }
    pub fn get_insertion_time_cost(&self, instance:Instance, position:i32, node_id:usize) -> f64{
        0.0
    }
}
*/
