use crate::instance;

struct EvNode {
    remaining_battery: f64,
    tempo_chegada: f64,
    tempo_saida: f64,
    node_type: instance::NodeType,
    id: u64,

    // Considerando somente as proximas posicoes, armazena a de menor folga (tempo chegada - final janela)
    // int posMenorFolga   = -1;
}

pub struct EvRoute {
    satelite: Option<u32>,
    id: Option<u32>,
    max_size: Option<u32>,
    current_distance: f64,
    current_demand: f64,
    route: Vec<EvNode>
}
impl Default for EvRoute {
    fn default() -> Self {
        EvRoute { 
            satelite: None, 
            id: None,
            max_size: None,
            current_distance: 0.0,
            current_demand: 0.0,
            route: Vec::new()
        }
    }
}
impl EvRoute {
    pub fn new(satelite: u32, id: u32, max_size: u32) -> EvRoute {
        EvRoute{
            satelite: Some(satelite),
            id: Some(id),
            max_size: Some(max_size),
            ..Default::default()
        }
    }
    pub fn check_route(&self) -> bool {
        false
    }
}
