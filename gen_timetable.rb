中央線快速（上り） = %w(立川 国立 西国分寺 国分寺 武蔵小金井 東小金井 武蔵境 三鷹 吉祥寺 西荻窪 荻窪 阿佐ヶ谷 高円寺 中野 新宿)
中央特快（上り）   = %w(立川 国分寺 三鷹 中野 新宿)
東西線             = %w(中野 落合 高田馬場)
山手線（内回り）   = %w(高田馬場 新大久保 新宿 代々木 原宿 渋谷 恵比寿 目黒)

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

gen_schedule(中央線快速（上り）, "中央線快速（上り）", 10, 3, schedule)
gen_schedule(中央特快（上り）, "中央特快（上り）", 25, 6, schedule)
gen_schedule(東西線, "東西線", 10, 5, schedule)
gen_schedule(山手線（内回り）, "山手線（内回り）", 5, 3, schedule)

require 'json'
print schedule.map { |station, rows| { station: station, rows: rows } }.to_json
