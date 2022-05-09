use std::fs;
#[derive(PartialEq, Eq, Debug)]
pub enum NodeType {
    Client,
    Sat,
    Station, // recharging station
    Depot,
}

#[derive(PartialEq, Debug)]
pub struct Node {
    pos: (f64, f64),
    demand: f64,
    start_time_window: f64,
    end_time_window: f64,
    service_time: f64,
    node_type: NodeType,
}

#[derive(PartialEq, Eq, Debug)]
enum VehicleType {
    Electric,
    Truck,
}

#[derive(PartialEq, Debug)]
pub struct Vehicle { // TODO: make fields private
    vehicle_type: VehicleType,
    capacity: f64,
    cost_per_distance: f64,
    vehicle_cost: f64,
    battery: Option<f64>,
    battery_per_distance: Option<f64>,
    recharging_rate: Option<f64>,
}

#[derive(PartialEq, Debug, Default)]
pub struct Instance {
    nodes: Vec<Node>,
    vehicles: Vec<Vehicle>,
    distance_matrix: Vec<Vec<f64>>, // matrix //
    n_evs: u32,
    n_trucks: u32,
    n_sats: u32,
    n_clients: u32,
    n_stations: u32,
    max_route_stations: u32, //3
    truck_capacity: f64,
    ev_capacity: f64,
    ev_battery: f64,

    // n_depots: i32,
}
/*
impl Default for Instance {
    fn default() -> Self {
        Instance { 
            nodes: (), 
            vehicles: (),
            distance_matrix: (),
            n_evs: (),
            n_trucks: (), 
            n_sats: (), 
            n_clients: (), 
            n_stations: (), 
            max_route_stations: (), 
            truck_capacity: (), 
            ev_capacity: (), 
            ev_battery: () 
        }
    }

} */
impl Instance {
    pub fn new(
        nodes: Vec<Node>,
        vehicles: Vec<Vehicle>,
        distance_matrix: Vec<Vec<f64>>, // matrix //
        n_evs: u32,
        n_trucks: u32,
        n_sats: u32,
        n_clients: u32,
        n_stations: u32,
        max_route_stations: u32, //3
        truck_capacity: f64,
        ev_capacity: f64,
        ev_battery: f64,
    ) -> Instance { 
        Instance{
            nodes, vehicles, distance_matrix, n_evs, 
            n_trucks, n_sats, n_clients, n_stations, 
            max_route_stations, truck_capacity, ev_capacity, ev_battery
        }
    }
    /* pub fn n_evs(&mut self, n_evs: u32) -> &mut Self{
        self.n_evs = n_evs;
        self
    } */
    pub fn get_client_range(&self) -> (u32, u32) {
        let start = 1 + self.n_sats + self.n_stations;
        (start, start + self.n_clients) 
    }
    pub fn get_sat_range(&self) -> (u32, u32) {
        let start = 1;
        (start, start + self.n_sats) 
    }
    pub fn get_station_range(&self) -> (u32, u32) {
        let start: u32 = 1 + self.n_sats;
        (start, start + self.n_stations) 
    }

    /* pub fn get_vehicle_capacity(&self, id:i32) -> f64 {
        let vehicle = self.vehicles.get(id);
        match vehicle {
            Some(veh) => veh.capacity,
            None      => panic!("Out of bounds! @ instance::get_truck_demand.")
        }
    }
    pub fn get_ev_battery(&self, id:i32) -> f64 {
        let vehicle = self.vehicles.get(id);
        match vehicle {
            Some(veh) => match veh.battery{
                Some(bat) => bat,
                None => panic("Not electric vehicle @ instance::get_ev_battery.")
            }
            None => panic!("Out of bounds! @ instance::get_truck_demand.")
        }
    } */
    pub fn get_ev_battery(&self) -> f64 {
        self.ev_battery
    }
    pub fn get_ev_capacity(&self) -> f64 {
        self.ev_capacity
    }
    pub fn get_truck_demand(&self) -> f64 {
        self.truck_capacity
    }
    pub fn get_n_sats(&self) -> u32 {
        self.n_sats
    }
    pub fn get_n_clients(&self) -> u32 {
        self.n_clients
    }
    pub fn get_n_stations(&self) -> u32 {
        self.n_stations
    }
    pub fn get_n_trucks(&self) -> u32 {
        self.n_trucks
    }
    pub fn get_n_evs(&self) -> u32 {
        self.n_evs
    }
    pub fn get_distance(&self, id1: u32, id2: u32) -> f64 {
        self.distance_matrix[id1 as usize][id2 as usize]
    }
    pub fn is_client(&self, node_id: u32) -> bool {
        self.nodes[node_id as usize].node_type == NodeType::Client
    }
    pub fn instance_from_file(filename: &str) -> Instance {
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        let lines = contents.lines();
        let mut lines_vec: Vec<String> = Vec::new();
        let mut num_matrix: Vec<Vec<f64>> = Vec::new();
        // stores file lines in a string Vec
        for line in lines{
            lines_vec.push((*line).to_string());
        }
        for line in lines_vec {
            let splited: Vec<&str> = line.split_whitespace().collect();
            let mut line_numbers: Vec<f64> = Vec::new();
            for num in splited {
                let parsed = num.parse::<f64>();
                match parsed {
                    Ok(o) => line_numbers.push(o),
                    Err(e) => panic!("Couldnt parse number @ create instance. Err: {}", e)
                }
            }
            num_matrix.push(line_numbers);
        }
        println!("{:?}", num_matrix);
        let n_trucks   = num_matrix[0][0] as usize;
        let n_evs      = num_matrix[0][1] as usize;
        let n_depots   = num_matrix[0][2] as usize;
        let n_sats     = num_matrix[0][3] as usize;
        let n_stations = num_matrix[0][4] as usize;
        let n_clients  = num_matrix[0][5] as usize;
        let n_nodes = n_clients + n_sats + n_depots + n_stations;
        let mut vehicles: Vec<Vehicle> = Vec::new();
        let mut nodes: Vec<Node> = Vec::new();
        let mut distance_matrix: Vec<Vec<f64>> = vec![vec![0.0; n_nodes]; n_nodes]; // cria vetor de vetores com todos elementos=0.0
        let mut truck_capacity = 0.0;
        let mut ev_capacity = 0.0;
        let mut ev_battery = 0.0;
        for i in 0..(n_trucks as usize){
            let capacity = num_matrix[i][0];
            let cost_per_distance = num_matrix[i][1];
            let cost = num_matrix[i][2]; // vehicle cost
            if i == 0 {
                truck_capacity = capacity;
            }
            let truck = Vehicle{
                vehicle_type: VehicleType::Truck,
                capacity,
                vehicle_cost: cost,
                cost_per_distance,
                battery: None,
                recharging_rate: None,
                battery_per_distance: None,
            };
            vehicles.push(truck);
        }
        for i in 0..(n_evs as usize){
            let idx = i + n_trucks as usize;
            let capacity = num_matrix[idx][0];
            let cost_per_distance = num_matrix[idx][1];
            let cost = num_matrix[idx][2]; // vehicle cost

            let battery = num_matrix[idx][3];
            let recharging_rate = num_matrix[idx][4];
            let battery_per_distance = num_matrix[idx][5];
            if i == 0{
                ev_battery = battery;
                ev_capacity = capacity;
            }
            let ev = Vehicle{
                vehicle_type: VehicleType::Electric,
                capacity,
                vehicle_cost: cost,
                cost_per_distance,
                battery: Some(battery),
                recharging_rate: Some(recharging_rate),
                battery_per_distance: Some(battery_per_distance),
            };
            vehicles.push(ev);
        }
        println!("vehicles: {:?}", vehicles);
        for i in 0..(n_nodes as usize){
            let node_type;
            if i < n_depots as usize {
                node_type = NodeType::Depot;
            }
            else if i < (n_depots + n_sats) as usize {
                node_type = NodeType::Sat;
            }
            else if i < (n_depots + n_sats + n_stations) as usize {
                node_type = NodeType::Station;
            }
            else if i < (n_depots + n_sats + n_stations + n_clients) as usize {
                node_type = NodeType::Client;
            }
            else{
                panic!("Node out of bounds")
            }
            let x = num_matrix[4+i][0];
            let y = num_matrix[4+i][1];
            let demand = num_matrix[4+i][2];
            let start_time_window = num_matrix[4+i][5];
            let end_time_window = num_matrix[4+i][6];
            let service_time = num_matrix[4+i][7];
            let node = Node { 
                pos:(x, y), 
                demand, 
                start_time_window, 
                end_time_window, 
                service_time, 
                node_type 
            };
            nodes.push(node);
        }
        for i in 0..nodes.len(){
            for j in 0..nodes.len(){
                distance_matrix[i][j] = distance(&nodes[i].pos, &nodes[j].pos);
            }
        }
        Instance::new(
            nodes,
            vehicles,
            distance_matrix,
            n_evs as u32,
            n_trucks as u32,
            n_sats as u32,
            n_clients as u32,
            n_stations as u32,
            3,
            truck_capacity,
            ev_capacity,
            ev_battery,
        )
    }
}
fn distance(p1:&(f64,f64), p2:&(f64,f64)) -> f64{
    f64::hypot(p1.0 - p2.0, p1.1 - p2.1)
}

