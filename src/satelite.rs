use crate::ev_route::EvRoute;
use crate::instance::{Vehicle, Instance};

#[derive(Debug)]
pub struct Satelite{
    pub routes: Vec<EvRoute>,
    pub demand: f64,
    pub tempo_de_chegada: Option<f64>,
    pub sat_id: usize,
}
impl Satelite{
    pub fn get_demand(&self) -> f64 {
        self.demand
    }
    pub fn check_satelite(&self) -> bool {
        false
    }
    pub fn new(sat_id: usize) -> Satelite {
        Satelite {
            routes: Vec::new(),
            demand: 0.0,
            tempo_de_chegada: None,
            sat_id,
        }
    }
    /*
    pub fn add_empty_route(&self){
        self.routes.push(EvRoute::default())
    }
    */
    pub fn get_ev_routes(&self) -> &Vec<EvRoute>{
        &self.routes
    }
    pub fn set_tempo_de_chegada(&mut self, t:f64){
        self.tempo_de_chegada = Some(t);
    }
    pub fn add_vehicle(&mut self, vehicle:&Vehicle, instance:&Instance){
        let route = EvRoute::new(instance, self.sat_id, vehicle, self.tempo_de_chegada.unwrap()); //TODO: dont use unwrap here
        self.routes.push(route);
    }
}
