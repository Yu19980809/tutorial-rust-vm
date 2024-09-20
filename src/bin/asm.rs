use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufReader, BufRead, Write};

fn main() -> Result<(), String> {
  let args: Vec<_> = env::args().collect();

  if args.len() != 2 {
    panic!("usage: {} <input>", args[0]);
  }

  let file = File::open(Path::new(&args[1]))
    .map_err(|e| format!("failed to open: {}", e))?;

  /*
   *  for each line in file
   *    for each space aeparated token
   *      try to parse as base-16 number
   *      append to output if number
   *      else die 
   */
  let mut output: Vec<u8> = Vec::new();
  
  for line in BufReader::new(file).lines() {
    let line_inner = line.map_err(|_e| "foo")?;

    for t in line_inner.split(" ").filter(|x| x.len() > 0) {
      let b = u8::from_str_radix(t, 16).map_err(|e| format!("parse int: {}", e))?;
      output.push(b);
    }
  }

  let mut stdout = io::stdout().lock();
  stdout.write_all(&output).map_err(|e| format!("{}", e))?;
  Ok(())
}
