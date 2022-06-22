use crate::instance::Instance;
use crate::auxStructures::RouteNode;
use crate::auxStructures::Insertion;
use crate::instance::NodeType;
use crate::instance::Vehicle;

#[derive(Debug)]
pub struct Route {
    pub origin_id: usize,
    pub max_size: usize,
    pub current_distance: f64,
    pub current_demand: f64,
    pub nodes: Vec<RouteNode>,
    pub menor_folga: Option<f64>,
    pub time_per_distance: f64,
    pub max_demand: f64, // AKA vehicle capacity
}
impl Route {
    pub fn new(origin_type: NodeType, origin_id: usize, max_size: usize, vehicle: &Vehicle, t_chegada: f64) -> Route { // TODO: recebe instancia, nao tipo de origem.
            let node0= RouteNode::new(t_chegada, t_chegada, origin_type, origin_id);
            let node1= RouteNode::new(t_chegada, t_chegada, origin_type, origin_id);
        Route{
            origin_id,
            max_size,
            nodes: vec![node0, node1],
            current_distance: 0.0,
            current_demand: 0.0,
            max_demand: vehicle.capacity,
            menor_folga: None,
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
        // update distance
        self.current_distance += insertion.distance;
        let node_info = instance.get_node(insertion.node_id);
        let route_node = RouteNode::new(insertion.t_chegada, insertion.t_saida, node_info.node_type, node_info.node_id);
        self.nodes.insert(insertion.pos, route_node);
        // update cost* in evroute
        // update times!!!!!!!!!!!
        self.push_time_windows_starting_at(instance, insertion.pos+1, insertion.time_spent); // also
                                                                                             // updates
                                                                                             // menor
                                                                                             // folga
        /*
        for node in &mut self.nodes[insertion.pos+1 ..]{
            node.tempo_chegada += insertion.time_spent;
            node.tempo_saida += insertion.time_spent;
            let folga = node.get_folga(instance);
            match self.menor_folga{
                Some(menor_folga) => if folga < menor_folga {
                    self.menor_folga = Some(folga);
                }
                None => (),
            }
        }
        */
    }
    pub fn len(&self) -> usize{
        return self.nodes.len();
    }
    pub fn push_time_windows_starting_at(&mut self, instance: &Instance, position: usize, time: f64){
        for node in &mut self.nodes[position ..]{
            node.tempo_chegada += time;
            node.tempo_saida += time;
            let node_folga = node.get_folga(instance);
            match self.menor_folga{
                Some(menor_folga) => if node_folga < menor_folga {
                    self.menor_folga = Some(node_folga);
                }
                None => self.menor_folga = Some(node_folga),
            }
        }


    }
}
