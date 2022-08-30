use std::cmp::min;

use crate::aux_structures::RechargeInsertion;
use crate::instance::Instance;
use crate::aux_structures::RouteNode;
use crate::aux_structures::Insertion;
use crate::instance::NodeType;
use crate::instance::Vehicle;
use crate::instance::VehicleType;
const BIG_NUMBER: f64 = 1e6;

#[derive(Debug, Clone)]
pub struct Route {
    pub origin_id: usize,
    pub max_size: usize,
    pub current_distance: f64,
    pub current_demand: f64,
    pub nodes: Vec<RouteNode>,
    pub time_per_distance: f64,
    pub max_demand: f64, // AKA vehicle capacity
}
impl Route {
    pub fn new(origin_type: NodeType, origin_id: usize, max_size: usize, vehicle: &Vehicle, t_chegada: f64) -> Route { // TODO: recebe instancia, nao tipo de origem.
        let remaining_battery = match vehicle.vehicle_type{
            VehicleType::Electric => Some(vehicle.battery.unwrap()),
            VehicleType::Truck    => None,
        };
        let node0 = RouteNode::new(t_chegada, t_chegada, origin_type, origin_id, BIG_NUMBER, remaining_battery);
        let node1 = RouteNode::new(t_chegada, t_chegada, origin_type, origin_id, BIG_NUMBER, remaining_battery);
        Route{
            origin_id,
            max_size,
            nodes: vec![node0, node1],
            current_distance: 0.0,
            current_demand: 0.0,
            max_demand: vehicle.capacity,
            time_per_distance: vehicle.time_per_distance,
        }
    }
    pub fn get_remaining_capacity(&self) -> f64{
        self.max_demand - self.current_demand
    }
    pub fn push(&mut self, instance: &Instance, node_id: usize){
        todo!()
    }
    /*
    pub fn push(&mut self, instance: &Instance, node_id: usize){
        if self.nodes.len() > self.max_size{
            panic!("route size greater than its maximum size, size={} / max_size={}", self.nodes.len(), self.max_size);
        } else if self.nodes.len() == self.max_size{
            panic!("route size equals it max size, cant insert");
        }
        let last_node = &self.nodes[self.nodes.len()-2];
        println!("lastNode: {:?}", last_node);
        let node_info = instance.get_node(node_id);
        let distance = instance.get_distance(last_node.node_id, node_info.node_id);
        let t_chegada = last_node.tempo_saida + distance*self.time_per_distance;
        let route_node = RouteNode::new(t_chegada, t_chegada + node_info.service_time, node_info.node_type, node_id);
        self.current_distance += distance;
        self.current_demand += node_info.demand;
        //TODO: update last node time windows
        let route_last_node = &mut self.nodes[self.len()-1];
            route_last_node.tempo_chegada += distance*self.time_per_distance + distance_to_end; //TODO: add function
                                                                              //to routeNode that
                                                                              //pushes time windows
            route_last_node.tempo_saida += distance*self.time_per_distance  distance_to_end;

        self.nodes.insert(self.nodes.len()-1, route_node); // TODO: push than swap instead of
                                                           // inserting

    }
    */
    pub fn insert(&mut self, instance: &Instance, insertion: &Insertion, is_safe_insertion: bool){
        // insert recharge, insert node
        // update demand
        // update time arrival times, check time windowssss
        // update bateries
        if is_safe_insertion{
            if self.nodes.len() > self.max_size{
                panic!("route size greater than its maximum size");
            } else if self.nodes.len() == self.max_size{
                panic!("route size equals it max size, cant insert");
            }
            else if insertion.get_demand(instance) > self.get_remaining_capacity(){
                panic!("Not enough capacity to insert in route");
            }
        }
        // update demand
        self.current_demand += insertion.get_demand(instance);
        
        let route_node = RouteNode::new(-1.0, -1.0, NodeType::Client, insertion.node_id, -1.0, None);
        self.nodes.insert(insertion.position, route_node);
        match insertion.recharge{ 
            Some(recharge) => {
                let recharge_node = RouteNode::new(-1.0, -1.0, NodeType::Station, recharge.station_id, -1.0, None);
                self.nodes.insert(recharge.position, route_node);
            },
            None => {
            }
        }
        let total_distance = 0.0;
        for (i, route_node) in self.nodes.iter().enumerate(){

        }
        // update distance
        self.current_distance += insertion.distance;
        let node_info = instance.get_node(insertion.node_id);
        /*
        match insertion.recharge{
            Some(recharge) => {
                let station_info = instance.get_node(recharge.station_id);
                let station_node = RouteNode::new(recharge.t_chegada, recharge.t_chegada, NodeType::Station, recharge.station_id, 0.0, Some(recharge.battery_before_station));
                self.nodes.insert(insertion.pos, station_node);
            },
            None => {},
        }
        */
        let route_node = RouteNode::new(insertion.t_chegada, insertion.t_saida, NodeType::Client, insertion.node_id, 0.0, Some(0.0));
        self.nodes.insert(insertion.position+1, route_node);
        self.push_time_windows_starting_at(instance, insertion.position+1, insertion.time_spent); // also updates menor folga
        let menor_folga_proximo = self.nodes[insertion.position].menor_folga_adiante;
        let mut route_node = RouteNode::new(insertion.t_chegada, insertion.t_saida, node_info.node_type, node_info.node_id, 0.0, None);
        if route_node.get_folga(instance) < menor_folga_proximo{
            route_node.menor_folga_adiante = route_node.get_folga(instance);
        } else{
            route_node.menor_folga_adiante = menor_folga_proximo;
        }

        for (i, route_node) in &mut self.nodes.iter().enumerate(){
            let mut menor_folga = 1e6;
            for (j, route_node) in &mut self.nodes.iter().enumerate(){
            }
        }
//         self.nodes.insert(insertion.pos, route_node);
        // update cost* in evroute
        // update times!!!!!!!!!!!
    }
    pub fn insert_recharge(&mut self, instance: &Instance, recharge_insertion: &RechargeInsertion, is_safe_insertion: bool){
        let node_info = instance.get_node(recharge_insertion.station_id);
        let menor_folga_proximo = self.nodes[recharge_insertion.position].menor_folga_adiante;
        let mut route_node = RouteNode::new(recharge_insertion.t_chegada, recharge_insertion.t_saida, node_info.node_type, node_info.node_id, 0.0, None);// TODO: add remaining baterry
        if route_node.get_folga(instance) < menor_folga_proximo{
            route_node.menor_folga_adiante = route_node.get_folga(instance);
        } else{
            route_node.menor_folga_adiante = menor_folga_proximo;
        }
        self.nodes.insert(recharge_insertion.position, route_node);
        // update cost* in evroute
        // update times!!!!!!!!!!!
    }
    pub fn len(&self) -> usize{
        return self.nodes.len();
    }
    pub fn push_time_windows_starting_at(&mut self, instance: &Instance, position: usize, time: f64){
        for node in &mut self.nodes[position ..]{
            node.empurrar_janela_tempo(time);
        }


    }
}
