use std::cmp::max;
use std::collections::HashMap;

use crate::instance::Instance;
use crate::instance::Node;
use crate::aux_structures::RouteNode;
use crate::instance::NodeType;
use crate::route::Route;
use crate::instance::Vehicle;
use crate::aux_structures::{Insertion, RechargeInsertion};


#[derive(Debug, Clone)]
struct Recharge{
    remaining_battery: f64,
    node_id: usize,
    first_client_in_zone: usize,
}

#[derive(Debug, Clone)]
pub struct EvRoute{
    route: Route,
    pub vehicle: Vehicle,
    recharges: HashMap<usize, Recharge>
    // current_battery: f64, //  TODO: battery differs depending on where you are on the route
} 
impl EvRoute {
    pub fn new(instance: &Instance, satelite_id: usize, vehicle: &Vehicle, satelite_starting_time: f64) -> EvRoute{
        let battery = match vehicle.battery{
            Some(battery) => battery,
            None => panic!("In EvRoute, vehicle should be Electric but is truck"),
        };
       let mut recharges = HashMap::new();
        recharges.insert(satelite_id, Recharge{remaining_battery: battery, node_id: satelite_id});
        EvRoute {
            // current_battery: battery,
            route: Route::new(crate::instance::NodeType::Sat, satelite_id, instance.get_n_clients() + instance.get_n_stations() + 2, vehicle, satelite_starting_time), 
            vehicle: *vehicle,
            recharges,
        }
    }
    pub fn insert(&mut self, instance: &Instance, insertion: &Insertion){
        let battery_cost = insertion.distance*self.vehicle.battery_per_distance.unwrap();
        match &insertion.recharge{
            Some(recharge) => {
                let mut current_recharge_zone = self.get_mut_recharging_zone(insertion.position).unwrap();
                current_recharge_zone.remaining_battery = recharge.battery_before_next_station;
                self.recharges.insert(recharge.station_id, Recharge { remaining_battery: recharge.battery_before_station, node_id: insertion.position });
            }
            None => {
                let mut current_recharge_zone = self.get_mut_recharging_zone(insertion.position).unwrap();
                if current_recharge_zone.remaining_battery < battery_cost{
                    panic!("Not enough battery to insert node {} in evRoute {}. current bat: {}. Cost: {}", insertion.node_id, insertion.route_id, current_recharge_zone.remaining_battery, battery_cost);
                }
                current_recharge_zone.remaining_battery -= battery_cost;
            }
        }
        self.route.insert(instance, insertion, true);
        /*
        match &insertion.recharge{
            Some(recharge) => {
                let battery = &mut self.vehicle.battery.clone().unwrap();
                let mut current_recharge_zone = self.get_mut_recharging_zone(insertion.pos).unwrap();
                current_recharge_zone.remaining_battery = *battery - recharge.recharging_amount;
                self.recharges.insert(recharge.station_id, Recharge { remaining_battery: self.vehicle.battery.unwrap(), node_id: insertion.pos });
                let current_recharge_zone = self.get_mut_recharging_zone(insertion.pos).unwrap();
            }
            None =>{
                let mut current_recharge_zone = self.get_mut_recharging_zone(insertion.pos).unwrap();
                if current_recharge_zone.remaining_battery < battery_cost{
                    panic!("Not enough battery to insert node {} in evRoute {}. current bat: {}. Cost: {}", insertion.node_id, insertion.route_id, current_recharge_zone.remaining_battery, battery_cost);
                }
                current_recharge_zone.remaining_battery -= battery_cost;
            }

        }
        self.route.insert(instance, insertion, true);
        */
    }
    pub fn can_insert_node(&self, instance: &Instance, node_id: usize, position: usize, this_route_id: usize, this_sat_id: usize) -> Option<Insertion>{
        let node_info = instance.get_node(node_id);
        let prev_node = self.get_node(position-1); //(usize) note that position is always > 0
        let next_node = self.get_node(position);
        // let time_to_next_node = distance_to_next_node*self.vehicle.time_per_distance;
        let distance = instance.get_insert_distance(node_id, prev_node.node_id, next_node.node_id);
        let distance_to_node = instance.get_distance(prev_node.node_id, node_id);
        let time_to_node = distance_to_node*self.vehicle.time_per_distance;
        let t_chegada = prev_node.tempo_saida + time_to_node;
        let t_saida =  t_chegada + node_info.service_time; // ele chega no noh, fica o tempo de servico e dps vai embora
        let cost = distance*self.vehicle.cost_per_distance;
        let battery_cost = distance*self.vehicle.battery_per_distance.unwrap();
        // let total_time_spent  = instance.get_insert_time(node_id, prev_node.node_id, next_node.node_id, self.vehicle.vehicle_id);
        let total_time_spent = instance.get_insert_distance(node_id, prev_node.node_id, next_node.node_id) + node_info.service_time;
        let menor_folga = self.route.nodes[position].menor_folga_adiante; //menor folga CONTANDO O NO na position
        if total_time_spent > menor_folga{
            println!("Quebra menor folga!");
            return None;
        }
        else if position == 0 || position >= self.route.max_size{ // if position == 0, its trying to
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
        let current_recharge_zone = self.get_recharging_zone(position).0.unwrap();
        if battery_cost > current_recharge_zone.remaining_battery{
            let recharge = self.can_insert_station_in_route(instance, node_id, position, total_time_spent, this_route_id)?;
            let distance_from_station_to_client = instance.get_distance(recharge.node_id, node_id);
            let t_chegada_cliente = recharge.t_saida + distance_from_station_to_client*self.vehicle.battery_per_distance.unwrap();
            let t_saida_cliente = t_chegada_cliente.max(node_info.start_time_window) + instance.get_node(node_id).service_time;
            // let insertion = Insertion::new(position, this_sat_id, this_route_id, node_id, cost, t_chegada_cliente, t_saida_cliente, total_time_spent, distance, crate::instance::NodeType::Sat, Some(recharge));
            let insertion = Insertion::new(position, this_sat_id, this_route_id, node_id, cost, t_chegada_cliente, t_saida_cliente, total_time_spent, distance, crate::instance::NodeType::Sat, None); // TODO FIXXXXXXXXXXXXX to commented line above
        }
        else{
            // TODO add cost if its recharging station
            // TODO also, add time if its recharging station. Actually, add different insert function
            // for RSs 
            let insertion = Insertion::new(position, this_sat_id, this_route_id, node_id, cost, t_chegada, t_saida, total_time_spent, distance, crate::instance::NodeType::Sat, None);
            return Some(insertion);
        }
        return None;
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
    fn get_mut_recharging_zone(&mut self, node_index: usize) -> Option<&mut Recharge> {
        for i in node_index..self.route.nodes.len(){
            let node = &self.route.nodes[i];
            if node.node_type == NodeType::Station{
                return self.recharges.get_mut(&node.node_id);
            } else if node.node_type == NodeType::Sat && i != 0{
                return self.recharges.get_mut(&node.node_id);
            }
        }
        None
    }
    fn get_recharging_zone(&self, node_index: usize) -> (Option<&Recharge>, usize) {
        for i in node_index..self.route.nodes.len(){
            let node = &self.route.nodes[i];
            if node.node_type == NodeType::Station{
                return (self.recharges.get(&node.node_id), i);
            } else if node.node_type == NodeType::Sat && i != 0{
                return (self.recharges.get(&node.node_id), i);
            }
        }
        (None, 0)
    }
    fn can_insert_station_in_route(&self, instance: &Instance, client_id: usize, position: usize, client_insertion_time: f64, route_id: usize) -> Option<Insertion>{
        let client_info = instance.get_node(client_id);
        let (recharge, next_station_position) =  self.get_recharging_zone(position);
        let recharge = recharge.expect("position doesnt have recharging zone, but should");
        let prev_node = &self.route.nodes[position-1];
        let battery_prev = prev_node.remaining_battery.unwrap(); // battery left at that position
                                                                 // (not battery left in the pool)
        let (best_station_id, station_distance)  = instance.get_best_station_to_insert_between(prev_node.node_id, client_id);
        let battery_at_station_insertion = battery_prev - station_distance;
        if  battery_at_station_insertion < 0.0 {
            return None;
        }
        let next_node = &self.route.nodes[position];
        let bateria_next = next_node.remaining_battery.unwrap();
        let bateria_gasta_ate_next_node = self.vehicle.battery.unwrap() - bateria_next;

        let distance_station_client = instance.get_distance(best_station_id, client_id);
        let distance_client_next = instance.get_distance(client_id, next_node.node_id);
        let battery_left_next_station_after_insert = 
            recharge.remaining_battery 
            + bateria_gasta_ate_next_node 
            - distance_station_client 
            - distance_client_next;
        if battery_left_next_station_after_insert < 0.0 {
            println!("Bateria insuficiente na prox estacao: {}", battery_left_next_station_after_insert);
            return None;
        }
        // checou bateria, agora basta checar demanda (que nao precisa) e janelas de tempo!

        let distance_prevn_station = instance.get_distance(prev_node.node_id, best_station_id);
        let old_distance_prev_next = instance.get_distance(prev_node.node_id, next_node.node_id);
        let total_distance = 
            distance_prevn_station 
            + distance_station_client 
            + distance_client_next 
            - old_distance_prev_next;
        let total_insertion_time = total_distance*self.vehicle.time_per_distance;
        let bateria_adicionada = self.vehicle.battery.unwrap() - battery_at_station_insertion;
        let tempo_recarga = self.vehicle.recharging_rate.unwrap()*bateria_adicionada;
        let total_insertion_time = total_insertion_time + tempo_recarga;
        let t_chegada_station = prev_node.tempo_saida + distance_prevn_station*self.vehicle.time_per_distance;
        let t_saida_station = t_chegada_station + tempo_recarga;
        let t_chegada_cliente = distance_station_client*self.vehicle.time_per_distance + tempo_recarga;
        let t_saida_cliente = t_chegada_cliente + client_info.service_time;
        let cost = total_distance*self.vehicle.cost_per_distance;
        let recharge_insertion = RechargeInsertion::new(
            best_station_id, 
            position, 
            t_chegada_station, 
            t_saida_station, 
            bateria_adicionada, 
            battery_at_station_insertion, 
            battery_left_next_station_after_insert,
        );
        if next_node.menor_folga_adiante < total_insertion_time{
            return None;
        }
        let insertion = Insertion::new(
            position,
            self.route.origin_id,
            route_id, 
            client_id,
            cost, 
            t_chegada_cliente, 
            t_saida_cliente, 
            total_insertion_time, 
            total_distance, 
            NodeType::Sat, 
            Some(recharge_insertion),
        );
        return Some(insertion);
        /*
        let mut bateria_max = self.vehicle.battery.unwrap();
        let prev_node = &self.route.nodes[station_position];
        // descobre bateria no noh anterior ao lugar q vai inserir a estacao e o cliente
        let node_before_station = &self.route.nodes[position-1];
        let node_after_client = &self.route.nodes[position];
        let bateria_antes_estacao = bateria_max;
        let (best_station_id, station_distance)  = instance.get_best_station_to_insert_between(node_before_station.node_id, client_id);
        let distance_to_station = instance.get_distance(prev_node.node_id, best_station_id);
        let distance_from_station_to_client = instance.get_distance(best_station_id, client_id);
        let distance_from_client_to_next = instance.get_distance(client_id, node_after_client.node_id);
        let distance_before_insertion = instance.get_distance(node_before_station.node_id, node_after_client.node_id);

        let distancia_total = distance_to_station + distance_from_station_to_client + distance_from_client_to_next - distance_before_insertion;

        let t_chegada_estacao  = node_before_station.tempo_saida + distance_to_station*self.vehicle.time_per_distance;
        let bateria_na_estacao = bateria_antes_estacao - self.vehicle.battery_per_distance.unwrap()*distance_to_station;
        if bateria_na_estacao < 0.0{
            println!("BAteria na estacao < 0.0");
            return None;
        }
        let falta_x_bateria = -(recharge.remaining_battery - distancia_total*self.vehicle.battery_per_distance.unwrap());
        let precisa_x_bateria = -(recharge.remaining_battery - (distance_from_station_to_client + distance_from_client_to_next)*self.vehicle.battery_per_distance.unwrap());
        if recharge.remaining_battery < distance_to_station*self.vehicle.battery_per_distance.unwrap(){
            return None;
        }

        let ganho_de_bateria = self.vehicle.battery.unwrap() - bateria_na_estacao;
        if ganho_de_bateria < precisa_x_bateria{
            println!("ganho de bateria < falta bateria");
            println!("bateria na estacao: {}", bateria_na_estacao);
            println!("ganho: {}, falta: {}", ganho_de_bateria, precisa_x_bateria);
            return None;
        }
        let tempo_recarga = ganho_de_bateria*self.vehicle.recharging_rate.unwrap();
        let t_saida_estacao = t_chegada_estacao + tempo_recarga;
        let t_chegada_cliente = t_saida_estacao + distance_from_station_to_client*self.vehicle.battery_per_distance.unwrap();
        let t_saida_cliente = t_chegada_cliente + instance.get_node(client_id).service_time;
        let t_chegada_next = t_saida_cliente + distance_from_client_to_next*self.vehicle.battery_per_distance.unwrap();
        let total_insertion_time = t_chegada_next - node_before_station.tempo_saida;
        if total_insertion_time > self.get_node(position).menor_folga_adiante{
            println!("insertion time < menor folga");
            return None;
        }
        let recharge = RechargeInsertion::new(best_station_id, position, t_chegada_estacao, t_saida_estacao, ganho_de_bateria);
        Some(recharge)
        */
    }
}
fn check_time_window(t_chegada: f64, node_info: &Node) -> bool {
    t_chegada < node_info.end_time_window
}
