use aoc_lib::AocImplementation;
use std::collections::VecDeque;
use itertools::Itertools;
use std::ops::Range;

fn main() {
    let day7 = Day7 {
        program_input: 5
    };
    day7.start(7)
}

struct Day7 {
    program_input: i32
}

impl AocImplementation<i32> for Day7 {
    fn process_input(&self, input: &str) -> Vec<i32> {
        input.split(',').map(|n| n.parse().unwrap_or_else(|_| panic!("Failed to parse number: {}", n))).collect()
    }

    fn execute(&self, program: Vec<i32>) -> Option<i32> {
        let result = find_max_phase_signal(program, (5..=9).collect());
        Some(result.signal)
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


#[derive(Clone, Debug, Eq, PartialEq)]
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
            let total_idx = (3 - i - 1) + *instruction_pointer + 1;
            let value = program[total_idx as usize];

            let param = Parameter::parse(mode, &value, &total_idx);
            p.push(param)
        }

        p
    }

    // true if the program should continue
    fn execute(&self, program: &mut Vec<i32>, input: &mut Inputs, outputs: &mut Vec<i32>) -> InstructionResult {
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
                program[*out as usize] = input.next();
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
                if v != 0 {
                    InstructionResult::GoTo(target.get_value(program))
                } else {
                    InstructionResult::Continue(self.len())
                }
            }
            Instruction::JumpIfFalse { value, target } => {
                let v = value.get_value(program);
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

struct Inputs {
    values: VecDeque<i32>
}

impl Inputs {
    fn new() -> Inputs {
        return Inputs {
            values: VecDeque::new()
        }
    }

    fn push(&mut self, v: i32) {
        self.values.push_back(v)
    }

    fn next(&mut self) -> i32 {
        self.values.pop_front().unwrap_or(0)
    }
}


impl From<i32>  for Inputs {
    fn from(v: i32) -> Self {
        let mut input = Inputs::new();
        input.push(v);
        input
    }
}

impl From<usize>  for Inputs {
    fn from(v: usize) -> Self {
        (v as i32).into()
    }
}

impl From<(i32, i32)> for Inputs {
    fn from(v: (i32, i32)) -> Self {
        let mut input = Inputs::new();
        input.push(v.0);
        input.push(v.1);
        input
    }
}

fn run_intcode(mut program: Vec<i32>, mut input: Inputs) -> ExecutionResult {
    let end = program.len() as i32;
    let mut outputs = Vec::new();

    let mut i = 0;
    while i < end {
        let op = program[i as usize];
        let instruction = Instruction::parse(&i, &program);
        let result = instruction.execute(&mut program, &mut input, &mut outputs);
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

struct PhaseResult {
    sequence: Vec<i32>,
    signal: i32,
}

#[derive(Debug)]
struct Amplifier {
    program: Vec<i32>,
    instruction_pointer: i32,
    name: String,
    initial: bool
}

enum AmplifierResult {
    Output(i32),
    Halt,
}

impl Amplifier {
    fn new(program: Vec<i32>, name: char) -> Amplifier {
        Amplifier {
            program,
            instruction_pointer: 0,
            name: name.to_string(),
            initial: true
        }
    }

    fn run(&mut self, phase: i32, input: i32) -> AmplifierResult {
        let end = self.program.len() as i32;
        let mut outputs = Vec::new();

        while self.instruction_pointer < end {
            let op = self.program[self.instruction_pointer as usize];
            let instruction = Instruction::parse(&self.instruction_pointer, &self.program);

            let mut inputs = if self.initial {
                (phase, input).into()
            } else {
                input.into()
            };
            self.initial = false;

            let result = instruction.execute(&mut self.program, &mut inputs, &mut outputs);
            match instruction {
                Instruction::Output {..} => {
                    match result {
                        InstructionResult::Continue(by) => self.instruction_pointer += by,
                        _ => panic!("Output is supposed to have a continue result")
                    }

                    let o = *outputs.last().unwrap();
                    return AmplifierResult::Output(o)
                }
                _ =>  {
                    match result {
                        InstructionResult::Halt => return AmplifierResult::Halt,
                        InstructionResult::Continue(by) => self.instruction_pointer += by,
                        InstructionResult::GoTo(target) => self.instruction_pointer = target,
                    }
                }
            }


        }

        unreachable!()
    }
}

fn run_for_phase_signal(sequence: &Vec<usize>, program: Vec<i32>) -> i32 {
    let mut amplifiers: Vec<Amplifier> = (b'A'..=b'E').map(char::from).map(|name| Amplifier::new(program.clone(), name)).collect();

//        println!("amps: {:#?}", amplifiers);
    let mut last_e_output = 0;
    let mut next_input = 0;
    for (index, phase) in sequence.iter().enumerate().cycle() {
        let amp = &mut amplifiers[index];
//        println!("Running amp: {} with input {} and phase {}", amp.name, next_input, phase);
        let result = amp.run(*phase as i32, next_input);
        match result {
            AmplifierResult::Output(o) => {
//                println!("Got output: {} from {}", o, amp.name);
                if amp.name == "E" {
                    last_e_output = o;
                }
                next_input = o;
            },
            AmplifierResult::Halt => {
                println!("Got halt instruction, last from e: {}", last_e_output);
                return last_e_output
            },
        }
    }

//    for i in sequence {
//        let mut inputs: Inputs = (*i).into();
//        inputs.push(next_input);
//        let result = run_intcode(program.clone(), inputs);
//        next_input = *result.outputs.last().unwrap();
//
//    }

    next_input
}

fn find_max_phase_signal(program: Vec<i32>, sequence: Vec<usize>) -> PhaseResult  {
    let amplifier_count = 5;
    let max_sequence = sequence.into_iter().permutations(amplifier_count).max_by_key(|sequence| {
        run_for_phase_signal(sequence, program.clone())
    }).unwrap();

    let signal = run_for_phase_signal(&max_sequence, program.clone());

    PhaseResult {
        signal,
        sequence: max_sequence.into_iter().map(|v| v as i32).collect()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    mod max_phase_signal {
        use super::*;

//        #[test]
//        fn example1() {
//            let result = find_max_phase_signal(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], (0..5).collect());
//            assert_eq!(result.signal, 43210);
//            assert_eq!(result.sequence, vec![4,3,2,1,0])
//        }
//
//        #[test]
//        fn example2() {
//            let result = find_max_phase_signal(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], (0..5).collect());
//            assert_eq!(result.signal, 54321);
//            assert_eq!(result.sequence, vec![0,1,2,3,4])
//        }
//
//        #[test]
//        fn example3() {
//            let result = find_max_phase_signal(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], (0..5).collect());
//            assert_eq!(result.signal, 65210);
//            assert_eq!(result.sequence, vec![1,0,4,3,2])
//        }

        #[test]
        fn example4() {
            let result = find_max_phase_signal(vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5], (5..=9).collect());
            assert_eq!(result.signal, 139629729);
            assert_eq!(result.sequence, vec![9,8,7,6,5]);
        }

        #[test]
        fn example5() {
            let result = find_max_phase_signal(vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                                                    -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                                                    53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10], (5..=9).collect());
            assert_eq!(result.signal, 18216);
            assert_eq!(result.sequence, vec![9,7,8,5,6]);
        }

    }

    mod intcode_machine {
        use super::*;
        #[test]
        fn example1() {
            let result = run_intcode(vec![1, 0, 0, 0, 99], 1.into());
            assert_eq!(result.program, vec![2, 0, 0, 0, 99]);
        }

        #[test]
        fn example2() {
            let result = run_intcode(vec![2, 3, 0, 3, 99], 1.into());
            assert_eq!(result.program, vec![2, 3, 0, 6, 99]);
        }

        #[test]
        fn example3() {
            let result = run_intcode(vec![2, 4, 4, 5, 99, 0], 1.into());
            assert_eq!(result.program, vec![2, 4, 4, 5, 99, 9801]);
        }

        #[test]
        fn example4() {
            let result = run_intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], 1.into());
            assert_eq!(result.program, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
        }

        #[test]
        fn example5() {
            let result = run_intcode(vec![1002, 4, 3, 4, 33], 1.into());
            assert_eq!(result.program, vec![1002, 4, 3, 4, 99]);
        }

        #[test]
        fn example6_true() {
            let result = run_intcode(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 8.into());
            assert_eq!(result.outputs.last().unwrap(), &1);
        }

        #[test]
        fn example6_false() {
            let result = run_intcode(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], 7.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example7_true() {
            let result = run_intcode(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 7.into());
            assert_eq!(result.outputs.last().unwrap(), &1);
        }

        #[test]
        fn example7_false() {
            let result = run_intcode(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 8.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example7_false2() {
            let result = run_intcode(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], 9.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example8_true() {
            let result = run_intcode(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 8.into());
            assert_eq!(result.outputs.last().unwrap(), &1);
        }

        #[test]
        fn example8_false1() {
            let result = run_intcode(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 7.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example8_false2() {
            let result = run_intcode(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], 9.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example9_true() {
            let result = run_intcode(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 7.into());
            assert_eq!(result.outputs.last().unwrap(), &1);
        }

        #[test]
        fn example9_false1() {
            let result = run_intcode(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 8.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example9_false2() {
            let result = run_intcode(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], 9.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example10() {
            let result = run_intcode(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 0.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example10_1() {
            let result = run_intcode(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 1.into());
            assert_eq!(result.outputs.last().unwrap(), &1);
        }

        #[test]
        fn example10_2() {
            let result = run_intcode(vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], 2.into());
            assert_eq!(result.outputs.last().unwrap(), &1);
        }

        #[test]
        fn example11_0() {
            let result = run_intcode(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 0.into());
            assert_eq!(result.outputs.last().unwrap(), &0);
        }

        #[test]
        fn example11_1() {
            let result = run_intcode(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 1.into());
            assert_eq!(result.outputs.last().unwrap(), &1);
        }

        #[test]
        fn example11_2() {
            let result = run_intcode(vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], 2.into());
            assert_eq!(result.outputs.last().unwrap(), &1);
        }


        fn get_large_program() -> Vec<i32> {
            vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99]
        }

        #[test]
        fn example_large_less_than() {
            let result = *run_intcode(get_large_program(), 7.into()).outputs.last().unwrap();
            assert_eq!(result, 999);
        }

        #[test]
        fn example_large_equal_to() {
            let result = *run_intcode(get_large_program(), 8.into()).outputs.last().unwrap();
            assert_eq!(result, 1000);
        }

        #[test]
        fn example_large_greater_than() {
            let result = *run_intcode(get_large_program(), 9.into()).outputs.last().unwrap();
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
}