use std::fs;
// mod utils;
use utils::{Direction, get_instructions};

pub fn run() {
    let filename = "data/day3small.txt";
    let spliter = "\r\n";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let items: Vec<&str> = contents.split(spliter).collect();

    let get_first: Vec<&str> = items[0].split(",").collect();
    let get_sec: Vec<&str> = items[1].split(",").collect();

    let line1 = drawline(get_instructions(get_first), 1);
    let line2 = drawline(get_instructions(get_sec), 2);

    let intersections = find_match(line1, line2);

    println!("intersections {:?}", intersections);

    println!("smallest combined steps {}", find_shortest(intersections));
}

#[derive(Debug,Copy, Clone)]
struct Segment{
    line: i32,
    bottom: (i32, i32),
    top: (i32, i32),
    dir: Direction,
    length: i32,
}
#[derive(Debug)]
struct Intersection{
    coords: (i32,i32),
    total_steps: i32,
}

fn drawline(instructions: Vec<Direction>, owner: i32) -> Vec<Segment> {
    let mut loc = (0, 0);
    let mut length = 0;
    let mut line: Vec<Segment> = Vec::new();
    for segment in instructions {
        match segment {
            Direction::Up(num) => {
                line.push(Segment {
                    line: owner,
                    bottom: loc,
                    top: (loc.0, loc.1 + num),
                    dir: Direction::Up(num),
                    length: num + length,
                });
                length += num;
                loc.1 += num;
            }
            Direction::Down(num) => {
                line.push(Segment {
                    line: owner,
                    bottom: (loc.0, loc.1 - num),
                    top: loc,
                    dir: Direction::Down(num),
                    length: num + length,
                });
                length += num;
                loc.1 -= num;
            }
            Direction::Right(num) => {
                line.push(Segment {
                    line: owner,
                    bottom: loc,
                    top: (loc.0 + num, loc.1),
                    dir: Direction::Right(num),
                    length: num + length,
                });
                length += num;
                loc.0 += num;
            }
            Direction::Left(num) => {
                line.push(Segment {
                    line: owner,
                    bottom: (loc.0 - num, loc.1),
                    top: loc,
                    dir: Direction::Right(num),
                    length: num + length,
                });
                length += num;
                loc.0 -= num;
            }
        }
    }
    line
}


fn find_match(l1: Vec<Segment>, l2: Vec<Segment>) -> Vec<Intersection> {
    let mut matches: Vec<Intersection> = Vec::new();
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


fn find_intersect(v: Segment, h: Segment) -> Intersection{
    println!("v-len {:?}, h- len {:?} line lengths before intersect adjustment", v.length, h.length);
    //len before intersect
    let h_seg_len = h.top.0 - h.bottom.0;
    let v_seg_len = v.top.1 - v.bottom.1;
    println!("seg len: v-{:?} h-{:?}",v_seg_len, h_seg_len);
    
    let h_adjusted = h.top.0 - v.top.0;
    let v_adjusted = v.top.1 - h.top.1;

    let v_dif = v_seg_len - v_adjusted;
    let h_dif = h_seg_len - h_adjusted;

    
    println!("seg len after intersect adjustment v-{:?} h-{:?}", v_adjusted, h_adjusted);
    let h_intersect_len = h.length - h_dif;
    let v_intersect_len = v.length - v_dif;
    println!("seg len - seg len: v-{:?} h-{:?}",v_intersect_len, h_intersect_len);

    Intersection{
        coords: (v.top.0, h.bottom.1),
        total_steps: h_intersect_len + v_intersect_len
    }
}
fn find_shortest(data: Vec<Intersection>) ->  i32{
    let mut smallest = data[0].total_steps.clone();
    for i in data {
        if i.total_steps < smallest{
            smallest = i.total_steps.clone();
        }
    }
    smallest
}

