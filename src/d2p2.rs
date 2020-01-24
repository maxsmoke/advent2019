mod utils;
use utils::IntType;

fn main(){
    //op code 1 is addition
    //op code 2 is multiplication
    //op code 99 is halt
    /* 
        Part 2
        Which 2 inputs at index 1 & 2 will create an output of 19690720
        at 
        index 0
        possible values are [0 and 99] (inclusive)
        
        submission is as follows:
        What is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.)
    */
    let vector: Vec<i32> = Vec::new();
    let data = utils::read("data/day2.txt",",",IntType::I32(vector));   
    
    // let noun: i32 = 12;
    // let verb: i32 = 2;
    
    let intcode = match data {
        IntType::I32(vector) => vector,
        _ => vec!(0),
    };

    let desired_output: i32 = 19690720;
    let mut result_noun: i32 = 0;
    let mut result_verb: i32 = 0;

    'outer: for verb in 0..99{
        for noun in 0..99{
            let testing_incode = intcode.clone();
            let result = find_output(noun, verb, testing_incode);
            if result == desired_output{
                result_noun = noun;
                result_verb = verb;
                break 'outer;
            }
        }
    }
    
    println!("noun is {:?} and verb is {}", result_noun, result_verb);
    println!("final answer is {}", 100 * result_noun + result_verb);

}
enum Operation{
    Add,
    Multiply,
}
/* struct Answer{
    noun: i32,
    verb: i32
} */

fn operation<'a>(op_index: usize, data: &'a Vec<i32>, op: Operation) -> i32{
    let loc_1 = data[op_index + 1];
    let loc_2 = data[op_index + 2];
    match op{
        Operation::Add => data[loc_1 as usize] + data[loc_2 as usize],
        Operation::Multiply => data[loc_1 as usize] * data[loc_2 as usize],
    }
}

fn find_output(noun: i32, verb: i32,  mut intcode: Vec<i32>)  -> i32{
    intcode[1] = noun;
    intcode[2] = verb;
    println!("begin {:?}", intcode);

    let mut loc: usize  = 0;
    let length = intcode.len();

    while loc < length{
        let op_code = intcode[loc].clone();

        let answer = match op_code{
            99 => {
                println!("op 99, HALT at {}", loc);
                break;
            },
            1 => operation(loc, &intcode, Operation::Add),
            2 => operation(loc, &intcode, Operation::Multiply),
            _ => break,
        };

            let output = intcode[loc + 3];
            intcode[output as usize] = answer;

            loc = loc + 4;
    }
    println!("number at position 0: {:?}", intcode[0]);
    intcode[0]
}