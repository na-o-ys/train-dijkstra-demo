#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let timetable = TimeTable::read_file();
    println!("{:?}", &timetable);
    let result = dijkstra(Time { hour: 8, min: 5 }, "武蔵小金井".to_string(), "武蔵境".to_string(), &timetable);
    println!("{:?}", result);
}

fn dijkstra(time: Time, start: String, goal: String, timetable: &TimeTable) -> Option<Node> {
    let mut heap = BinaryHeap::new();
    heap.push(Node::initialize(time, &start));

    let mut visited = HashSet::new();
    while let Some(node) = heap.pop() {
        // the first access to the station
        visited.insert(node.station_to());

        if node.station_to() == goal { return Some(node); }

        // 1-hop reachable nodes
        for n in reachable_nodes(&node, timetable) {
            if !visited.contains(&n.station_to()) {
                heap.push(n);
            }
        }
    }
    None
}

// station を time 以降に出発する全ての便について, 1-hop で到達する station
fn reachable_nodes(node: &Node, timetable: &TimeTable) -> Vec<Node> {
    let mut nodes = vec![];
    if let Some(rows) = timetable.station(&node.station_to()) {
        for row in rows {
            if row.depart_time >= node.arrive_time() {
                nodes.push(node.forward(RouteInfo {
                    station_to: row.station_to.clone(),
                    station_from: node.station_to(),
                    depart_time: row.depart_time,
                    arrive_time: row.arrive_time,
                    line: row.line.clone()
                }));
            }
        }
    }
    nodes
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node(Vec<RouteInfo>);

impl Node {
    fn new(route: RouteInfo) -> Node {
        Node(vec![route])
    }

    fn initialize(time: Time, station: &String) -> Node {
        Node::new(RouteInfo {
            station_from: String::new(),
            station_to: station.clone(),
            arrive_time: time,
            depart_time: time,
            line: String::new()
        })
    }

    fn curr(&self) -> &RouteInfo {
        self.0.last().unwrap()
    }

    fn station_to(&self) -> String {
        self.curr().station_to.clone()
    }

    fn arrive_time(&self) -> Time {
        self.curr().arrive_time
    }

    fn forward(&self, route: RouteInfo) -> Node {
        let mut node = self.clone();
        node.0.push(route);
        node
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.arrive_time().cmp(&self.arrive_time())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct RouteInfo {
    station_from: String,
    station_to: String,
    depart_time: Time,
    arrive_time: Time,
    line: String
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Time {
    hour: u32,
    min: u32,
}

impl Ord for Time {
    fn cmp(&self, other: &Time) -> Ordering {
        let selfmins = &self.hour * 60 + &self.min;
        let othermins = other.hour * 60 + other.min;
        selfmins.cmp(&othermins)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct TimeTable {
    stations: HashMap<String, StationTimeTable>
}

#[derive(Debug)]
struct StationTimeTable {
    station: String,
    rows: Vec<TimeTableRow>
}

#[derive(Deserialize)]
struct StationTimeTableJson {
    station: String,
    rows: Vec<TimeTableRowJson>
}

impl StationTimeTableJson {
    fn to_station_timetable(&self) -> StationTimeTable {
        StationTimeTable {
            station: self.station.clone(),
            rows: self.rows.iter().map(|row| row.to_timetable_row()).collect()
        }
    }
}

#[derive(Deserialize)]
struct TimeTableRowJson {
    station_to: String,
    line: String,
    depart_time: String,
    arrive_time: String
}

impl TimeTableRowJson {
    fn to_timetable_row(&self) -> TimeTableRow {
        fn timestr_to_time(timestr: &String) -> Time {
            let v = timestr.parse::<u32>().unwrap();
            Time { hour: v / 100, min: v % 100 }
        }
        TimeTableRow {
            station_to: self.station_to.clone(),
            line: self.line.clone(),
            depart_time: timestr_to_time(&self.depart_time),
            arrive_time: timestr_to_time(&self.arrive_time)
        }
    }
}

impl TimeTable {
    fn read_file() -> TimeTable {
        let file = File::open("timetable.json").unwrap();
        let reader = BufReader::new(file);
        let station_jsons: Vec<StationTimeTableJson> = serde_json::from_reader(reader).unwrap();
        let mut stations = HashMap::new();
        for station_json in station_jsons {
            let station_timetable = station_json.to_station_timetable();
            stations.insert(station_timetable.station.clone(), station_timetable);

        }
        TimeTable { stations: stations }
    }

    fn station(&self, station: &String) -> Option<&Vec<TimeTableRow>> {
        self.stations.get(station).map(|c| &c.rows)
    }
}

#[derive(Debug)]
struct TimeTableRow {
    station_to: String,
    line: String,
    depart_time: Time,
    arrive_time: Time,
}
