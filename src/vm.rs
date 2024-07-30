use crate::memory::{LinearMemory, Addressable};

#[derive(Debug)]
#[repr(u8)]
pub enum Register {
  A, B, C, M, SP, PC, BP, FLAGS,
}

impl Register {
  pub fn from_u8(v: u8) -> Option<Self> {
    match v {
      v if v == Register::A as u8 => Some(Register::A),
      v if v == Register::B as u8 => Some(Register::B),
      v if v == Register::C as u8 => Some(Register::C),
      v if v == Register::M as u8 => Some(Register::M),
      v if v == Register::SP as u8 => Some(Register::SP),
      v if v == Register::PC as u8 => Some(Register::PC),
      v if v == Register::BP as u8 => Some(Register::BP),
      v if v == Register::FLAGS as u8 =>Some(Register::FLAGS),
      _ => None,
    }
  }
}

#[derive(Debug)]
#[repr(u8)]
pub enum Op {
  Nop,
  Push(u8),
  PopRegister(Register),
  AddStack,
  AddRegister(Register, Register),
}

impl Op {
  pub fn value(&self) -> u8 {
    unsafe { *<*const _>::from(self).cast::<u8>() }
  }
}

/**
  * insruction = [0 0 0 0 0 0 0 0 | 0 0 0 0 0 0 0 0]
  *                   OPERATOR    | ARG(s)
  *                               | 8 bit literal
  *                               | REG1 | REG2
*/
fn parse_instruction(ins: u16) -> Result<Op, String> {
  let op = (ins & 0xff) as u8;
  match op {
    x if x == Op::Nop.value() => Ok(Op::Nop),
    x if x == Op::Push(0).value() => {
      let args = (ins & 0xff00) >> 8;
      Ok(Op::Push(args as u8))
    },
    x if x == Op::PopRegister(Register::A).value() => {
      let reg = ((ins & 0xf00) >> 8) as u8;
      if let Some(r) = Register::from_u8(reg) {
        Ok(Op::PopRegister(r))
      } else {
        Err(format!("unknown register 0x{:X}", reg))
      }
    },
    x if x == Op::AddStack.value() => {
      Ok(Op::AddStack)
    },
    _ => Err(format!("unknown operator 0x{:X}", op)),
  }
}

pub struct Machine {
  registers: [u16; 8],
  pub memory: Box<dyn Addressable>,
}

impl Machine {
  pub fn new() -> Self {
    Self {
      registers: [0; 8],
      memory: Box::new(LinearMemory::new(8*1024)),
    }
  }

  pub fn get_register(&self, r: Register) -> u16 {
    self.registers[r as usize]
  }

  pub fn pop(&mut self) -> Result<u16, String> {
    let sp = self.registers[Register::SP as usize] - 2;
    if let Some(v) = self.memory.read2(sp) {
      self.registers[Register::SP as usize] -= 2;
      Ok(v)
    } else {
      Err(format!("memory read fault @ 0x{:X}", sp))
    }
  }

  pub fn push(&mut self, v: u16) -> Result<(), String> {
    let sp = self.registers[Register::SP as usize];
    if !self.memory.write2(sp, v) {
      return Err(format!("memory write fault @ 0x{:X}", sp));
    }

    self.registers[Register::SP as usize] += 2;
    Ok(())
  }

  pub fn step(&mut self) -> Result<(), String> {
    let pc = self.registers[Register::PC as usize];
    let instruction = self.memory.read2(pc).unwrap();
    self.registers[Register::PC as usize] = pc + 2;

    let op = parse_instruction(instruction)?;
    match op {
      Op::Nop => Ok(()),
      Op::Push(v) => {
        self.push(v.into());
        Ok(())
      },
      Op::PopRegister(r) => {
        let value = self.pop()?;
        self.registers[r as usize] = value;
        Ok(())
      },
      Op::AddStack => {
        let a = self.pop()?;
        let b = self.pop()?;
        self.push(a + b)
      },
      Op::AddRegister(r1, r2) => {
        self.registers[r1 as usize] += self.registers[r2 as usize];
        Ok(())
      },
    }
  }
}
