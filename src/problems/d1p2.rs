
use super::utils::{fuel_calc, IntType, read};

pub fn run() {
    let vector: Vec<f32> = Vec::new();
    let extracted = read("data/day1.txt", "\r\n", IntType::F32(vector));
    let mut module_fuel: f32 = 0.0;

    let data = match extracted{
        IntType::F32(e) => e,
        _ => vec!(0.0),// in case of read failure
    };
    for i in data{
       let acc = 0.0;
       let fuel_mass = fuel_calc(i);
       module_fuel += fuel_mass + fuel_mass_calc(fuel_mass, acc);

    }
    print!("total fuel + module fuel: {}",  module_fuel);
}

fn fuel_mass_calc(mass: f32, mut aggregator: f32) -> f32{
    let test = fuel_calc(mass);
    if test <= 0.0{
        aggregator
    }
    else{
        aggregator += fuel_calc(mass);
        fuel_mass_calc(fuel_calc(mass), aggregator)
    }
}

