#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

#[derive(Eq, PartialEq, Debug)]
struct Node {
    reach: Reach,
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.reach.time.cmp(&self.reach.time)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Reach {
    station: String,
    time: Time,
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

fn dijkstra(start: Reach, goal: String, timetable: &TimeTable) -> Option<Node> {
    let mut heap = BinaryHeap::new();
    {
        let initial_node = Node { reach: start };
        heap.push(initial_node);
    }

    let mut visited = HashSet::new();
    while let Some(node) = heap.pop() {
        // the first access to the station
        visited.insert(node.reach.station.clone());

        if node.reach.station == goal { return Some(node); }

        // 1-hop reachable nodes
        for n in reachable_nodes(&node.reach, timetable) {
            if !visited.contains(&n.reach.station) {
                heap.push(n);
            }
        }
    }
    None
}

fn reachable_nodes(reach: &Reach, timetable: &TimeTable) -> Vec<Node> {
    // station を time 以降に出発する全ての便について, 1-hop で到達する station
    let mut nodes = vec![];
    if let Some(rows) = timetable.station(&reach.station) {
        for row in rows {
            if row.depart_time >= reach.time {
                nodes.push(Node { reach: Reach { station: row.station_to.clone(), time: row.arrive_time } });
            }
        }
    }
    nodes
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
    depart_time: Time,
    arrive_time: Time,
}

fn main() {
    let time = Time { hour: 8, min: 5 };
    let reach = Reach { station: "武蔵小金井".to_string(), time: time };
    let node = Node { reach: reach };
    let timetable = TimeTable::read_file();
    println!("{:?}", &timetable);
    let result = dijkstra(node.reach, "武蔵境".to_string(), &timetable);
    println!("{:?}", result);
}
