use std::fs;
const TIME_PER_DISTANCE: f64 = 1.0;
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum NodeType {
    Client,
    Sat,
    Station, // recharging station
    Depot,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Node {
    pub pos: (f64, f64),
    pub demand: f64,
    pub start_time_window: f64,
    pub end_time_window: f64,
    pub service_time: f64,
    pub node_type: NodeType,
    pub node_id: usize,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum VehicleType {
    Electric,
    Truck,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vehicle { // TODO: make fields private
    pub vehicle_type: VehicleType,
    pub capacity: f64,
    pub cost_per_distance: f64,
    pub time_per_distance: f64,
    pub vehicle_cost: f64,
    pub battery: Option<f64>,
    pub battery_per_distance: Option<f64>,
    pub recharging_rate: Option<f64>,
    pub vehicle_id: usize,
}

#[derive(PartialEq, Debug, Default)]
pub struct Instance {
    nodes: Vec<Node>,
    vehicles: Vec<Vehicle>,
    distance_matrix: Vec<Vec<f64>>, // matrix //
    n_evs: usize,
    n_trucks: usize,
    n_sats: usize,
    n_clients: usize,
    n_stations: usize,
    max_route_stations: usize, //3
    truck_capacity: f64,
    ev_capacity: f64,
    ev_battery: f64,
    best_stations_to_insert: Vec<Vec<(usize,f64)>>, // matrix //

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
        n_evs: usize,
        n_trucks: usize,
        n_sats: usize,
        n_clients: usize,
        n_stations: usize,
        max_route_stations: usize, //3
        truck_capacity: f64,
        ev_capacity: f64,
        ev_battery: f64,
    ) -> Instance { 
        let mut inst = Instance{
            nodes, vehicles, distance_matrix, n_evs, 
            n_trucks, n_sats, n_clients, n_stations, 
            max_route_stations, truck_capacity, ev_capacity, ev_battery, best_stations_to_insert: Vec::new()
        };
        let mut vec :Vec<Vec<(usize, f64)>>= Vec::new();
        // let mut vec = Vec::with_capacity(5);
        // vec.resize(5, 0);
        for i in 0..inst.get_n_nodes() {
            let mut aux = Vec::with_capacity(inst.get_n_nodes());
            aux.resize(inst.get_n_nodes(), (999, 1e6)); // USE OPTION INSTEAD OF 999 1e6
            vec.push(aux);
        }
        for (i,node0) in inst.get_nodes().iter().enumerate(){
            for (j,node1) in inst.get_nodes().iter().enumerate(){
                if i != j {
                    let mut best_station_id = 999;
                    let mut best_distance = 1e6;
                    for (s, station) in inst.get_stations().iter().enumerate(){
                        if station.node_id != node0.node_id && station.node_id != node1.node_id{
                            let distance = inst.get_insert_distance(station.node_id, node0.node_id, node1.node_id);
                            if distance < best_distance{
                                best_distance = distance;
                                best_station_id = station.node_id;
                            }
                        }
                    }
                    vec[i][j] = (best_station_id, best_distance);
                }
            }
        }
        inst.best_stations_to_insert = vec;
        inst
    }
    // get ranges
    pub fn get_clients(&self) -> &[Node]{
        let client_range = self.get_client_range();
        &self.nodes[client_range.0 .. client_range.1]
    }
    pub fn get_best_station_to_insert_between(&self, a: usize, b: usize) -> (usize, f64){
        self.best_stations_to_insert[a][b]
    }
    pub fn get_nodes(&self) -> &[Node]{
        &self.nodes
    }
    pub fn get_sats(&self) -> &[Node]{
        let sat_range = self.get_sat_range();
        &self.nodes[sat_range.0 .. sat_range.1]
    }
    pub fn get_stations(&self) -> &[Node]{
        let station_range = self.get_station_range();
        &self.nodes[station_range.0 .. station_range.1]
    }
    pub fn get_depots(&self) -> &[Node]{
        let depot_range = self.get_client_range();
        &self.nodes[depot_range.0 .. depot_range.1]
    }
    pub fn get_depot(&self) -> &Node{
        &self.nodes[0]
    }
    pub fn get_vehicle(&self, index: usize) -> &Vehicle{
        &self.vehicles[index]
    }
    pub fn get_first_vehicle_distance_cost(&self) -> f64{
        self.vehicles[0].cost_per_distance
    }

    pub fn get_node_demand(&self, node_id: usize) -> f64 {
        self.nodes[node_id as usize].demand
    }
    pub fn get_node(&self, node_id: usize) -> &Node {
        &self.nodes[node_id]
    }
    /* pub fn n_evs(&mut self, n_evs: usize) -> &mut Self{
        self.n_evs = n_evs;
        self
    } */
    fn get_client_range(&self) -> (usize, usize) {
        let start = 1 + self.n_sats + self.n_stations;
        (start, start + self.n_clients) 
    }
    fn get_sat_range(&self) -> (usize, usize) {
        let start = 1;
        (start, start + self.n_sats) 
    }
    fn get_station_range(&self) -> (usize, usize) {
        let start: usize = 1 + self.n_sats;
        (start, start + self.n_stations) 
    }
    fn get_depot_range(&self) -> (usize, usize) {
        // let start: usize = 1 + self.n_sats;
        (0, 1) 
    }
    pub fn get_node_type(&self, node_id: usize) -> NodeType{
        self.get_node(node_id).node_type
    }
    pub fn get_insert_distance(&self, inserted: usize, a:usize, b:usize) -> f64{
        self.get_distance(a, inserted) + self.get_distance(inserted, b) - self.get_distance(a, b)
    }
    pub fn get_insert_time(&self, inserted: usize, a:usize, b:usize, vehicle_id: usize) -> f64{
        let vehicle = &self.get_vehicle(vehicle_id);
        vehicle.time_per_distance*(self.get_distance(a, inserted) + self.get_distance(inserted, b) - self.get_distance(a, b)) 
            + self.nodes[inserted].service_time
    }
    pub fn get_ev_battery(&self) -> f64 {
        self.ev_battery
    }
    pub fn get_ev_capacity(&self) -> f64 {
        self.ev_capacity
    }
    pub fn get_truck_demand(&self) -> f64 {
        self.truck_capacity
    }
    pub fn get_n_sats(&self) -> usize {
        self.n_sats
    }
    pub fn get_n_clients(&self) -> usize {
        self.n_clients
    }
    pub fn get_n_stations(&self) -> usize {
        self.n_stations
    }
    pub fn get_n_trucks(&self) -> usize {
        self.n_trucks
    }
    pub fn get_n_evs(&self) -> usize {
        self.n_evs
    }
    pub fn get_distance(&self, id1: usize, id2: usize) -> f64 {
        self.distance_matrix[id1 as usize][id2 as usize]
    }
    pub fn is_client(&self, node_id: usize) -> bool {
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
            let i_plus_1 = i +1;
            let capacity = num_matrix[i_plus_1][0];
            let cost_per_distance = num_matrix[i_plus_1][1];
            let cost = num_matrix[i_plus_1][2]; // vehicle cost
            if i == 0 {
                truck_capacity = capacity;
            }
            let truck = Vehicle{
                vehicle_type: VehicleType::Truck,
                capacity,
                vehicle_cost: cost,
                cost_per_distance,
                time_per_distance: TIME_PER_DISTANCE,
                battery: None,
                recharging_rate: None,
                battery_per_distance: None,
                vehicle_id: i
            };
            vehicles.push(truck);
        }
        for i in 0..(n_evs as usize){
            let idx = i + n_trucks+1 as usize;
            let capacity = num_matrix[idx][0];
            let cost_per_distance = num_matrix[idx][1];
            let cost = num_matrix[idx][2]; // vehicle cost

            let battery = num_matrix[idx][3];
            // let battery = 1000f64;
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
                time_per_distance: TIME_PER_DISTANCE,
                battery: Some(battery),
                recharging_rate: Some(recharging_rate),
                battery_per_distance: Some(battery_per_distance),
                vehicle_id: i + vehicles.len(),
            };
            vehicles.push(ev);
        }
        println!("vehicles: {:?}", vehicles);

        // let first_node_index = 4;
        let first_node_index = n_evs + n_trucks + 1;
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
            println!("{} {}",n_nodes, i);
            let x = num_matrix[first_node_index+i][0];
            let y = num_matrix[first_node_index+i][1];
            let demand = num_matrix[first_node_index+i][2];
            let start_time_window = num_matrix[first_node_index+i][5];
            let end_time_window = num_matrix[first_node_index+i][6];
            let service_time = num_matrix[first_node_index+i][7];
            let node_id = i;
            let node = Node { 
                pos:(x, y), 
                demand, 
                start_time_window, 
                end_time_window, 
                service_time, 
                node_type,
                node_id,
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
            n_evs,
            n_trucks,
            n_sats,
            n_clients,
            n_stations,
            3,
            truck_capacity,
            ev_capacity,
            ev_battery,
        )
    }

    fn get_n_nodes(&self) -> usize {
        self.nodes.len()
    }
}
fn distance(p1:&(f64,f64), p2:&(f64,f64)) -> f64{
    f64::hypot(p1.0 - p2.0, p1.1 - p2.1)
}

