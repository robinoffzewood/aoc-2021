use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut yellow = Submarine{x: 0, z: 0, aim: 0};

    assert_eq!(150, yellow.pilot("example.txt"));
    println!("part 1 = {}", yellow.pilot("input.txt"));

    assert_eq!(900, yellow.pilot_with_aim("example.txt"));
    println!("part 2 = {}", yellow.pilot_with_aim("input.txt"));

    let duration = start.elapsed();
    println!("Finished in {:?}", duration);
}
struct Submarine {
    x: i32,
    z: i32, // depth
    aim: i32
}

impl Submarine {
    fn pilot(&mut self, f_name: &str) -> i32 {
        self.x=0;
        self.z=0;

        // Load file
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        let moves = Submarine::parse_moves(&str_in);

        for m in moves {
            match m.0 {
                "forward" => self.x += m.1,
                "down" => self.z += m.1,
                "up" => self.z -= m.1,
                _ => panic!("invalid instruction")
            }
        }
        self.x * self.z
    }

    fn pilot_with_aim(&mut self, f_name: &str) -> i32 {
        self.x=0;
        self.z=0;
        self.aim=0;

        // Load file
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        let moves = Submarine::parse_moves(&str_in);

        for m in moves {
            match m.0 {
                "forward" => {
                    self.x += m.1;
                    self.z += self.aim * m.1;
                },
                "down" => self.aim += m.1,
                "up" => self.aim -= m.1,
                _ => panic!("invalid instruction")
            }
        }
        self.x * self.z
    }

    fn parse_moves(lines: &str) -> Vec<(&str, i32)> {
        let mut m = Vec::new();
        for line in lines.split("\n") {
            if line.is_empty() {break;}
            let l : Vec<&str> = line.split_whitespace().collect();
            assert_eq!(l.len(), 2);
            let v = l[1].parse::<i32>().unwrap();
            m.push((l[0], v));
        }
        m
    }
}
