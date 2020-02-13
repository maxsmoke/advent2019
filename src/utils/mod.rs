use std::fs;

#[derive(Debug)]
pub enum IntType{
    I32(Vec<i32>),
    F32(Vec<f32>),
}

pub fn read(filename: &str, spliter: &str, mut int_type: IntType) -> IntType{

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let items: Vec<&str> = contents.split(spliter).collect();

    let data = match int_type {
        IntType::I32(ref mut vector) => {
            for i in items {
                vector.push(i.parse::<i32>().unwrap());
            }
            IntType::I32(vector.to_owned())
        },
        IntType::F32(ref mut vector) => {
            for i in items {
                vector.push(i.parse::<f32>().unwrap());
            }
            IntType::F32(vector.to_owned())
        },
    };
    data
}
pub fn fuel_calc(mass: f32) -> f32 {
    let cal = mass / 3.0;
    cal.floor() - 2.0
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up(i32),
    Down(i32),
    Right(i32),
    Left(i32),
}

pub fn get_instructions(data: Vec<&str>) -> Vec<Direction> {
    let mut instruction: Vec<Direction> = Vec::new();

    for item in data {
        let first_char = item.get(..1).unwrap();
        let num = item.get(1..).unwrap().parse::<i32>().unwrap();

        let dir = match first_char {
            "U" => Direction::Up(num),
            "D" => Direction::Down(num),
            "L" => Direction::Left(num),
            "R" => Direction::Right(num),
            _ => Direction::Up(0),
        };
        &instruction.push(dir);
    }
    instruction
}
