use std::io;

fn main() {
    println!("Enter a number and check if is on fibonacci sequence or not ;)");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Enter a number! ");
    let number: u32 = number.trim().parse().unwrap();

    if number == 0 || number == 1 {
        println!("{number} is on fibonacci!")
    } else {
        fibonacci(number);
    }

    fn fibonacci(num: u32) {
        let mut depart = 0;
        let mut next = 1;

        let mut result = depart + next;
        println!("0 => 0");
        println!("0 => 1");

        loop {
            if result > num {
                return println!("{num} is not on fibonacci");
            } else if result == num {
                return println!("{num} is on fibonacci");
            } else {
                depart = next;
                next = result;
                result = depart + next;
                println!("{depart} => {result}")
            }
        }
    }
}

