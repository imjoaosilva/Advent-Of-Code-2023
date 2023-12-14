trait Challenge {
    fn new(initial_path: String, input_path: String) -> Self;
    fn run_tests(&self) -> ();
    fn run(&self) -> ();
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

    fn run_tests(&self) {
        println!("Running tests in: {}", self.initial_path)
    }

    fn run(&self) {
        println!("Running code in: {}", self.input_path)
    }
}

fn main() {
    let part_01 = Part01::new(String::from("initial1.txt"), String::from("input1.txt"));
    part_01.run_tests();
}
