use std::{borrow::Borrow, usize};

fn main() {




}

struct Register {
     value : u128
}

struct Machine {
    instruction_pointer : usize,
    tape : Vec<Register>
}

impl Register {

    pub fn new() -> Register {
        Register
        {
            value : 0
        }
    }

    pub fn increment(&mut self) -> () {
        self.value = self.value + 1;
    }

    pub fn decrement(&mut self) -> () {
        self.value = self.value - 1 ;
    }

    pub fn zero (& mut self) -> (){
        self.value = 0;
    }

    pub fn copy(&mut self , val : u128) -> (){
        self.value = val;
    }

    pub fn is_zero(&self) -> bool{
        self.value == 0
    }
    
}

impl Machine{
    pub fn new() -> Machine{
        Machine{
            tape : Vec::with_capacity(64),
            instruction_pointer : 0
        }
    }

    pub fn increment(&mut self , r1 : usize) -> () {
        match self.tape.get_mut(r1) {
            None => panic!( " {} is not a valid register", r1) ,
            Some(r ) => r.increment()
        }
    }

    pub fn decrement(&mut self , r1 : usize) -> () {
        match self.tape.get_mut(r1) {
            None => panic!( " {} is not a valid register", r1) ,
            Some(r ) => r.decrement()
        }
    }

    pub fn zero (& mut self , r1 : usize) -> (){
        match self.tape.get_mut(r1) {
            None => panic!( " {} is not a valid register", r1) ,
            Some(r ) => r.zero()
        }
    }

    pub fn copy(&mut self , r1 : usize , r2 : usize) -> (){

        let val : u128;

        match self.tape.get(r1) {
            None => panic!( " {} is not a valid register", r1) ,
            Some(r ) => val =  r.value
        }

        match self.tape.get_mut(r2){
            None => panic!( " {} is not a valid register", r2) ,
            Some(r ) => r.copy(val)
        }
        
    }

    pub fn jump(&mut self, n : usize) -> () {
        self.instruction_pointer = n
    }

    pub fn jump_if_zero(&mut self, r1 : usize, n : usize) {
        let val : u128;

        match self.tape.get(r1) {
            None => panic!( " {} is not a valid register", r1) ,
            Some(r ) => val =  r.value
        }

        if r1 == 0  { self.jump(n) }
    }

    
}