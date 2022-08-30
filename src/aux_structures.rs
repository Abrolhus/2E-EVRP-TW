use crate::instance::NodeType;
use crate::instance::Instance;
use std::cmp::Ordering;
const BIG_NUMBER: f64 = 1e6;

#[derive(Debug, Clone)]
pub struct RouteNode {
    // remaining_battery: Option<f64>,
    pub tempo_chegada: f64,
    pub tempo_saida: f64,
    pub node_type: NodeType,
    pub node_id: usize,
    pub menor_folga_adiante: f64,
    pub remaining_battery: Option<f64>,
    // Considerando somente as proximas posicoes, armazena a de menor folga (tempo chegada - final janela)
    // int posMenorFolga   = -1;
}
impl RouteNode{
    // default node when creating a empty ev route
    /*
    pub fn default_ev_node(sat_id: usize) -> RouteNode {
        RouteNode{
            tempo_chegada: 0.0,
            tempo_saida: 0.0,
            node_type: NodeType::Sat,
            node_id: sat_id,
            remaining_battery: 
            menor_folga_adiante: BIG_NUMBER, // TODO: colocar folga real do satelite aq, 
        }
    }
    */
    // default node when creating a empty truck route
    /* pub fn default_truck_node() -> RouteNode {
        RouteNode{
            tempo_chegada: 0.0,
            tempo_saida: 0.0,
            node_type: NodeType::Depot,
            node_id: 0,  // depot id
            remaining_battery: None,
            menor_folga_adiante: BIG_NUMBER, // TODO: colocar folga real do deposito aq, 
        }
    } */
    pub fn new(tempo_chegada: f64, tempo_saida: f64, node_type: NodeType, node_id: usize, menor_folga_adiante: f64, remaining_battery: Option<f64>)-> RouteNode{
        RouteNode {
            tempo_chegada,
            tempo_saida,
            node_type,
            node_id,
            menor_folga_adiante,
            remaining_battery,
        }
    }
    pub fn empurrar_janela_tempo(&mut self, time: f64){
        self.tempo_chegada += time;
        self.tempo_saida += time;
    }
    pub fn get_folga(&self, instance: &Instance) -> f64{
        instance.get_node(self.node_id).end_time_window - self.tempo_chegada 
    }
    pub fn get_demand(&self, instance: &Instance) -> f64 {
        instance.get_node_demand(self.node_id)
    }
}

#[derive(Debug)]
pub struct Insertion {
    pub position: usize,
    pub node_id: usize,
    pub origin_id: usize,
    pub route_id: usize,
    pub t_chegada: f64,
    pub cost: f64,
    pub distance: f64,
    pub t_saida: f64,
    pub time_spent: f64,
    pub origin_type: NodeType,
    pub recharge: Option<RechargeInsertion>,
}
impl Insertion{
    pub fn new(pos: usize, origin_id:usize, route_id: usize, node_id: usize, cost:f64, t_chegada:f64, t_saida: f64, time_spent: f64, distance:f64, origin_type: NodeType, recharge: Option<RechargeInsertion>) -> Insertion{
        Insertion { position: pos,
            route_id,
            origin_id,
            node_id, // satelite or depot id
            cost, // TODO: calc cost;
            t_saida,
            distance,
            t_chegada,
            time_spent,
            origin_type,
            recharge,
        }
    }
    pub fn get_demand(&self, instance: &Instance) -> f64 {
        instance.get_node_demand(self.node_id)
    }
}
impl Eq for Insertion{}
impl Ord for Insertion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.partial_cmp(&other.cost).unwrap() // TODO: hehe this thing is gonna give me some
                                                    // headaches
    }
}
impl PartialOrd for Insertion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Insertion {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RechargeInsertion{
    pub station_id: usize,
    pub position: usize,
    pub t_chegada: f64,
    pub t_saida: f64,
    pub battery_before_station: f64,
    pub battery_before_next_station: f64,
    pub recharging_amount: f64,
}
impl RechargeInsertion{
    pub fn new(station_id: usize, position: usize, t_chegada: f64, t_saida: f64, recharging_amount: f64, battery_before_station: f64, battery_before_next_station: f64) -> RechargeInsertion{
        RechargeInsertion{
            station_id,
            position,
            t_chegada,
            t_saida,
            battery_before_next_station,
            battery_before_station,
            recharging_amount,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SingleRouteInsertion{
    pub node_id: usize,
    pub origin_id: usize,
    pub cost: f64,
    pub origin_type: NodeType,
    pub recharge_before: Option<RechargeInsertion>,
    pub recharge_after:  Option<RechargeInsertion>,
}
impl SingleRouteInsertion{
    pub fn new(origin_id:usize, node_id: usize, cost:f64, origin_type: NodeType, recharge_before: Option<RechargeInsertion>, recharge_after: Option<RechargeInsertion>) -> SingleRouteInsertion{
        SingleRouteInsertion { 
            node_id,
            origin_id,
            cost,
            origin_type,
            recharge_after,
            recharge_before,
        }
    }
    pub fn get_demand(&self, instance: &Instance) -> f64 {
        instance.get_node_demand(self.node_id)
    }
}
impl Eq for SingleRouteInsertion{}
impl Ord for SingleRouteInsertion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.partial_cmp(&other.cost).unwrap() // TODO: hehe this thing is gonna give me some
                                                    // headaches
    }
}
impl PartialOrd for SingleRouteInsertion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for SingleRouteInsertion {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
