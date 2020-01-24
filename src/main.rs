use std::fs;

fn main(){
    let filename = "data/day3.txt";
    let spliter = "\r\n";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let items: Vec<&str> = contents.split(spliter).collect();

    let get_first: Vec<&str> = items[0].split(",").collect();
    let get_sec: Vec<&str> = items[1].split(",").collect();

    let data = (get_first.to_vec(),get_sec.to_vec());

    println!("data {:?}", data);
}

