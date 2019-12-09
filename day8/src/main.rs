use aoc_lib::AocImplementation;
use itertools::Itertools;

fn main() {
    let day = Day8{};
    day.start(8);
}

struct Day8 {}

impl AocImplementation<u8> for Day8 {
    fn process_input(&self, input: &str) -> Vec<u8> {
        input.split("").filter(|s| s != &"").map(|s| s.parse().unwrap()).collect()
    }

    fn execute(&self, input: Vec<u8>) -> Option<i32> {
        let layer: Vec<u8> = input.into_iter()
            .chunks(25 * 6)
            .into_iter()
            .map(|l| l.collect::<Vec<u8>>())
            .min_by_key(|l| l.iter().filter(|b| **b == 0).count())
            .unwrap();

        let digit1 = layer.iter().filter(|l| **l == 1).count();
        let digit2 = layer.iter().filter(|l| **l == 2).count();

        Some((digit1 * digit2) as i32)
    }
}