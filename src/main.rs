use std::fs;
mod utils;
use utils::{Direction, get_instructions};

fn main() {
    let filename = "data/day3xsmall.txt";
    let spliter = "\r\n";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let items: Vec<&str> = contents.split(spliter).collect();

    let get_first: Vec<&str> = items[0].split(",").collect();
    let get_sec: Vec<&str> = items[1].split(",").collect();

    let line1 = drawline(get_instructions(get_first), 1);
    let line2 = drawline(get_instructions(get_sec), 2);

    // println!("line1 {:?}, {:?}", line1[0], line1[1]);

    let intersections = find_match(line1, line2);

    println!("intersections {:?}", intersections);
    
    let mut distance: Vec<i32> = find_intersect_distance(intersections);
    distance.sort();

    println!("distances {:?}", distance);
}

#[derive(Debug,Copy, Clone)]
struct Segment{
    line: i32,
    bottom: (i32, i32),
    top: (i32, i32),
    dir: Direction,
    length: i32,
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
fn find_intersect(v: Segment, h: Segment) -> (i32,i32){
    println!("v-len {:?}, h- len {:?} before intersect adjustment", v.length, h.length);
    //len before intersect
    let h_seg_len = h.top.0 - h.bottom.0;
    let v_seg_len = v.top.1 - v.bottom.1;
    println!("seg len: v-{:?} h-{:?}",v_seg_len, h_seg_len);
    
    // let h_intersect_len = h_len + v.top.0;
    let h_intersect_len = h.length - h_seg_len + v.top.0;
    let v_intersect_len = v.length - v_seg_len + h.top.1;
    println!("seg len - seg len: v-{:?} h-{:?}",v_intersect_len, h_intersect_len);

    // let h_dif = h.length - h_intersect_len;
    // let v_dif = v.length - v_intersect_len;
    // println!("h-seg len adjusted {}", h_dif);
    // println!("v-seg len adjusted {}", v_dif);
    println!("");
    // println!("v-length - remainder = {}", h.length - remainder);
    // println!("h-length = {}", v.length - v.bottom.1);
    (v.top.0, h.bottom.1)
}

fn find_intersect_distance(val: Vec<(i32,i32)>) -> Vec<i32>{  
    let mut result: Vec<i32> = Vec::new();  

    for mut n in val{
        if n.0 < 0{
            n.0 *= -1;
        }
        if n.1 < 0{
            n.1 *= -1;
        }
        result.push(n.0 + n.1);
    }
    result
}
