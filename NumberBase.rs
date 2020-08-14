/*
 * Intakes two numbers, converts the first to the a number system of the second number
 * Jake Armendariz
 *
*/
use std::io;

//Reads integer from stdin, continues to ask and convert until proper format is given
fn read_integer() -> i64{
    let number:i64 = loop {
        let mut number_str = String::new();

        io::stdin()
            .read_line(&mut number_str)
            .expect("Failed to read line");

        match number_str.trim().parse() {
            Ok(num) => {
                break num;
            },
            Err(_) => {
                println !("Could not extract integer from number");
                continue;
            }
        };
    };
    return number
}

//returns the largest mutliple of current value < base that fits under the remaining total\
//indicates the next digit in our result
fn max_constant_multiple(base:i64, current_val:i64, remaining:i64) -> i64{
    for index in 0..base {
        if current_val * index > remaining{
            return index - 1;
        }else if  current_val * index == remaining {
            return index;
        }

    }
    return base - 1;
}

//calculates the highest power for new base system (how many digits)
fn highest_power(number:i64, base:i64) -> u32{
    let mut index:u32 = 0;
    loop {
        if base.pow(index) > number {
            return index-1;
        }
        index += 1;
    }
}

//converts the base of inputted number
fn convert_base(number:i64, base:i64) -> String{
    //gets number of digits for output
    let mut power:u32 = highest_power(number, base);
    let mut result = String::new();
    let mut remaining:i64 = number;
    //loop until the the number was created (finished 0th digit)
    loop {
        let current_val:i64 = base.pow(power);
        if current_val <= remaining {
            let constant = max_constant_multiple(base, current_val, remaining);
            result.push_str(&constant.to_string());
            remaining -= constant*current_val;
        }else{
            result.push('0');
        }

        if power == 0{ break; }
        power -= 1;
    }
    return result;
}

//Printing the arguments and result
fn main() {
    println!("Hi");
    
    println!("Please enter a 64 bit integer you would like to encode");
    let number:i64 = read_integer();

    println!("Please enter a 32 bit integer you would like as the base");
    let base:i64 = read_integer();
    
    let result = convert_base(number, base);
    println!("{}", result);
}
