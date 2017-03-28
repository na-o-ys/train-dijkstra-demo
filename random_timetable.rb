stations = 10000

schedule = {}

def timeformat(hour, min)
  hour += min / 60
  min %= 60
  "#{hour.to_s.rjust(2, '0')}#{min.to_s.rjust(2, '0')}"
end

def gen_schedule(stations, line, interval, speed, schedule)
  (4..24).each do |hour|
    0.step(59, interval) do |min|
      stations.each_cons(2).each_with_index do |station, i|
        schedule[station[0]] ||= []
        schedule[station[0]] << {
          station_to: station[1],
          line: line,
          depart_time: timeformat(hour, min + i * speed),
          arrive_time: timeformat(hour, min + (i + 1) * speed)
        }
      end
    end
  end
end

(4..24).each do |hour|
  0.step(59, 5).each do |min|
    (0..stations).each do |i|
      schedule["S#{i}"] <<= []
      schedule["S#{i}"] << {
        station_to: "S#{rand(stations)}",
        line: "random",
        depart_time: timeformat(hour, min),
        arrive_time: timeformat(hour, min + 3)
      }
    end
  end
end

require 'json'
print schedule.map { |station, rows| { station: station, rows: rows } }.to_json
