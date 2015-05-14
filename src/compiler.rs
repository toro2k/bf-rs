use std::fmt;
use std::io;
use std::io::Read;

use vm::Inst;


pub fn compile_bf<T: Read>(input: T) -> Result<Vec<Inst>, Error> {

    let mut code = vec![];
    let mut counter = 0;
    let mut loop_stack = vec![];

    for maybe_byte in input.bytes() {
        let byte = try!(maybe_byte);

        match byte {
            PLUS => code.push(Inst::Inc),
            MINUS => code.push(Inst::Dec),

            GT => code.push(Inst::Next),
            LT => code.push(Inst::Prev),

            COMMA => code.push(Inst::Input),
            DOT => code.push(Inst::Output),

            LBRACK => {
                loop_stack.push(counter);
                code.push(Inst::JumpIfZero(0));
            },

            RBRACK => {
                if let Some(matching_counter) = loop_stack.pop() {
                    code.push(Inst::JumpUnlessZero(matching_counter + 1));
                    code[matching_counter] = Inst::JumpIfZero(counter + 1);
                } else {
                    return Err(Error::unmatched_bracket());
                }
            },

            _ => continue,
        }

        counter += 1;
    }

    if !loop_stack.is_empty() {
        return Err(Error::unmatched_bracket());
    }

    Ok(code)
}


const PLUS: u8 = 43; // +
const COMMA: u8 = 44; // ,
const MINUS: u8 = 45; // -
const DOT: u8 = 46; // .
const LT: u8 = 60; // <
const GT: u8 = 62; // >
const LBRACK: u8 = 91; // [
const RBRACK: u8 = 93; // ]


#[derive(Debug, PartialEq)]
pub struct Error {
    description: &'static str,
}

impl Error {
    fn unmatched_bracket() -> Error {
        Error { description: "unmatched bracket" }
    }

    fn io_error() -> Error {
        Error { description: "io error" }
    }
}

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Error {
        Error::io_error()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description)
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use vm::Inst::*;

    #[test]
    fn compile_simple_instructions() {
        let code = compile_bf("+-><,.".as_bytes()).unwrap();
        assert_eq!(vec![Inc, Dec, Next, Prev, Input, Output], code);
    }

    #[test]
    fn compile_empty_loop() {
        let code = compile_bf("[]".as_bytes()).unwrap();
        assert_eq!(vec![JumpIfZero(0x02), JumpUnlessZero(0x01)], code);
    }

    #[test]
    fn compile_clear_loop() {
        let code = compile_bf("[-]".as_bytes()).unwrap();
        assert_eq!(vec![JumpIfZero(0x03), Dec, JumpUnlessZero(0x01)], code);
    }

    #[test]
    fn unmatched_right_bracket() {
        let error = compile_bf("[]]".as_bytes()).unwrap_err();
        assert_eq!(Error::unmatched_bracket(), error);
    }

    #[test]
    fn unmatched_left_bracket() {
        let error = compile_bf("[][".as_bytes()).unwrap_err();
        assert_eq!(Error::unmatched_bracket(), error);
    }

    #[test]
    fn initial_non_command_characters_doesnt_panic_the_compiler() {
        // checks I don't do stupid things with the instructions counter!
        let code = compile_bf("a+".as_bytes()).unwrap();
        assert_eq!(vec![Inc], code);
    }
}
