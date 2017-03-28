use std::cmp::Ordering;
use time::Time;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct RouteInfo {
    pub station_from: String,
    pub station_to: String,
    pub depart_time: Time,
    pub arrive_time: Time,
    pub line: String
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Node(Vec<RouteInfo>);

impl Node {
    pub fn new(route: RouteInfo) -> Node {
        Node(vec![route])
    }

    pub fn initialize(time: Time, station: &String) -> Node {
        Node::new(RouteInfo {
            station_from: String::new(),
            station_to: station.clone(),
            arrive_time: time,
            depart_time: time,
            line: String::new()
        })
    }

    pub fn station_to(&self) -> String {
        self.curr().station_to.clone()
    }

    pub fn arrive_time(&self) -> Time {
        self.curr().arrive_time
    }

    pub fn forward(&self, route: RouteInfo) -> Node {
        let mut node = self.clone();
        node.0.push(route);
        node
    }

    fn curr(&self) -> &RouteInfo {
        self.0.last().unwrap()
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

