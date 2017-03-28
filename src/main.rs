#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod node;
mod time;
mod timetable;

use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::io::stdin;
use node::Node;
use node::RouteInfo;
use time::Time;
use timetable::TimeTable;

fn main() {
    let timetable = TimeTable::read_file("timetable.json");

    println!("\n出発駅:");
    let start = read_line();
    println!("\n到着駅:");
    let goal = read_line();
    println!("\n出発時刻:");
    let time = read_line().parse().unwrap();

    let result = dijkstra(time, start, goal, &timetable);
    if let Some(node) = result {
        print_result(&node);
    }
}

/// start 駅を time 以降に出発して、goal 駅に到達する経路をダイクストラ法で求める
fn dijkstra(time: Time, start: String, goal: String, timetable: &TimeTable) -> Option<Node> {
    let mut heap = BinaryHeap::new();
    // 初期ノード (出発時刻, 出発駅) を追加
    heap.push(Node::initialize(time, &start));

    // 到達済みフラグ
    let mut visited = HashSet::new();
    while let Some(node) = heap.pop() {
        // 到達済みフラグを立てる
        visited.insert(node.station_to());

        // ゴール駅に到達
        if node.station_to() == goal { return Some(node); }

        // 1-hop で到達可能なノードを列挙
        for next_node in reachable_nodes(&node, timetable) {
            if !visited.contains(&next_node.station_to()) {
                heap.push(next_node);
            }
        }
    }
    None
}

/// station を time 以降に出発して 1-hop で到達可能な駅を列挙する
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

fn print_result(result: &Node) {
    for route in result.vec() {
        if !route.station_from.is_empty() {
            println!("{} {}\t{}", route.depart_time, &route.station_from, &route.line);
        }
    }
    let last = result.vec().last().unwrap();
    println!("{} {}", last.arrive_time, &last.station_to);
}

fn read_line() -> String {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

