use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::{borrow::Borrow, str::FromStr, usize};
fn main() {
    let program: &str = "INC 0 
     INC 0
     INC 0
     CPY 0 1
     INC 63
     SYS 63 1";

    let mut m1 = Machine::new(program.to_string());

    m1.run_program();

    m1.print_state();
}

enum Instructions {
    Increment,
    Decrement,
    Zero,
    Copy,
    Jump,
    JumpIfZero,
    SysCall,
}

#[derive(FromPrimitive)]
enum SysCall {
    PrintNumber = 0,
    PrintNumberNL = 1,
    PrintChar = 2,
    PrintCharNL = 3,
}

#[derive(Debug, Clone, Copy)]
struct Register {
    value: i128,
}

#[derive(Debug, Clone)]
struct Machine {
    instruction_pointer: usize,
    tape: [Register; 64],
    program: String,
}

impl Register {
    pub fn new() -> Register {
        Register { value: 0 }
    }

    pub fn increment(&mut self) -> () {
        self.value = self.value + 1;
    }

    pub fn decrement(&mut self) -> () {
        self.value = self.value - 1;
    }

    pub fn zero(&mut self) -> () {
        self.value = 0;
    }

    pub fn copy(&mut self, val: i128) -> () {
        self.value = val;
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl Machine {
    pub fn new(program: String) -> Machine {
        Machine {
            tape: [Register::new(); 64],
            instruction_pointer: 0,
            program: program,
        }
    }

    pub fn increment(&mut self, r1: usize) -> () {
        self.tape[r1].increment()
    }

    pub fn decrement(&mut self, r1: usize) -> () {
        self.tape[r1].decrement()
    }

    pub fn zero(&mut self, r1: usize) -> () {
        self.tape[r1].zero()
    }

    pub fn copy(&mut self, r1: usize, r2: usize) -> () {
        self.tape[r2].copy(self.tape[r1].value)
    }

    pub fn jump(&mut self, n: usize) -> () {
        self.instruction_pointer = n
    }

    pub fn jump_if_zero(&mut self, r1: usize, n: usize) {
        if self.tape[r1].is_zero() {
            self.jump(n)
        }
    }

    pub fn sys_call(&self, syscall_reg: usize, data_reg: usize) {
        let syscall: i128 = self.tape[syscall_reg].value;
        let data: i128 = self.tape[data_reg].value;

        match FromPrimitive::from_i128(syscall) {
            Some(SysCall::PrintNumber) => print!("{}", data),
            Some(SysCall::PrintNumberNL) => println!("{}", data),
            Some(SysCall::PrintChar) => {
                print!("{}", char::from_u32(data as u32).unwrap_or_default())
            }
            Some(SysCall::PrintCharNL) => {
                println!("{}", char::from_u32(data as u32).unwrap_or_default())
            }
            None => panic!("Invalid syscall code : {}", syscall),
        }
    }

    pub fn get_val_from_register(&self, reg: usize) -> i128 {
        self.tape[reg].value
    }

    fn exec_instruction(&mut self, inst: Instructions, arg1: usize, arg2: usize) -> () {
        match inst {
            Instructions::Increment => self.increment(arg1),
            Instructions::Decrement => self.decrement(arg1),
            Instructions::Zero => self.zero(arg1),
            Instructions::Copy => self.copy(arg1, arg2),
            Instructions::Jump => self.jump(arg1),
            Instructions::JumpIfZero => self.jump_if_zero(arg1, arg2),
            Instructions::SysCall => self.sys_call(arg1, arg2),
        }
    }

    pub fn interpret(&mut self) -> () {
        let currline = self.program.lines().nth(self.instruction_pointer).unwrap();

        let currinst = currline.split_whitespace();

        let inst: Instructions;
        let arg1: usize;
        let mut arg2: usize = 0;

        match currinst.clone().nth(0) {
            Some(s) => match s {
                "INC" => inst = Instructions::Increment,
                "DEC" => inst = Instructions::Decrement,
                "ZER" => inst = Instructions::Zero,
                "CPY" => inst = Instructions::Copy,
                "JMP" => inst = Instructions::Jump,
                "JMZ" => inst = Instructions::JumpIfZero,
                "SYS" => inst = Instructions::SysCall,
                _ => panic!("{} is not a valid instruction", s),
            },
            None => panic!("No instruction at line {}", self.instruction_pointer),
        }

        match inst {
            Instructions::Increment => {
                arg1 = currinst.clone().nth(1).unwrap().parse::<usize>().unwrap()
            }
            Instructions::Decrement => {
                arg1 = currinst.clone().nth(1).unwrap().parse::<usize>().unwrap()
            }
            Instructions::Zero => arg1 = currinst.clone().nth(1).unwrap().parse::<usize>().unwrap(),
            Instructions::Copy => {
                arg1 = currinst.clone().nth(1).unwrap().parse::<usize>().unwrap();
                arg2 = currinst.clone().nth(2).unwrap().parse::<usize>().unwrap()
            }
            Instructions::Jump => arg1 = currinst.clone().nth(1).unwrap().parse::<usize>().unwrap(),
            Instructions::JumpIfZero => {
                arg1 = currinst.clone().nth(1).unwrap().parse::<usize>().unwrap();
                arg2 = currinst.clone().nth(2).unwrap().parse::<usize>().unwrap()
            }
            Instructions::SysCall => {
                arg1 = currinst.clone().nth(1).unwrap().parse::<usize>().unwrap();
                arg2 = currinst.clone().nth(2).unwrap().parse::<usize>().unwrap()
            }
        }
        self.exec_instruction(inst, arg1, arg2);

        if self.instruction_pointer < self.program.lines().count() {
            self.instruction_pointer += 1;
        } else {
            println!("Arrived at end of program");
        }
    }

    pub fn run_program(&mut self) {
        println!("Running program...");

        let program_line_count = self.program.lines().count();

        while self.instruction_pointer < program_line_count {
            self.interpret()
        }
    }

    pub fn print_state(&self) -> () {
        println!("Program : {}", self.program);

        println!("Instruction pointer = {}", self.instruction_pointer);

        println!("\n State of registers :");
        for n in 0..64 {
            println!("R{} = {}", n, self.get_val_from_register(n as usize))
        }
    }
}
