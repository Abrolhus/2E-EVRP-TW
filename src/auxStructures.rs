use crate::instance::NodeType;
use crate::instance::Instance;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct RouteNode {
    // remaining_battery: Option<f64>,
    pub tempo_chegada: f64,
    pub tempo_saida: f64,
    pub node_type: NodeType,
    pub node_id: usize,
    // Considerando somente as proximas posicoes, armazena a de menor folga (tempo chegada - final janela)
    // int posMenorFolga   = -1;
}
impl RouteNode{
    // default node when creating a empty ev route
    pub fn default_ev_node(sat_id: usize) -> RouteNode {
        RouteNode{
            tempo_chegada: 0.0,
            tempo_saida: 0.0,
            node_type: NodeType::Sat,
            node_id: sat_id
        }
    }
    // default node when creating a empty truck route
    pub fn default_truck_node() -> RouteNode {
        RouteNode{
            tempo_chegada: 0.0,
            tempo_saida: 0.0,
            node_type: NodeType::Depot,
            node_id: 0  // depot id
        }
    }
    pub fn new(tempo_chegada: f64, tempo_saida: f64, node_type: NodeType, node_id: usize)-> RouteNode{
        RouteNode { 
            tempo_chegada, 
            tempo_saida,
            node_type,
            node_id,
        }
    }
    pub fn get_folga(&self, instance: &Instance) -> f64{
        instance.get_node(self.node_id).end_time_window - self.tempo_chegada 
    }
    pub fn get_t_serviço(&self, instance: &Instance) -> f64{
        instance.get_node(self.node_id).service_time
    }
    // tempo de espera ==> tempo que o veiculo fica esperando ate a janela do cliente abrir
    pub fn get_t_espera(&self, instance: &Instance) -> f64{
        let node_info = instance.get_node(self.node_id);
        node_info.start_time_window - self.tempo_chegada
    }
    pub fn get_demand(&self, instance: &Instance) -> f64 {
        instance.get_node_demand(self.node_id)
    }
}

#[derive(Debug)]
pub struct Insertion {
    pub pos: usize,
    pub node_id: usize,
    pub origin_id: usize,
    pub route_id: usize,
    pub t_chegada: f64,
    pub cost: f64,
    pub distance: f64,
    pub t_saida: f64,
    pub time_spent: f64,
    pub origin_type: NodeType,
}
impl Insertion{
    pub fn new(pos: usize, origin_id:usize, route_id: usize, node_id: usize, cost:f64, t_chegada:f64, t_saida: f64, time_spent: f64, distance:f64, origin_type: NodeType) -> Insertion{
        Insertion { pos,
            route_id,
            origin_id,
            node_id, // satelite or depot id
            cost, // TODO: calc cost;
            t_saida,
            distance,
            t_chegada,
            time_spent,
            origin_type
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
