use crate::instance::{Instance, Node};
use crate::instance::NodeType;
use crate::aux_structures::{RouteNode, Insertion};
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

        let menor_folga = self.route.nodes[position].menor_folga_adiante;
        if total_time_spent > menor_folga{
            println!("Quebra menor folga!");
            return None;
        }
        if position == 0 || position >= self.route.max_size{ // if position == 0, its trying to insert before the satelite
            println!("Tentou inserir antes do satelite");
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
            let insertion = Insertion::new(position, this_sat_id, this_route_id, node_id, cost, t_chegada, t_saida, total_time_spent, distance, NodeType::Depot, None);
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
