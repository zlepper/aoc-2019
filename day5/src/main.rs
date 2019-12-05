use aoc_lib::AocImplementation;

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

#[derive(Clone, Debug, Eq, PartialEq)]
enum Parameter {
    Position(i32),
    Immediate(i32),
}

impl Parameter {
    fn parse(mode: &str, value: &i32, index: &i32) -> Parameter {
        match mode {
            "0" => Parameter::Position(*value),
            "1" => Parameter::Immediate(*index),
            _ => panic!("Unexpected parameter mode: {}", mode)
        }
    }

    fn get_value(&self, program: &Vec<i32>) -> i32 {
        match self {
            Parameter::Position(pos) => program[*pos as usize],
            Parameter::Immediate(value) => program[*value as usize],
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
    JumpIfTrue { value: Parameter, target: Parameter },
    JumpIfFalse { value: Parameter, target: Parameter },
    LessThan { first: Parameter, second: Parameter, out: i32 },
    Equals { first: Parameter, second: Parameter, out: i32 },
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
            "05" => {
                let params = Instruction::parse_parameters(2, &op_parts[..], program, index);

                let value = params[0].to_owned();
                let target = params[1].to_owned();

                Instruction::JumpIfTrue { value, target }
            }
            "06" => {
                let params = Instruction::parse_parameters(2, &op_parts[..], program, index);

                let value = params[0].to_owned();
                let target = params[1].to_owned();

                Instruction::JumpIfFalse { value, target }
            }
            "07" => {
                let params = Instruction::parse_parameters(3, &op_parts[..], program, index);

                let first = params[0].to_owned();
                let second = params[1].to_owned();
                let out = params[2].unwrap_inner();

                Instruction::LessThan { first, second, out }
            }
            "08" => {
                let params = Instruction::parse_parameters(3, &op_parts[..], program, index);

                let first = params[0].to_owned();
                let second = params[1].to_owned();
                let out = params[2].unwrap_inner();

                Instruction::Equals { first, second, out }
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
            Instruction::Equals { first, second, out } => 4,
            Instruction::LessThan { first, second, out } => 4,
            Instruction::JumpIfTrue { value, target } => 3,
            Instruction::JumpIfFalse { value, target } => 3,
            Instruction::Input { out } => 2,
            Instruction::Output { p } => 2,
            Instruction::Terminate => 1,
        }
    }

    fn parse_parameters(number: i32, op_parts: &str, program: &Vec<i32>, instruction_pointer: &i32) -> Vec<Parameter> {
        let mut p = Vec::new();

        for i in (0..3).rev().take(number as usize) {
            let idx = i as usize;
            let mode = &op_parts[idx..=idx];
            println!("Loading mode {} for idx {}", mode, idx);
            let total_idx = (3 - i - 1) + *instruction_pointer + 1;
            let value = program[total_idx as usize];

            let param = Parameter::parse(mode, &value, &total_idx);
            p.push(param)
        }

        p
    }

    // true if the program should continue
    fn execute(&self, program: &mut Vec<i32>, input: &i32, outputs: &mut Vec<i32>) -> InstructionResult {
        println!("Executing instruction: {:#?}", self);
        match self {
            Instruction::Add { left, right, out } => {
                program[*out as usize] = left.get_value(program) + right.get_value(program);
                InstructionResult::Continue(self.len())
            }
            Instruction::Multiply { left, right, out } => {
                program[*out as usize] = left.get_value(program) * right.get_value(program);
                InstructionResult::Continue(self.len())
            }
            Instruction::Input { out } => {
                program[*out as usize] = *input;
                InstructionResult::Continue(self.len())
            }
            Instruction::Output { p } => {
                let value = p.get_value(program);
                outputs.push(value);
                InstructionResult::Continue(self.len())
            }
            Instruction::Terminate => {
                InstructionResult::Halt
            }
            Instruction::LessThan { first, second, out } => {
                let result = if first.get_value(program) < second.get_value(program) {
                    1
                } else {
                    0
                };
                program[*out as usize] = result;
                InstructionResult::Continue(self.len())
            }
            Instruction::Equals { first, second, out } => {
                let result = if first.get_value(program) == second.get_value(program) {
                    1
                } else {
                    0
                };
                program[*out as usize] = result;
                InstructionResult::Continue(self.len())
            }
            Instruction::JumpIfTrue { value, target } => {
                let v = value.get_value(program);
                println!("Checking if {} is true", v);
                if v != 0 {
                    InstructionResult::GoTo(target.get_value(program))
                } else {
                    InstructionResult::Continue(self.len())
                }
            }
            Instruction::JumpIfFalse { value, target } => {
                let v = value.get_value(program);
                println!("Checking if {} is false", v);
                if v == 0 {
                    InstructionResult::GoTo(target.get_value(program))
                } else {
                    InstructionResult::Continue(self.len())
                }
            }
        }
    }
}

#[derive(Debug)]
enum InstructionResult {
    Continue(i32),
    GoTo(i32),
    Halt,
}

struct ExecutionResult {
    program: Vec<i32>,
    outputs: Vec<i32>,
}

fn run_intcode(mut program: Vec<i32>, input: i32) -> ExecutionResult {
    let end = program.len() as i32;
    let mut outputs = Vec::new();

    let mut i = 0;
    while i < end {
        let op = program[i as usize];
        let instruction = Instruction::parse(&i, &program);
        println!("Index: {}, op: {}, state: {:#?}", i, op, program);
        let result = instruction.execute(&mut program, &input, &mut outputs);
        println!("Result: {:#?}", result);
        match result {
            InstructionResult::Halt => return ExecutionResult { program, outputs },
            InstructionResult::Continue(by) => i += by,
            InstructionResult::GoTo(target) => i = target,
        }
    }

    ExecutionResult {
        program,
        outputs,
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
        let result = run_intcode(vec![1002, 4, 3, 4, 33], 1);
        assert_eq!(result.program, vec![1002, 4, 3, 4, 99]);
    }

    #[test]
    fn example6_true() {
        let result = run_intcode(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8);
        assert_eq!(result.outputs.last().unwrap(), &1);
    }

    #[test]
    fn example6_false() {
        let result = run_intcode(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example7_true() {
        let result = run_intcode(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7);
        assert_eq!(result.outputs.last().unwrap(), &1);
    }

    #[test]
    fn example7_false() {
        let result = run_intcode(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example7_false2() {
        let result = run_intcode(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 9);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example8_true() {
        let result = run_intcode(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8);
        assert_eq!(result.outputs.last().unwrap(), &1);
    }

    #[test]
    fn example8_false1() {
        let result = run_intcode(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 7);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example8_false2() {
        let result = run_intcode(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 9);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example9_true() {
        let result = run_intcode(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7);
        assert_eq!(result.outputs.last().unwrap(), &1);
    }

    #[test]
    fn example9_false1() {
        let result = run_intcode(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example9_false2() {
        let result = run_intcode(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 9);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example10() {
        let result = run_intcode(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 0);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example10_1() {
        let result = run_intcode(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 1);
        assert_eq!(result.outputs.last().unwrap(), &1);
    }

    #[test]
    fn example10_2() {
        let result = run_intcode(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 2);
        assert_eq!(result.outputs.last().unwrap(), &1);
    }

    #[test]
    fn example11_0() {
        let result = run_intcode(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0);
        assert_eq!(result.outputs.last().unwrap(), &0);
    }

    #[test]
    fn example11_1() {
        let result = run_intcode(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 1);
        assert_eq!(result.outputs.last().unwrap(), &1);
    }

    #[test]
    fn example11_2() {
        let result = run_intcode(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 2);
        assert_eq!(result.outputs.last().unwrap(), &1);
    }


    fn get_large_program() -> Vec<i32> {
        vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]
    }

    #[test]
    fn example_large_less_than() {
        let result = *run_intcode(get_large_program(), 7).outputs.last().unwrap();
        assert_eq!(result, 999);
    }

    #[test]
    fn example_large_equal_to() {
        let result = *run_intcode(get_large_program(), 8).outputs.last().unwrap();
        assert_eq!(result, 1000);
    }

    #[test]
    fn example_large_greater_than() {
        let result = *run_intcode(get_large_program(), 9).outputs.last().unwrap();
        assert_eq!(result, 1001);
    }

    #[test]
    fn gets_parameters() {
        let opcode = "01105";

        let program = vec![1105, 2, 4];
        let params = Instruction::parse_parameters(2, opcode, &program, &0);
        assert_eq!(params, vec![Parameter::parse("1", &2, &1), Parameter::parse("1", &4, &2)]);
    }
}