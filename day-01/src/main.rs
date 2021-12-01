use std::cmp::min;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut test = Sweep::from_file("example.txt");
    let mut my_input = Sweep::from_file("input.txt");

    // part 1
    let wdw_size = 1;
    assert_eq!(7, test.incr(wdw_size));
    println!("Increases ({}) = {}", wdw_size, my_input.incr(wdw_size));

    // part 2
    let wdw_size = 3;
    assert_eq!(5, test.incr(wdw_size));
    println!("Increases ({}) = {}", wdw_size, my_input.incr(wdw_size));

    let duration = start.elapsed();
    println!("Finished in {:?}", duration);
}
struct Sweep {
    depths: Vec<i32>
}

impl Sweep {
    fn incr(&mut self, window_size: u8) -> i32 {
        let mut tot = 0;
        let mut cur;
        let mut prv = 0;
        let max_index = self.depths.len() - window_size as usize + 1;
        for i in 0..max_index {
            cur = 0;
            for w in 0..window_size {
                cur += &self.depths[min(i+w as usize, self.depths.len()-1)];
            }
            if i > 0 { // skip the first one
                if cur - prv > 0 {
                    tot += 1;
                }
            }
            prv = cur;
        }
        tot
    }

    fn from_file(f_name: &str) -> Sweep {
        // split by empty line
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        let str_depths : Vec<&str> = str_in.split("\r\n\r\n").collect();

        let mut depths = Vec::new();
        for s in str_depths[0].lines() {
            let d = s.parse::<i32>().expect("invalid depth");
            depths.push(d);
        }
        Sweep {depths}
    }
}
