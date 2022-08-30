use crate::aux_structures::{SingleRouteInsertion, Insertion, RechargeInsertion};
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

    pub fn can_insert_in_single_route(&self, instance: &Instance, client_id: usize) -> Option<SingleRouteInsertion> {
        let dummy_vehicle = instance.get_vehicle(1);
        let vehicle_battery = dummy_vehicle.battery.unwrap();
        let vehicle_battery_distance = dummy_vehicle.battery_per_distance.unwrap();
        let client_info = instance.get_node(client_id);
        let demand = client_info.demand;
        let total_distance = instance.get_insert_distance(client_id, self.sat_id, self.sat_id);
        let distance_to_client = instance.get_distance(self.sat_id, client_id);
        let t_chegada_client = self.tempo_de_chegada.unwrap() + distance_to_client;
        let cost = total_distance;
        if t_chegada_client > client_info.end_time_window{
            println!("time window broke");
            return None;
        }
        else if vehicle_battery < total_distance*vehicle_battery_distance {
            println!("battery broke");
            let best_station = instance.get_best_station_to_insert_between(self.sat_id, client_id);
            let distance_to_station = instance.get_distance(self.sat_id, best_station.0);
            let distance_station_client = instance.get_distance(best_station.0, client_id);
            let battery_at_station = vehicle_battery - distance_to_station;
            let recharge_time = distance_to_station/dummy_vehicle.recharging_rate.unwrap();
            let t_chegada_client = self.tempo_de_chegada.unwrap() + distance_to_station + distance_station_client + recharge_time;
            if t_chegada_client > client_info.end_time_window{
                return None;
            }
            else{
                let rechargeInsertion = RechargeInsertion::new(best_station.0, 1, 0.0, 0.0, 0.0, 0.0, 0.0);
                let insertion = SingleRouteInsertion { 
                    node_id: client_id,
                    origin_id: self.sat_id,
                    cost,
                    origin_type: crate::instance::NodeType::Sat,
                    recharge_before: Some(rechargeInsertion),
                    recharge_after: None,
                };
            println!("done!");
            Some(insertion)
            }
        }
        else if dummy_vehicle.capacity < demand{
            println!("capacity broke");
            return None;
        }
        else{
            let insertion = SingleRouteInsertion { 
                node_id: client_id,
                origin_id: self.sat_id,
                cost,
                origin_type: crate::instance::NodeType::Sat,
                recharge_before: None,
                recharge_after: None,
            };
            println!("done!");
            Some(insertion)
        }
    }
    pub fn insert_single_route(&mut self, instance: &Instance, single_insertion: &SingleRouteInsertion){
        self.add_vehicle(instance.get_vehicle(1), instance); //TODO: pensar numa maneira de nao
        let length = self.routes.len();
        let ev_route = &mut self.routes[length-1];
        let ins = *single_insertion;
        let insertion: Insertion = Insertion::new(1, self.sat_id, 0, single_insertion.node_id, single_insertion.cost, 0.0, 0.0, 0.0, 0.0, crate::instance::NodeType::Sat, single_insertion.recharge_before);
        ev_route.insert(instance, &insertion);
    }
}
