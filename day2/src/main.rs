use std::fs;

fn main() {
    let mut program: Vec<i32> = fs::read_to_string("input.txt").unwrap().split(',').map(|n| n.parse().unwrap()).collect();


    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut temp_program = program.clone();
            temp_program[1] = noun;
            temp_program[2] = verb;

            let result= run_intcode(temp_program);
            if result[0] == 19_690_720 {
                println!("Verb={}, noun={}, answer={}", verb, noun, 100 * noun + verb);
                return;
            }
        }
    }

}

fn run_intcode(mut program: Vec<i32>) -> Vec<i32> {
    let end = program.len();

    for i in (0..end).step_by(4)  {
        let op = program[i];
//        println!("op: {}, i: {}", op, i);
        match op {
            1 => {
                let pos1 = program[i + 1];
                let pos2 = program[i + 2];
                let out_pos = program[i + 3];
                let left = program[pos1 as usize];
                let right = program[pos2 as usize];
                let sum = left + right;
                program[out_pos as usize] = sum;
            },
            2 => {
                let pos1 = program[i + 1];
                let pos2 = program[i + 2];
                let out_pos = program[i + 3];
                let left = program[pos1 as usize];
                let right = program[pos2 as usize];
                let product = left * right;
                program[out_pos as usize] = product;
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
        let result = run_intcode(vec![1,0,0,0,99]);
        assert_eq!(result, vec![2,0,0,0,99]);
    }
    #[test]
    fn example2() {
        let result = run_intcode(vec![2,3,0,3,99]);
        assert_eq!(result, vec![2,3,0,6,99]);
    }
    #[test]
    fn example3() {
        let result = run_intcode(vec![2,4,4,5,99,0]);
        assert_eq!(result, vec![2,4,4,5,99,9801]);
    }
    #[test]
    fn example4() {
        let result = run_intcode(vec![1,1,1,4,99,5,6,0,99]);
        assert_eq!(result, vec![30,1,1,4,2,5,6,0,99]);
    }
}