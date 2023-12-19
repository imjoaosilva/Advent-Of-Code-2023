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
            let code_numbers: Vec<i32> = code
                .split("")
                .map(|x| x.parse().unwrap_or(-1))
                .filter(|&num| num != -1)
                .collect();

            let number = [code_numbers[0], code_numbers[code_numbers.len() - 1]]
                .iter()
                .map(|&num| num.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<i32>()
                .unwrap_or(-1);

            acc + number
        });

        println!("Result: {}", result);
    }
}

fn main() {
    let part_01 = Part01::new(String::from("./initial1.txt"), String::from("./input1.txt"));
    part_01.run(true);
    part_01.run(false);
}
