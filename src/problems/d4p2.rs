// use std::str;
/*
FIND 6 DIGIT NUMBER WITH THESE CONDITIONS

It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).

EXAMPLES
    111111 meets these criteria (double 11, never decreases).
    223450 does not meet these criteria (decreasing pair of digits 50).
    123789 does not meet these criteria (no double).
*/
pub fn run() {
    let low = 353_096;
    let high = 843_212;

    let testnum = 112233;
    let testnum2 = 123444;
    let testnum3 = 112233;
    let testnum4 = 222226;

    let test = testnum.to_string().into_bytes();
    let test2 = testnum2.to_string().into_bytes();
    let test3 = testnum3.to_string().into_bytes();
    let test4 = testnum4.to_string().into_bytes();

    println!("test should be true: {}",has_dupe(test));
    println!("test2 should be false: {}",has_dupe(test2));
    println!("test3 should be true: {}",has_dupe(test3));
    println!("test4 should be false: {}",has_dupe(test4));
    
    /* let mut result: Vec<i32> = Vec::new();
    for num in low .. high {
        // if number descends false continue to next number
        //convert Int into bytes Vec<u8> for easy character comparison
        if descends(num.to_string().into_bytes()) == false{
            continue;
        }
        // if number has an adjacent duplicate
        else if has_dupe(num.to_string().into_bytes()) == false{
            continue;
        }
        else{
            result.push(num);
        }
    } 
    println!("result: {:?}", result.len()); */
   
}
fn descends(digits: Vec<u8>) -> bool {
    let mut last: u8 = digits[0].clone();
    for current in digits {
        if current < last {
            return false;
        } else {
            last = current.clone();
            continue;
        };
    }
    true
}
fn has_dupe(digits: Vec<u8>) -> bool{
    let mut count = 0; 
    let mut data: Vec<i32> = Vec::new();

    for n in 0..digits.len() {
        if n + 1 < digits.len(){
            let first = digits[n];
            let sec = digits[n + 1];
            if first == sec{
                count = count + 1;
                continue;
            }else{
                if count == 0{
                    continue;
                }else{
                    data.push(count);
                    count = 0;
                }
            }
        }
    }
    print!("data {:?}", data);
    if data.len() == 0 {
        return true
    }else {
        for i in 0..data.len(){
            if data[i] % 2 == 0{
                return false;
            }
            else{
                continue;
            }
        }
    }
    true
}
