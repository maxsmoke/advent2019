// use std::fs;
mod utils;
use utils::{fuel_calc, IntType};

fn main() {
   let vector: Vec<f32> = Vec::new();
   let extracted = utils::read("data/day1.txt","\r\n", IntType::F32(vector));
   let mut result: f32 = 0.0;

   let data = match extracted{
        IntType::F32(e) => e,
        _ => vec!(0.0), // in case of read failure
   };

   for i in data{
       result += fuel_calculator(i);
   }
   print!("total fuel is: {}", result);
}
