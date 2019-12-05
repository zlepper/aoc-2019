use aoc_lib::AocImplementation;

fn main() {
    let day2 = Day5 {
        program_input: 1
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

#[derive(Clone, Debug)]
enum Parameter {
    Position(i32),
    Immediate(i32),
}

impl Parameter {
    fn parse(mode: &str, value: &i32) -> Parameter {
        match mode {
            "0" => Parameter::Position(*value),
            "1" => Parameter::Immediate(*value),
            _ => panic!("Unexpected parameter mode: {}", mode)
        }
    }

    fn get_value(&self, program: &Vec<i32>) -> i32 {
        match self {
            Parameter::Position(pos) => program[*pos as usize],
            Parameter::Immediate(value) => *value,
        }
    }

    fn unwrap_inner(&self) -> i32 {
        match self {
            Parameter::Position(pos) => *pos,
            Parameter::Immediate(value) => *value,
        }
    }
}


#[derive(Clone, Debug)]
enum Instruction {
    Add {
        left: Parameter,
        right: Parameter,
        out: i32,
    },
    Multiply {
        left: Parameter,
        right: Parameter,
        out: i32,
    },
    Input {
        out: i32
    },
    Output {
        p: Parameter
    },
    Terminate,
}

impl Instruction {
    fn parse(index: &i32, program: &Vec<i32>) -> Instruction {
        let op = program[*index as usize];

        let op_parts = format!("{:05}", op);

        let op_code = &op_parts[3..5];
        match op_code {
            "01" => {
                let params = Instruction::parse_parameters(3, &op_parts[..], program, index);

                let left = params[0].to_owned();
                let right = params[1].to_owned();
                let out = params[2].unwrap_inner();

                Instruction::Add { left, right, out }
            }
            "02" => {
                let params = Instruction::parse_parameters(3, &op_parts[..], program, index);

                let left = params[0].to_owned();
                let right = params[1].to_owned();
                let out = params[2].unwrap_inner();

                Instruction::Multiply { left, right, out }
            }
            "03" => {
                let params = Instruction::parse_parameters(1, &op_parts[..], program, index);

                let out = params[0].unwrap_inner();

                Instruction::Input { out }
            }
            "04" => {
                let params = Instruction::parse_parameters(1, &op_parts[..], program, index);

                let p = params[0].to_owned();

                Instruction::Output { p }
            }
            "99" => {
                Instruction::Terminate
            }

            _ => panic!("Unexpected instruction: {}", op_parts),
        }
    }

    fn len(&self) -> i32 {
        match self {
            Instruction::Add { left, right, out } => 4,
            Instruction::Multiply { left, right, out } => 4,
            Instruction::Input { out } => 2,
            Instruction::Output { p } => 2,
            Instruction::Terminate => 1,
        }
    }

    fn parse_parameters(number: i32, op_parts: &str, program: &Vec<i32>, instruction_pointer: &i32) -> Vec<Parameter> {
        let mut p = Vec::new();

        for i in (0..number).rev() {
            let idx = i as usize;
            let mode = &op_parts[idx..idx + 1];
            let value = program[((number - i - 1) + *instruction_pointer + 1) as usize];

            let param = Parameter::parse(mode, &value);
            p.push(param)
        }

        p
    }

    // true if the program should continue
    fn execute(&self, program: &mut Vec<i32>, input: &i32, outputs: &mut Vec<i32>) -> bool {
        match self {
            Instruction::Add { left, right, out } => {
                program[*out as usize] = left.get_value(program) + right.get_value(program);
            }
            Instruction::Multiply { left, right, out } => {
                program[*out as usize] = left.get_value(program) * right.get_value(program);
            }
            Instruction::Input { out } => {
                program[*out as usize] = *input
            }
            Instruction::Output { p } => {
                let value = p.get_value(program);
                outputs.push(value);
            }
            Instruction::Terminate => {
                return false;
            }
        }

        true
    }
}

struct ExecutionResult {
    program: Vec<i32>,
    outputs: Vec<i32>
}

fn run_intcode(mut program: Vec<i32>, input: i32) -> ExecutionResult {
    let end = program.len() as i32;
    let mut outputs = Vec::new();

    let mut i = 0;
    while i < end {
        let op = program[i as usize];
        let instruction = Instruction::parse(&i, &program);
        let should_continue = instruction.execute(&mut program, &input, &mut outputs);
        if !should_continue {
            return ExecutionResult {
                program, outputs,
            };
        }
        i += instruction.len()
    }

    ExecutionResult {
        program, outputs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let result = run_intcode(vec![1, 0, 0, 0, 99], 1);
        assert_eq!(result.program, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn example2() {
        let result = run_intcode(vec![2, 3, 0, 3, 99], 1);
        assert_eq!(result.program, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn example3() {
        let result = run_intcode(vec![2, 4, 4, 5, 99, 0], 1);
        assert_eq!(result.program, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn example4() {
        let result = run_intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], 1);
        assert_eq!(result.program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn example5() {
        let result = run_intcode(vec![1002,4,3,4,33], 1);
        assert_eq!(result.program, vec![1002,4,3,4,99]);
    }
}