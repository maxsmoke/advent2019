use std::fs;

#[derive(Debug)]
pub enum IntType{
    I32(Vec<i32>),
    F32(Vec<f32>),
    // TXT(Lines<'a, 'b>), 
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