use std::env;

struct Caclulator;

#[allow(dead_code)]
impl Caclulator {
    fn add(&self, num1: i32, num2: i32) -> i32 {
        return num1 + num2
    }

    fn sub(&self, num1: i32, num2: i32) -> i32 {
        return num1 - num2
    }
    
    fn times(&self, num1: i32, num2: i32) -> i32 {
        return num1 * num2
    }

    fn div(&self, num1: i32, num2: i32) -> i32 {
        return num1 / num2
    }
}

fn main() {

    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();

    let calc1 = Caclulator;

    if command == "add" {
        println!("{}", calc1.add(arguments[2].parse::<i32>().unwrap(), arguments[3].parse::<i32>().unwrap()));
    }
    if command == "sub" {
        println!("{}", calc1.sub(arguments[2].parse::<i32>().unwrap(), arguments[3].parse::<i32>().unwrap()));
    }
    if command == "times" {
        println!("{}", calc1.times(arguments[2].parse::<i32>().unwrap(), arguments[3].parse::<i32>().unwrap()));
    }
    if command == "div" {
        println!("{}", calc1.div(arguments[2].parse::<i32>().unwrap(), arguments[3].parse::<i32>().unwrap()));
    }

}
