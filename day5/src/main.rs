use aoc_lib::AocImplementation;
use core::num::FpCategory::Infinite;
use term::terminfo::parm::Param;

fn main() {
    let day2 = Day5 {
        program_input: 5
    };
    day2.start(5)
}

struct Day5 {
    program_input: i32
}

impl AocImplementation<i32> for Day5 {
    fn process_input(&self, input: &str) -> Vec<i32> {
        input.split(',').map(|n| n.parse().unwrap_or_else(|_| panic!("Failed to parse number: {}", n))).collect()
    }

    fn execute(&self, program: Vec<i32>) -> Option<i32> {
        let result = run_intcode(program, self.program_input);

        println!("Outputs: {:#?}", result.outputs);

        Some((*result.outputs.last().unwrap()) as i32)
    }
}
