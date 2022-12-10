use crate::util::read_lines;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Addx(i32),
    Noop
}
use Instruction::*;

#[derive(Debug)]
struct Cpu {
    cycle: usize,
    x: i32,
    inst_cycle: u32,
    instructions: Vec<Instruction>,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self { cycle: 1, x: 1, inst_cycle: 0, instructions }
    }

    fn is_completed(&self) -> bool {
        self.instructions.is_empty()
    }

    fn signal_strength(&self) -> i32 {
        self.cycle as i32 * self.x
    }

    fn crt_char(&self) -> char {
        let crt_pos = ((self.cycle - 1) % 40) as i32;
        if (self.x-1..=self.x+1).contains(&crt_pos) {
            'X'
        } else {
            '.'
        }
    }

    fn tick(&mut self) {
        if self.is_completed() { return; }

        self.cycle += 1;
        self.inst_cycle += 1;

        match self.instructions.first().unwrap() {
            Addx(v) => {
                if self.inst_cycle == 2 {
                    self.x += v;
                    self.next_inst();
                }
            }
            Noop => { self.next_inst(); }
        }
        return;
    }

    fn next_inst(&mut self) {
        self.inst_cycle = 0;
        self.instructions.remove(0);
    }
}

fn parse_input() -> Vec<Instruction> {
   read_lines("inputs/day10/input.txt").iter().map(|line| {
       let mut parts = line.split(" ");
       match parts.next().unwrap() {
           "addx" => Addx(parts.next().unwrap().parse().unwrap()),
           "noop" => Noop,
           _ => { unreachable!() }
       }
   }).collect()
}

pub fn part1() {
    let mut cpu = Cpu::new(parse_input());
    let total_signal_strength : i32= (0..220)
        .map(|_| {
            cpu.tick();
            (cpu.cycle, cpu.signal_strength())
        })
        .filter(|(cycle,_)| (cycle + 20) % 40 == 0 )
        .map(|(_,signal_strength)| signal_strength )
        .sum();
    println!("{}", total_signal_strength);
}

pub fn part2() {
    let mut cpu = Cpu::new(parse_input());
    let chars : Vec<_> = (0..240)
        .map(|_| {
            let char = cpu.crt_char();
            cpu.tick();
            char
        }).collect();
    chars.chunks(40).for_each(|line| println!("{}", line.iter().collect::<String>()));
}