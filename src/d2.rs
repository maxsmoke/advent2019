mod utils;
use utils::IntType;

fn main(){
    //op code 1 is addition
    //op code 2 is multiplication
    //op code 99 is halt
    /* 
    example
    1,9,10,3,2,3,11,0,99,30,40,50

    op code 1 @ index[0]; 
        index[9] + index[10] => index[4];

    mov 4 index's to next op code.
    
    op code 2 @ index[4]; 
    index[3] * index[11] => index[7];
    
    mov 4 index's to next op code.

    op code 99 @ index [8];
    HALT
    */
    let vector: Vec<i32> = Vec::new(); //FINAL
    let data = utils::read("data/day2.txt",",",IntType::I32(vector)); //FINAL
    // let data = IntType::I32(vec!(1,9,10,3,2,3,11,0,99,30,40,50)); //testing
    
    
    let mut intcode = match data {
        IntType::I32(vector) => vector,
        _ => vec!(0),
    };

    /* FINAL
    intcode[1] = 12;
    intcode[2] = 2;
    */
    intcode[1] = 12;
    intcode[2] = 2;

    println!("begin {:?}", intcode);

    let mut loc: usize  = 0;
    let length = intcode.len();

    while loc < length{
        let op_code = intcode[loc].clone();

        let answer = match op_code{
            99 => {
                println!("op 99, HALT at {}", loc);
                loc = length;
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
    println!("the end: {:?}", intcode);
    print!("number at position 0: {:?}", intcode[0]);

}
enum Operation{
    Add,
    Multiply,
}

fn operation<'a>(op_index: usize, data: &'a Vec<i32>, op: Operation) -> i32{
    let loc_1 = data[op_index + 1];
    let loc_2 = data[op_index + 2];
    match op{
        Operation::Add => data[loc_1 as usize] + data[loc_2 as usize],
        Operation::Multiply => data[loc_1 as usize] * data[loc_2 as usize],
    }
}
