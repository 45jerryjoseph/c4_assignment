fn main() {
    run(0, 10);
}

/**
 * This function prints in t&he terminal whether an integer is an even
 *  or odd number given a range of number.
 * 
 * It accepts two parameters:
 * start: start value of range
 * limit: end value of range
 * 
 * Example:
 * 
 * run(0, 10);
 * 
 * Should print:
 * 0 is an even number
 * 1 is an odd number
 * 2 is an even number
 * 3 is an odd number
 *          .
 *          .
 *          .
 * 9 is an odd number
 */
pub fn run(start: u32, limit: u32) {
    let x =start;
    let y =limit;
    for number in x..y{
        if(number%2) == 0{
            println!("{} is an even", number);
        }else {
            println!("{} is an odd", number)
        }
    }
}
// let number:[u32;] =[start..limit];
// Write your code here
// let mut x =start;
// let y =limit;
// match num {
// }
// println!("{}", num);
// num = num + 1;
// while num <10 {
//     println!("{}",num);
//     num = num + 1;

// }

