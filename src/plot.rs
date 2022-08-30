use std::cmp::min;
use std::io;
use gnuplot::PlotOption::{ArrowSize, LineWidth};
use gnuplot::{Figure, Caption, Color, PointSymbol, PlotOption::{PointSize, ArrowType, LineStyle}, AxesCommon, Coordinate, AutoOption, LabelOption::{TextColor, TextAlign, Font, TextOffset}, AlignType, ArrowheadType, DashType};

use crate::{solution::Solution, instance::Instance, aux_structures::RouteNode};
pub fn plot_solution(instance: &Instance, solution: &Solution, filename: &str) -> Option<Figure> {
    let mut fg = Figure::new();
    let depot = instance.get_depot();
    let clients = instance.get_clients();
    let satelites = instance.get_sats();
    let stations = instance.get_stations();
    let axes = fg.axes2d()
        .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(200.0))
        .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(200.0))
        .set_aspect_ratio(AutoOption::Fix(1.0))
        .points([depot.pos.0], [depot.pos.1], &[Caption("Depot"), Color("red"), PointSize(3.0), PointSymbol('S')])
        .points(satelites.iter().map(|sat| sat.pos.0).collect::<Vec<_>>(), satelites.iter().map(|sat| sat.pos.1).collect::<Vec<_>>(), &[Caption("Satelites"), Color("green"), PointSize(3.0), PointSymbol('R')])
        .points(clients.iter().map(|sat| sat.pos.0).collect::<Vec<_>>(), clients.iter().map(|sat| sat.pos.1).collect::<Vec<_>>(), &[Caption("clients"), Color("black"), PointSize(3.0), PointSymbol('O')])
        .points(stations.iter().map(|sat| sat.pos.0).collect::<Vec<_>>(), stations.iter().map(|sat| sat.pos.1).collect::<Vec<_>>(), &[Caption("Recharging Stations"), Color("blue"), PointSize(3.0), PointSymbol('T')])
        // .points(x, y, &[Caption("A line"), Color("black"), PointSymbol('O'), PointSize(5.0)])
        // .lines(&x, &y, &[Caption("A line"), Color("black")]);
        ;
    for client in instance.get_clients(){
        let (x, y) = client.pos;
        axes.label(client.node_id.to_string().as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("white"), TextAlign(AlignType::AlignCenter), Font("Arial Bold", 10.0)]);
        axes.label(format!("𐄷{}", client.demand).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextOffset(0.0, -1.0)]);
        axes.label(format!("⏲{}..{} +{}", client.start_time_window, client.end_time_window, client.service_time).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("black"), Font("Arial Bold", 10.0), TextOffset((0.0), (-2.0))]);
    }
    for sat in instance.get_sats(){
        let (x, y) = sat.pos;
        axes.label(sat.node_id.to_string().as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("black"), TextAlign(AlignType::AlignCenter), Font("Arial Bold", 10.0)]);
        // axes.label(format!("𐄷{}", sat.demand).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextOffset(0.0, -1.0)]);
        // axes.label(format!("⏲{}..{} +{}", sat.start_time_window, sat.end_time_window, sat.service_time).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("black"), Font("Arial Bold", 10.0), TextOffset((0.0), (-2.0))]);
    }
    for sat in instance.get_sats(){
        let (sat_x, sat_y) = sat.pos;
        let (depot_x, depot_y) = depot.pos;
        axes.arrow(Coordinate::Axis(sat_x), Coordinate::Axis(sat_y), Coordinate::Axis(depot_x), Coordinate::Axis(depot_y), &[ArrowType(ArrowheadType::NoArrow), LineStyle(DashType::SmallDot), Color("gray")]);
        for client in instance.get_clients(){
            let (client_x, client_y) = client.pos;
            axes.arrow(Coordinate::Axis(sat_x), Coordinate::Axis(sat_y), Coordinate::Axis(client_x), Coordinate::Axis(client_y), &[ArrowType(ArrowheadType::NoArrow), LineStyle(DashType::DotDash), Color("gray")]);
        }
    }
    for (truck_index, truck_route) in solution.get_truck_routes().iter().enumerate(){
        let mut last_node= &truck_route.get_nodes()[0];
        for (node_index, node) in truck_route.get_nodes().iter().enumerate(){
            if node_index != 0{
                let node_info = instance.get_node(node.node_id);
                let last_node_info = instance.get_node(last_node.node_id);
                axes.arrow(Coordinate::Axis(node_info.pos.0), Coordinate::Axis(node_info.pos.1),
                           Coordinate::Axis(last_node_info.pos.0), Coordinate::Axis(last_node_info.pos.1), 
                           &[ArrowSize(0.02), ArrowType(ArrowheadType::Filled), 
                           Color(format!("#f{}{}1{}1", min(2*truck_index as usize, 9), min(0, solution.get_truck_routes().len() - truck_index as usize), "f").as_str()),
                           LineStyle(DashType::DotDash),
                           LineWidth(1.5)]);
            }
            last_node = node;
        }
    }
    for sat in solution.get_satelites(){
        let sat_info = instance.get_node(sat.sat_id);
        let (sat_x, sat_y) = sat_info.pos;
        for (ev_index, route) in sat.get_ev_routes().iter().enumerate(){
            let mut last_node= &route.get_nodes()[0];
            for (node_index, node) in route.get_nodes().iter().enumerate(){
                if node_index != 0{
                    let node_info = instance.get_node(node.node_id);
                    let last_node_info = instance.get_node(last_node.node_id);
                    axes.arrow(Coordinate::Axis(node_info.pos.0), Coordinate::Axis(node_info.pos.1),
                               Coordinate::Axis(last_node_info.pos.0), Coordinate::Axis(last_node_info.pos.1), 
                               &[ArrowSize(0.02), ArrowType(ArrowheadType::Filled), Color(format!("#{}f{}1{}1", min(2*ev_index as usize, 9), min(0, sat.routes.len() - ev_index as usize), "f").as_str())]);
                    let media_x = (node_info.pos.0 + last_node_info.pos.0)/2.0;
                    let media_y = (node_info.pos.1 + last_node_info.pos.1)/2.0;
                    let distance = instance.get_distance(node_info.node_id, last_node_info.node_id);
                    axes.label(format!("{:.2}", distance).as_str(), Coordinate::Axis(media_x), Coordinate::Axis(media_y), &[TextColor("red"), TextAlign(AlignType::AlignCenter), Font("Arial Bold", 10.0)]);
                    let client = instance.get_node(node.node_id);
                    // axes.label(format!("⏲{}..{} +{}", client.start_time_window, client.end_time_window, client.service_time).as_str(), Coordinate::Axis(client.pos.0), Coordinate::Axis(client.pos.1), &[TextColor("black"), Font("Arial Bold", 10.0), TextOffset(0.0, -3.0)]);
                }
                last_node = node;
            }

        }
    }
    fg.show().unwrap();
    match fg.save_to_png(filename, 800, 800) {
        Ok(it) => it,
        Err(_err) => return None,
    }; 

    /* .points([depot.pos.0], [depot.pos.1], &[Caption("Depot"), Color("red"), PointSize(3.0), PointSymbol('S')])
    .points(satelites.iter().map(|sat| sat.pos.0).collect::<Vec<_>>(), satelites.iter().map(|sat| sat.pos.1).collect::<Vec<_>>(), &[Caption("Satelites"), Color("green"), PointSize(3.0), PointSymbol('R')])
    .points(clients.iter().map(|sat| sat.pos.0).collect::<Vec<_>>(), clients.iter().map(|sat| sat.pos.1).collect::<Vec<_>>(), &[Caption("clients"), Color("black"), PointSize(3.0), PointSymbol('O')]) */
    Some(fg)
}
pub fn plot_instance(instance: &Instance, display: bool) -> io::Result<Figure> {
    let x = [0u32, 1, 2];
    let y = [3u32, 4, 5];
    let mut fg = Figure::new();
    let depot = instance.get_depot();
    let clients = instance.get_clients();
    let satelites = instance.get_sats();
    let stations = instance.get_stations();
    // println!("stations: {:?}", stations);
    let axes = fg.axes2d()
        .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(200.0))
        .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(200.0))
        .set_aspect_ratio(AutoOption::Fix(1.0))
        .points([depot.pos.0], [depot.pos.1], &[Caption("Depot"), Color("red"), PointSize(3.0), PointSymbol('S')])
        .points(satelites.iter().map(|sat| sat.pos.0).collect::<Vec<_>>(), satelites.iter().map(|sat| sat.pos.1).collect::<Vec<_>>(), &[Caption("Satelites"), Color("green"), PointSize(3.0), PointSymbol('R')])
        .points(clients.iter().map(|sat| sat.pos.0).collect::<Vec<_>>(), clients.iter().map(|sat| sat.pos.1).collect::<Vec<_>>(), &[Caption("clients"), Color("black"), PointSize(3.0), PointSymbol('O')])
        .points(stations.iter().map(|sat| sat.pos.0).collect::<Vec<_>>(), stations.iter().map(|sat| sat.pos.1).collect::<Vec<_>>(), &[Caption("Recharging Stations"), Color("blue"), PointSize(3.0), PointSymbol('O')])
        // .points(x, y, &[Caption("A line"), Color("black"), PointSymbol('O'), PointSize(5.0)])
        // .lines(&x, &y, &[Caption("A line"), Color("black")]);
        ;
    for client in instance.get_clients(){
        let (x, y) = client.pos;
        axes.label(client.node_id.to_string().as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("white"), TextAlign(AlignType::AlignCenter), Font("Arial Bold", 10.0)]);
        axes.label(format!("𐄷{}", client.demand).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextOffset(0.0, -1.0)]);
        axes.label(format!("⏲{}..{} +{}", client.start_time_window, client.end_time_window, client.service_time).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("black"), Font("Arial Bold", 10.0), TextOffset((0.0), (-2.0))]);
    }
    for sat in instance.get_sats(){
        let (x, y) = sat.pos;
        axes.label(sat.node_id.to_string().as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("black"), TextAlign(AlignType::AlignCenter), Font("Arial Bold", 10.0)]);
        // axes.label(format!("𐄷{}", sat.demand).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextOffset(0.0, -1.0)]);
        // axes.label(format!("⏲{}..{} +{}", sat.start_time_window, sat.end_time_window, sat.service_time).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("black"), Font("Arial Bold", 10.0), TextOffset((0.0), (-2.0))]);
    }
    for station in instance.get_stations(){
        let (x, y) = station.pos;
        axes.label(station.node_id.to_string().as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("black"), TextAlign(AlignType::AlignCenter), Font("Arial Bold", 10.0)]);
        // axes.label(format!("𐄷{}", sat.demand).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextOffset(0.0, -1.0)]);
        // axes.label(format!("⏲{}..{} +{}", sat.start_time_window, sat.end_time_window, sat.service_time).as_str(), Coordinate::Axis(x), Coordinate::Axis(y), &[TextColor("black"), Font("Arial Bold", 10.0), TextOffset((0.0), (-2.0))]);
    }
    for sat in instance.get_sats(){
        let (sat_x, sat_y) = sat.pos;
        let (depot_x, depot_y) = depot.pos;
        axes.arrow(Coordinate::Axis(sat_x), Coordinate::Axis(sat_y), Coordinate::Axis(depot_x), Coordinate::Axis(depot_y), &[ArrowType(ArrowheadType::NoArrow), LineStyle(DashType::SmallDot), Color("gray")]);
        for client in instance.get_clients(){
            let (client_x, client_y) = client.pos;
            axes.arrow(Coordinate::Axis(sat_x), Coordinate::Axis(sat_y), Coordinate::Axis(client_x), Coordinate::Axis(client_y), &[ArrowType(ArrowheadType::NoArrow), LineStyle(DashType::DotDash), Color("gray")]);

        }
    }
    if display{
        fg.show().unwrap();
    }

    Ok(fg)
}
