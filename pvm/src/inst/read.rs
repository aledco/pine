use crate::inst::*;
use crate::parse::{Line, Literal, Parse, Token};
use crate::*;

extern crate pvm_proc_macros;
use pvm_proc_macros::*;


// Reads a character.
#[inst(name = "read", operands = [OperandFormat::Variable, OperandFormat::Variable])]
pub struct ReadInst {
    pub(crate) dest: Operand,
    pub(crate) buf: Operand
}

impl Instruction for ReadInst {
    fn execute(&mut self, env: &mut Environment) -> Result<(), Error> {
        let addr = from_u64!(self.buf.value(env)?; usize);
        let len = env.memory.len(addr)?;
        let buf = env.memory.get_buffer(addr, len)?;
        let result = env.stdin.borrow_mut().read(buf);
        match result {
            Ok(n) => {
                self.dest.set_value(to_u64!(n), env)?;
                Ok(())
            },
            Err(e) => Err(ExecuteError::error(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::*;

    #[test]
    fn test_read() {
        let bytes: &[u8] = &[97, 97, 97, 97, 97, 97];
        let stdin = Rc::new(RefCell::new(bytes));
        let stdout = Rc::new(RefCell::new(std::io::stdout()));
        let mut context = Environment::new(32, stdin, stdout);

        let addr = Operand::Variable("addr".to_string());
        let mut inst = AllocInst::new(addr.clone(), Operand::Constant(3));
        inst.execute(&mut context).unwrap();

        let d = Operand::Variable("x".to_string());
        let mut inst = ReadInst::new(d.clone(), addr.clone());
        inst.execute(&mut context).unwrap();

        let n = d.value(&context).unwrap();
        assert_eq!(n, 3);

        let addr = from_u64!(addr.value(&context).unwrap(); usize);
        let v = context.memory.load_byte(addr).unwrap();
        assert_eq!(v, 97);

        let addr = addr + 1;
        let v = context.memory.load_byte(addr).unwrap();
        assert_eq!(v, 97);

        let addr = addr + 1;
        let v = context.memory.load_byte(addr).unwrap();
        assert_eq!(v, 97);

        let addr = addr + 1;
        let v = context.memory.load_byte(addr).unwrap();
        assert_ne!(v, 97);
    }
}
