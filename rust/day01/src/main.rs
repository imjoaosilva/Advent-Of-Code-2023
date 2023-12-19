use std::fs;

trait Challenge {
    fn new(initial_path: String, input_path: String) -> Self;
    fn run(&self, test: bool) -> ();
}

struct Part01 {
    initial_path: String,
    input_path: String,
}

impl Challenge for Part01 {
    fn new(initial_path: String, input_path: String) -> Self {
        Part01 {
            initial_path,
            input_path,
        }
    }

    fn run(&self, test: bool) {
        let path = if test {
            &self.initial_path
        } else {
            &self.input_path
        };

        let codes = fs
            ::read_to_string(path)
            .expect("Something went wrong reading the file");

        let result = codes.lines().fold(0, |acc, code| {
            let mut code_numbers = code.split("").filter_map(|x| x.parse::<i32>().ok());
            
            let first: i32 = code_numbers.next().unwrap();
            let last: i32 = code_numbers.last().unwrap_or(first);

            acc + format!("{}{}", first, last).parse::<i32>().unwrap()
        });

        println!("Result: {}", result);
    }
}

fn main() {
    let part_01 = Part01::new(String::from("./initial1.txt"), String::from("./input1.txt"));
    part_01.run(true);
    part_01.run(false);
}
