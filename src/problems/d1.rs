
use super::utils::{fuel_calc, IntType, read,};

pub fn run() {
   let vector: Vec<f32> = Vec::new();
   let extracted = read("data/day1.txt","\r\n", IntType::F32(vector));
   let mut result: f32 = 0.0;

   let data = match extracted{
        IntType::F32(e) => e,
        _ => vec!(0.0), // in case of read failure
   };

   for i in data{
       result += fuel_calc(i);
   }
   print!("total fuel is: {}", result);
}
