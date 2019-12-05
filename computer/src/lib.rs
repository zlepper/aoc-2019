pub struct Program {
    memory: Vec<i32>,
    instruction_pointer: usize,
}

impl Program {
    pub fn new(code: Vec<i32>) -> Program {
        Program {
            memory: code,
            instruction_pointer: 0,
        }
    }

    pub fn execute(&mut self, input: i32) -> Vec<i32> {
        let end = self.memory.len();
        let mut outputs = Vec::new();

        while self.instruction_pointer < end {
            let instruction = Instruction::parse(self);
            let result = instruction.execute(&mut self.memory, &input, &mut outputs);
            println!("Result: {:#?}", result);
            match result {
                InstructionResult::Halt => return outputs,
                InstructionResult::Continue(by) => self.instruction_pointer += by as usize,
                InstructionResult::GoTo(target) => self.instruction_pointer = target as usize,
            }
        }

        outputs
    }

    pub fn get_memory(&self) -> &Vec<i32> {
        &self.memory
    }

    fn get_value(&self, offset: usize) -> i32 {
        self.memory[self.instruction_pointer + offset]
    }

    fn get_op_parts(&self) -> String {
        let op = self.get_value(0);

        format!("{:05}", op)
    }

    fn get_op_code(&self) -> i32 {
        let parts = self.get_op_parts();

        parts[3..5].parse().unwrap()
    }
}

trait GetParam<T> {
    fn get_param(&self, pos: usize) -> T;
}

impl GetParam<i32> for Program {
    fn get_param(&self, pos: usize) -> i32 {
        self.get_value(pos + 1)
    }
}

impl GetParam<Parameter> for Program {
    fn get_param(&self, pos: usize) -> Parameter {
        let parts = self.get_op_parts();
        let mode = parts.chars().rev().skip(2 + pos as usize).take(1).last().unwrap_or('0');
        let idx = self.instruction_pointer + pos + 1;
        Parameter::new(mode, idx)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Parameter {
    index: usize,
    mode: ParameterMode,
}

impl Parameter {
    fn new(mode: char, index: usize) -> Parameter {
        let m = match mode {
            '0' => ParameterMode::Position,
            '1' => ParameterMode::Immediate,
            _ => panic!("Unexpected parameter mode: {}", mode)
        };

        Parameter {
            index,
            mode: m,
        }
    }

    fn get_value(&self, program: &Vec<i32>) -> i32 {
        match self.mode {
            ParameterMode::Position => program[program[self.index] as usize],
            ParameterMode::Immediate => program[self.index],
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

trait ToParameter {
    fn into(&self, pos: i32, instruction_pointer: &i32) -> Parameter;
}

impl Instruction {
    fn get_value(pos: usize, instruction_pointer: &i32, program: &Vec<i32>) -> i32 {
        program[pos + 1 + *instruction_pointer as usize]
    }

    fn parse(program: &Program) -> Instruction {
        let op_code = program.get_op_code();

        match op_code {
            1 => {
                Instruction::Add {
                    left: program.get_param(0),
                    right: program.get_param(1),
                    out: program.get_param(2),
                }
            }
            2 => {
                Instruction::Multiply {
                    left: program.get_param(0),
                    right: program.get_param(1),
                    out: program.get_param(2),
                }
            }
            3 => {
                Instruction::Input { out: program.get_param(0) }
            }
            4 => {
                Instruction::Output { p: program.get_param(0) }
            }
            5 => {
                Instruction::JumpIfTrue {
                    value: program.get_param(0),
                    target: program.get_param(0),
                }
            }
            6 => {
                Instruction::JumpIfFalse {
                    value: program.get_param(0),
                    target: program.get_param(1),
                }
            }
            7 => {
                Instruction::LessThan {
                    first: program.get_param(0),
                    second: program.get_param(1),
                    out: program.get_param(2),
                }
            }
            8 => {
                Instruction::Equals {
                    first: program.get_param(0),
                    second: program.get_param(1),
                    out: program.get_param(2),
                }
            }
            99 => {
                Instruction::Terminate
            }

            _ => panic!("Unexpected instruction: {}", op_code),
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

    // true if the program should continue
    fn execute(&self, memory: &mut Vec<i32>, input: &i32, outputs: &mut Vec<i32>) -> InstructionResult {
        println!("Executing instruction: {:#?}", self);
        match self {
            Instruction::Add { left, right, out } => {
                memory[*out as usize] = left.get_value(memory) + right.get_value(memory);
                InstructionResult::Continue(self.len())
            }
            Instruction::Multiply { left, right, out } => {
                memory[*out as usize] = left.get_value(memory) * right.get_value(memory);
                InstructionResult::Continue(self.len())
            }
            Instruction::Input { out } => {
                memory[*out as usize] = *input;
                InstructionResult::Continue(self.len())
            }
            Instruction::Output { p } => {
                let value = p.get_value(memory);
                outputs.push(value);
                InstructionResult::Continue(self.len())
            }
            Instruction::Terminate => {
                InstructionResult::Halt
            }
            Instruction::LessThan { first, second, out } => {
                let result = if first.get_value(memory) < second.get_value(memory) {
                    1
                } else {
                    0
                };
                memory[*out as usize] = result;
                InstructionResult::Continue(self.len())
            }
            Instruction::Equals { first, second, out } => {
                let result = if first.get_value(memory) == second.get_value(memory) {
                    1
                } else {
                    0
                };
                memory[*out as usize] = result;
                InstructionResult::Continue(self.len())
            }
            Instruction::JumpIfTrue { value, target } => {
                let v = value.get_value(memory);
                println!("Checking if {} is true", v);
                if v != 0 {
                    InstructionResult::GoTo(target.get_value(memory))
                } else {
                    InstructionResult::Continue(self.len())
                }
            }
            Instruction::JumpIfFalse { value, target } => {
                let v = value.get_value(memory);
                println!("Checking if {} is false", v);
                if v == 0 {
                    InstructionResult::GoTo(target.get_value(memory))
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

fn run_intcode(memory: Vec<i32>, input: i32) -> ExecutionResult {
    let mut program = Program::new(memory);

    let output = program.execute(input);

    ExecutionResult {
        outputs: output,
        program: program.memory.clone(),
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
    fn gets_position_parameters() {
        let mem = vec![5, 2, 4];

        let program = Program::new(mem);
        let p: Parameter = program.get_param(0);
        assert_eq!(p, Parameter::new('0', 1));
        let p2: Parameter = program.get_param(1);
        assert_eq!(p2, Parameter::new('0', 2));
    }

    #[test]
    fn gets_immediate_parameters() {
        let mem = vec![1105, 2, 4];

        let program = Program::new(mem);
        let p: Parameter = program.get_param(0);
        assert_eq!(p, Parameter::new('1', 1));
        let p2: Parameter = program.get_param(1);
        assert_eq!(p2, Parameter::new('1', 2));
    }

    #[test]
    fn gets_immediate_parameter_values() {
        let mem = vec![1105, 2, 4, 8, 9, 10];

        let program = Program::new(mem);
        let p: Parameter = program.get_param(0);
        let value = p.get_value(program.get_memory());
        assert_eq!(value, 2)
    }
    #[test]
    fn gets_positional_parameter_values() {
        let mem = vec![5, 2, 4, 8, 9, 10];

        let program = Program::new(mem);
        let p: Parameter = program.get_param(0);
        let value = p.get_value(program.get_memory());
        assert_eq!(value, 4)
    }

    #[test]
    fn gets_immediate_parameter_values_later() {
        let mem = vec![5, 2, 4, 1108, 5, 10];

        let mut program = Program::new(mem);
        program.instruction_pointer = 3;
        let p: Parameter = program.get_param(0);
        let value = p.get_value(program.get_memory());
        assert_eq!(value, 5)
    }

    #[test]
    fn gets_positional_parameter_values_later() {
        let mem = vec![5, 2, 4, 8, 5, 10];

        let mut program = Program::new(mem);
        program.instruction_pointer = 3;
        let p: Parameter = program.get_param(0);
        let value = p.get_value(program.get_memory());
        assert_eq!(value, 10)
    }
}