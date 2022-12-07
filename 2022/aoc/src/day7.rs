use std::collections::HashMap;
use std::fs::read_to_string;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "src/day7.pest"]
struct InputParser;

#[derive(Default, Debug)]
struct FileSystem {
    dirs: HashMap<Vec<String>, usize>,
}

impl FileSystem {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, mut file_path: Vec<String>, size: usize) {
        while file_path.len() > 0 {
            file_path.pop();
            *self.dirs.entry(file_path.clone()).or_insert(0) += size;
        }
    }
}

struct Cursor<'a> {
    fs: &'a mut FileSystem,
    path: Vec<String>,
}

impl<'a> Cursor<'a> {
    fn new(fs: &mut FileSystem) -> Cursor {
        Cursor { fs, path: Vec::new() }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        let mut file_vec = self.path.clone();
        file_vec.push(name.into());
        self.fs.insert(file_vec, size);
    }

    fn pop_all(&mut self) { self.path.clear(); }
    fn push(&mut self, dir: &str) {
        self.path.push(dir.into());
    }

    fn pop(&mut self) {
        self.path.pop();
    }
}

fn parse_command(cursor: &mut Cursor, pair: Pair<Rule>) {
   match pair.as_rule() {
       Rule::cd => {
           let dir = pair.into_inner().next().unwrap();
           match dir.as_rule() {
               Rule::filename => { cursor.push(dir.as_str()); }
               Rule::up_dir => { cursor.pop(); }
               Rule::top_level => { cursor.pop_all(); }
               _ => { unreachable!() }
           }
       },
       Rule::ls => {
           for entry in pair.into_inner() {
               match entry.as_rule() {
                   Rule::file_entry => {
                       let mut pairs = entry.into_inner();
                       let size : usize = pairs.next().unwrap().as_str().parse().unwrap();
                       let name  = pairs.next().unwrap().as_str();
                       cursor.add_file(name, size);
                   }
                   Rule::dir_entry => {}
                   _ => { unreachable!() }
               }
           }
       }
       _ => { unreachable!(); }
   }
}

fn parse_input(filename: &str) -> FileSystem {
    let mut fs = FileSystem::new();
    let mut cursor = Cursor::new(&mut fs);

    let input = read_to_string(filename).unwrap();
    match InputParser::parse(Rule::input, &input) {
        Ok(mut pairs) => {
            let commands = pairs.next().unwrap().into_inner().next().unwrap().into_inner();
            for command in commands {
                parse_command(&mut cursor, command);
            }
        }
        Err(err) => {
            println!("Error parsing input: {}", err);
            panic!();
        }
    }
    fs
}

pub fn part1() {
    let fs = parse_input("inputs/day7/input.txt");
    let sum : usize = fs.dirs.values()
        .filter(| size| **size <= 100000 )
        .sum();

    println!("{}", sum);
}

pub fn part2() {
    let fs = parse_input("inputs/day7/input.txt");
    let total_size =fs.dirs[&Vec::new()];
    let amount_needed = 30000000 + total_size - 70000000;
    let mut sizes : Vec<_> = fs.dirs.values()
        .filter(|size| **size >= amount_needed)
        .collect();
    sizes.sort();
    println!("{}", sizes.first().unwrap());
}