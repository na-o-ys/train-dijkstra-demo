/// 時刻表 json の serialize / deserialize を扱う
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::convert::AsRef;
use std::path::Path;
use std::io::BufReader;
use time::Time;

#[derive(Debug)]
pub struct TimeTable {
    stations: HashMap<String, StationTimeTable>
}

impl TimeTable {
    pub fn read_file<P: AsRef<Path>>(path: P) -> TimeTable {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let station_jsons: Vec<json::StationTimeTableJson> = serde_json::from_reader(reader).unwrap();
        let mut stations = HashMap::new();
        for station_json in station_jsons {
            let station_timetable = station_json.to_station_timetable();
            stations.insert(station_timetable.station.clone(), station_timetable);

        }
        TimeTable { stations: stations }
    }

    pub fn station(&self, station: &String) -> Option<&Vec<TimeTableRow>> {
        self.stations.get(station).map(|c| &c.rows)
    }
}

#[derive(Debug)]
pub struct StationTimeTable {
    station: String,
    rows: Vec<TimeTableRow>
}

#[derive(Debug)]
pub struct TimeTableRow {
    pub station_to: String,
    pub line: String,
    pub depart_time: Time,
    pub arrive_time: Time,
}

mod json {
    use super::StationTimeTable;
    use super::TimeTableRow;

    #[derive(Deserialize)]
    pub struct StationTimeTableJson {
        station: String,
        rows: Vec<TimeTableRowJson>
    }

    impl StationTimeTableJson {
        pub fn to_station_timetable(&self) -> StationTimeTable {
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
            TimeTableRow {
                station_to: self.station_to.clone(),
                line: self.line.clone(),
                depart_time: self.depart_time.parse().unwrap(),
                arrive_time: self.arrive_time.parse().unwrap()
            }
        }
    }
}
