// compute
// Copyright 2018 AnonymousDapper

extern crate getopts;
extern crate rustyline;

mod component;

use std::io;
use std::{mem, env, process, fs};
use std::io::prelude::*;

use getopts::Options;

use rustyline::Editor;
use rustyline::error::ReadlineError;

use component::{logger, scanner};

// print version information
fn version() {
  println!("compute 0.0.1b (02-21-18)");
  process::exit(0)
}

// print usage information
fn usage() {
  println!("
    Usage: compute [options] [source_file.cpts]

    Options:
        -V --version   print version info and exit
        -h --help      print this message and exit
        -v --verbose   enable verbose (debugging) output"
  );
  process::exit(1);
}

// lets get out of here
fn quit(text: String) -> ! {
  println!("{}", text);
  process::exit(1);
}


struct Compute {
  errored: bool
}

impl Compute {
  fn new() -> Compute {
    Compute {
      errored: false
    }
  }

  // print error report
  fn error(&mut self, msg: String) {
    println!("Error: {}", msg);
    self.errored = true;
  }

  // start the run process
  fn run(&mut self, src: String) {
    println!("Input: '{}'", src);

    let scanner = scanner::Scanner::new(src);
    let tokens = scanner.scan();

    for token in tokens.iter() {
      println!("{}", token);
    };
  }

  // run from source
  fn run_file(mut self, path: String) {
    let mut file = match fs::File::open(path) {
      Ok(f) => f,
      Err(msg) => quit(msg.to_string())
    };

    let mut source = String::new();
    match file.read_to_string(&mut source) {
      Ok(_a) => { },
      Err(msg) => quit(msg.to_string())
    };

    mem::drop(file);
    self.run(source);

    if self.errored {
      process::exit(65);
    }
  }

  // run from terminal
  fn run_prompt(mut self) {
    let mut editor = Editor::<()>::new();

    loop {
      match editor.readline("compute => ") {
        Ok(line) => {
          editor.add_history_entry(&line);
          self.run(line);
        },
        Err(ReadlineError::Interrupted) => quit("Ctrl-C pressed".to_string()),
        Err(ReadlineError::Eof) => quit("Ctrl-D pressed".to_string()),
        Err(err) => {
          self.error(format!("{:?}", err));
          break
        }
      };

      self.errored = false;
    }
  }

}

// main
fn main() {
  let args: Vec<String> = env::args().collect();
  let mut opts = Options::new();

  let compute = Compute::new();

  opts.optflag("V", "version", "print version info and exit")
      .optflagmulti("v", "verbose", "enable verbose (debugging) output")
      .optflag("h", "help", "print help message and exit");

  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(msg) => quit(msg.to_string())
  };

  let verbosity: usize;
  let source_file: Option<String>;

  if matches.opt_present("V") {
    version();
  }

  if matches.opt_present("h") {
    usage();
  }

  if matches.opt_present("v") {
    verbosity = matches.opt_count("v");
  } else {
    verbosity = 0;
  }

  source_file = if !matches.free.is_empty() {
    Some(matches.free[0].clone())
  } else {
    None
  };

  if source_file == None {
    compute.run_prompt();
  } else {
    compute.run_file(source_file.unwrap());
  }

  // logger::error("idk what happened", 4, 12);
  // logger::warn("whats it this time", 1, 6);
  // logger::log("more news", 0, 0);
  // logger::debug("oh, just some testing", 500, 500);
}