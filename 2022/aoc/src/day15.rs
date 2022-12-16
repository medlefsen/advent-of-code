use crate::parsing::ParseFile;

#[derive(Parser)]
#[grammar="src/day15.pest"]
struct InputParser;

#[derive(Clone, Debug)]
struct Sensor {
    x: i32,
    y: i32,
    radius: i32,
}

impl Sensor {
    fn new(pos: (i32, i32), nearest_beacon: (i32, i32)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            radius: dist(pos, nearest_beacon),
        }
    }
    fn dist(&self, x: i32, y:i32) -> i32 {
        dist((self.x, self.y), (x,y))
    }
    fn in_radius(&self, x: i32, y:i32) -> bool {
        self.dist(x, y) <= self.radius
    }
}
fn dist(from: (i32,i32), to: (i32,i32)) -> i32 {
    (from.0.abs_diff(to.0) + from.1.abs_diff(to.1)) as i32
}

fn parse_input() -> Vec<((i32,i32),(i32,i32))> {
    let (input,) = InputParser::parse_file(Rule::input, "inputs/day15/input.txt");
    return input;
}
const MAX_SIZE : i32 = 4000000;
fn find_beacon(sensors: &Vec<Sensor>) -> (i32,i32) {
    for x in 0..=MAX_SIZE {
        let mut y = 0;
        while y <= MAX_SIZE {
            if let Some(s) = sensors.iter().find(|s| { s.in_radius(x,y) }) {
                let skip = s.y + s.radius - (s.x.abs_diff(x) as i32) + 1;
                y = skip;
            } else {
                return (x,y);
            }
        }
    }
    unreachable!();
}

fn is_closer(sensor: ((i32,i32), (i32,i32)), beacon: (i32,i32)) -> bool {
    dist(sensor.0, beacon) <= dist(sensor.0, sensor.1)
}

pub fn part1() {
    let input = parse_input();
    let max_x = input.iter().map(|(sensor, beacon)| {
        sensor.0 + dist(*sensor,*beacon)
    }).max().unwrap();
    let min_x = input.iter().map(|(sensor, beacon)| {
        sensor.0 - dist(*sensor,*beacon)
    }).min().unwrap();
    let y = 2000000;
    let blocked = (min_x..=max_x).filter(|x| {
        input.iter().any(|sensor| {
            let pos = (*x, y);
             sensor.1 != pos &&
            is_closer(*sensor, pos)
        })
    }).count();
    println!("{}", blocked);
}


pub fn part2() {
    let input = parse_input();
    let sensors : Vec<_> = input.iter().map(|(s, b)| Sensor::new(*s, *b) ).collect();
    let beacon = find_beacon(&sensors);

    println!("{}", (beacon.0 as i64) * 4000000 + (beacon.1 as i64));
}