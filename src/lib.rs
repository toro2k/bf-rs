use std::io::Read;
use std::io::Error;


pub const INC: u8 = 43; // +
pub const GET: u8 = 44; // ,
pub const DEC: u8 = 45; // -
pub const PUT: u8 = 46; // .
pub const PREV: u8 = 60; // <
pub const NEXT: u8 = 62; // >
pub const LOOP_BEG: u8 = 91; // [
pub const LOOP_END: u8 = 93; // ]


#[derive(Debug)]
pub struct Vm {
    memory: Vec<i8>,
    pointer: usize,
}

#[derive(Debug, PartialEq)]
pub enum Inst {
    Inc,
    Dec,
}

impl Vm {
    pub fn new(size: usize) -> Option<Vm> {
        if size > 0 {
            Some(Vm { memory: vec![0; size], pointer: 0 })
        } else {
            None
        }
    }

    pub fn eval(&mut self, code: &[Inst]) {
        let mut pc = 0;

        while pc < code.len() {
            let cmd = &code[pc];

            match cmd {
                &Inst::Inc => self.inc_cell(),
                &Inst::Dec => self.dec_cell(),
            }

            pc += 1;
        }
    }

    fn inc_cell(&mut self) {
        let v = self.get_cell();
        self.put_cell(v.wrapping_add(1));
    }

    fn dec_cell(&mut self) {
        let v = self.get_cell();
        self.put_cell(v.wrapping_sub(1));
    }

    fn get_cell(&self) -> i8 {
        self.memory[self.pointer]
    }

    fn put_cell(&mut self, value: i8) {
        self.memory[self.pointer] = value;
    }
}

pub fn read_and_strip_bf_code<T: Read>(input: T) -> Result<Vec<Inst>, Error> {
    let mut code = vec![];
    for maybe_byte in input.bytes() {
        let byte = try!(maybe_byte);
        match byte {
            INC => code.push(Inst::Inc),
            DEC => code.push(Inst::Dec),
            _ => continue,
        }
    }
    Ok(code)
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn i_can_use_a_string_as_input() {
        let code = read_and_strip_bf_code("+".as_bytes());
        assert_eq!(vec![Inst::Inc], code.unwrap());
    }

    #[test]
    fn test_inc_and_dec() {
        let mut vm = Vm::new(1).unwrap();

        vm.eval(&[Inst::Inc, Inst::Inc, Inst::Dec]);
        assert_eq!(vec![1], vm.memory);
    }

    #[test]
    fn integer_arithmetic_wraps_around() {
        let mut code = vec![];
        for _ in 0..256 {
            code.push(Inst::Inc);
        }
        let mut vm = Vm::new(1).unwrap();

        vm.eval(&code);
        assert_eq!(0, vm.memory[0]);
    }
}
