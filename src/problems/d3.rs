
use super::utils::{Direction, get_instructions};
use std::fs;

pub fn run() {
    let filename = "data/day3small.txt";
    let spliter = "\r\n";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let items: Vec<&str> = contents.split(spliter).collect();

    let get_first: Vec<&str> = items[0].split(",").collect();
    let get_sec: Vec<&str> = items[1].split(",").collect();

    let line1 = drawline(get_instructions(get_first));
    let line2 = drawline(get_instructions(get_sec));

    let intersections = find_match(line1, line2);

    println!("intersections {:?}", intersections);
    
    let mut distance: Vec<i32> = find_distance(intersections);
    distance.sort();

    println!("distances {:?}", distance);
}

#[derive(Copy, Clone, Debug)]
struct Segment {
    bottom: (i32, i32),
    top: (i32, i32),
    dir: Direction,
}

fn drawline(instructions: Vec<Direction>) -> Vec<Segment> {
    let mut loc = (0, 0);
    // let mut line = vec!(loc.clone());
    let mut line: Vec<Segment> = Vec::new();
    for segment in instructions {
        match segment {
            Direction::Up(num) => {
                line.push(Segment {
                    bottom: loc,
                    top: (loc.0, loc.1 + num),
                    dir: Direction::Up(num),
                });
                loc.1 += num.clone();
            }
            Direction::Down(num) => {
                line.push(Segment {
                    bottom: (loc.0, loc.1 - num),
                    top: loc,
                    dir: Direction::Down(num),
                });
                loc.1 -= num.clone();
            }
            Direction::Right(num) => {
                line.push(Segment {
                    bottom: loc,
                    top: (loc.0 + num, loc.1),
                    dir: Direction::Right(num),
                });
                loc.0 += num.clone();
            }
            Direction::Left(num) => {
                line.push(Segment {
                    bottom: (loc.0 - num, loc.1),
                    top: loc,
                    dir: Direction::Right(num),
                });
                loc.0 -= num.clone();
            }
        }
    }
    line
}

fn find_match(l1: Vec<Segment>, l2: Vec<Segment>) -> Vec<(i32, i32)> {
    let mut matches: Vec<(i32, i32)> = Vec::new();
    for seg in l1 {
        for seg2 in &l2 {
            match seg.dir {
                Direction::Up(_n) | Direction::Down(_n) => match seg2.dir {
                    Direction::Left(_n) | Direction::Right(_n) => {
                        if seg2.bottom.0 < seg.bottom.0 && seg.bottom.0 < seg2.top.0 {
                            if seg.bottom.1 < seg2.top.1 && seg2.top.1 < seg.top.1{
                                matches.push(find_intersect(seg, seg2.clone()));
                            }
                        } else {
                            continue;
                        }
                    }
                    Direction::Up(_n) | Direction::Down(_n) => continue,
                },
                Direction::Left(_n) | Direction::Right(_n) => match seg2.dir {
                    Direction::Up(_n) | Direction::Down(_n) => {
                        if seg.bottom.0 < seg2.bottom.0 && seg2.bottom.0 < seg.top.0 {
                            if seg2.bottom.1 < seg.top.1 && seg.top.1 < seg2.top.1{
                                matches.push(find_intersect(seg2.clone(), seg));
                            }
                        } else {
                            continue;
                        }
                    }
                    Direction::Left(_n) | Direction::Right(_n) => {
                        continue;
                    }
                },
            }
        }
    }
    matches
}
fn find_intersect(vertical: Segment, horizontal: Segment) -> (i32,i32){
    (vertical.top.0, horizontal.bottom.1)
}

fn find_distance(val: Vec<(i32,i32)>) -> Vec<i32>{  
    let mut result: Vec<i32> = Vec::new();  

    for n in val{
        result.push(n.0.abs() + n.1.abs());
    }
    result
}
