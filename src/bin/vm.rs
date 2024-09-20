use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, Read};

use vm::{Machine, Register};

fn signal_halt(vm: &mut Machine) -> Result<(), String> {
  vm.halt = true;
  Ok(())
}

pub fn main() -> Result<(), String> {
  /*
   * PUSH 10
   * PUSH 8
   * ADDSTACK
   * POPREGISTER A
   */
  // vm.memory.write(0, 0x1);
  // vm.memory.write(1, 10);
  // vm.memory.write(2, 0x1);
  // vm.memory.write(3, 8);
  // vm.memory.write(4, 0x3);
  // vm.memory.write(6, 0x2);
  // vm.memory.write(7, 0);

  let args: Vec<_> = env::args().collect();
  if args.len() != 2 { panic!("usage: {} <input>", args[0]); }

  let file = File::open(Path::new(&args[1])).map_err(|e| format!("failed to open: {}", e))?;

  let mut reader = BufReader::new(file);
  let mut program: Vec<u8> = Vec::new();
  reader.read_to_end(&mut program).map_err(|e| format!("read: {}", e))?;

  let mut vm = Machine::new();
  vm.define_handler(0xf0, signal_halt);
  vm.memory.load_from_vec(&program, 0);

  while !vm.halt {
    vm.step()?;
  }

  println!("A = {}", vm.get_register(Register::A));
  Ok(())
}
