use std::fs;

struct Part01<'a> {
    test_input: &'a str,
    final_input: &'a str,
}

struct Part02<'a> {
    test_input: &'a str,
    final_input: &'a str,
}

trait Challenge<'a> {
    fn new(test_input: &'a str, final_input: &'a str) -> Self;
    fn run(&self, test: bool) -> i32;
}

impl<'a> Challenge<'a> for Part01<'a> {
    fn new(test_input: &'a str, final_input: &'a str) -> Self {
        Part01 {
            final_input,
            test_input,
        }
    }

    fn run(&self, test: bool) -> i32 {
        let path = if test { &self.test_input } else { &self.final_input };

        let codes = fs::read_to_string(path).expect("Error reading the file");

        let result = codes.lines().fold(0, |ac, code| {
            let mut numbers = code.split("").filter_map(|c| c.parse::<i32>().ok());
            let first = numbers.next().unwrap();
            let second = numbers.last().unwrap_or(first);

            ac + first * 10 + second
        });

        result
    }
}

impl<'a> Challenge<'a> for Part02<'a> {
    fn new(test_input: &'a str, final_input: &'a str) -> Self {
        Part02 {
            final_input,
            test_input,
        }
    }
    fn run(&self, test: bool) -> i32 {
        let path = if test { &self.test_input } else { &self.final_input };

        let numbers_str = [
            "Zero",
            "One",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
        ];

        let codes = fs::read_to_string(path).expect("Error reading the file");

        let result = codes.lines().fold(0, |ac, code| {
            let mut new_code = code.to_string();

            numbers_str
                .iter()
                .enumerate()
                .for_each(|(i, n)| {
                    new_code = new_code.replace(&n.to_lowercase(), &i.to_string());
                });

            let mut numbers = new_code.split("").filter_map(|x| x.parse::<i32>().ok());

            let first = numbers.next().unwrap();
            let last = numbers.last().unwrap_or(first);

            println!("{first}{last}");

            ac + first * 10 + last
        });

        result
    }
}

fn main() {
    let part_01 = Part01::new("./test1.txt", "./final1.txt");
    println!("{}", part_01.run(true));
    println!("{}", part_01.run(false));
    let part_02 = Part02::new("./test2.txt", "./final2.txt");
    println!("{}", part_02.run(true));
}