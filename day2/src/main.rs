use aoc_lib::AocImplementation;

fn main() {
    let day2 = Day2 {};
    day2.start(2)
}

struct Day2 {}

impl AocImplementation<usize> for Day2 {
    fn process_input(&self, input: &str) -> Vec<usize> {
        input.split(',').map(|n| n.parse().unwrap()).collect()
    }

    fn execute(&self, input: Vec<usize>) -> Option<i32> {
        let program = input;
        for noun in 0..=99 {
            for verb in 0..=99 {
                let mut temp_program = program.clone();
                temp_program[1] = noun;
                temp_program[2] = verb;

                let result = run_intcode(temp_program);
                if result[0] == 19_690_720 {
                    return Some((100 * noun + verb) as i32);
                }
            }
        }
        None
    }
}

fn run_intcode(mut program: Vec<usize>) -> Vec<usize> {
    let end = program.len();

    for i in (0..end).step_by(4) {
        let op = program[i];
//        println!("op: {}, i: {}", op, i);
        match op {
            1 => {
                let pos1 = program[i + 1];
                let pos2 = program[i + 2];
                let out_pos = program[i + 3];
                let left = program[pos1];
                let right = program[pos2];
                let sum = left + right;
                program[out_pos] = sum;
            }
            2 => {
                let pos1 = program[i + 1];
                let pos2 = program[i + 2];
                let out_pos = program[i + 3];
                let left = program[pos1];
                let right = program[pos2];
                let product = left * right;
                program[out_pos] = product;
            }
            // terminal
            99 => return program,
            _ => panic!("Unexpected opcode {}", op)
        }
    }

    program
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = run_intcode(vec![1, 0, 0, 0, 99]);
        assert_eq!(result, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn example2() {
        let result = run_intcode(vec![2, 3, 0, 3, 99]);
        assert_eq!(result, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn example3() {
        let result = run_intcode(vec![2, 4, 4, 5, 99, 0]);
        assert_eq!(result, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn example4() {
        let result = run_intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        assert_eq!(result, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}