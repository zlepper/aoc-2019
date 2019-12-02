use std::fs;

pub trait AocImplementation<T> {

    fn start(&self, day: i32) {
        let contents = fs::read_to_string(format!("day{}/input.txt", day)).expect("Failed to read input file");

        let parsed = self.process_input(&contents);

        let answer = self.execute(parsed);

        match answer {
            Some(a) => println!("Puzzle answer: {}", a),
            None => eprintln!("Failed to calculate answer")
        }
    }

    fn process_input(&self, input: &str) -> Vec<T>;
    fn execute(&self, input: Vec<T>) -> Option<i32>;
}
