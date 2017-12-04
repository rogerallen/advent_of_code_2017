use std::env;
use std::process;

fn main() {
   let args: Vec<String> = env::args().collect();
   println!("Advent of Code 2017");
   if args.len() != 2 {
      usage();
   }
   println!("Args: {:?}", &args);
   let day = &args[1];
   match day as &str {
     "1" => { day01(); }
     _ => { usage(); }
   }
}

fn usage() {
   let args: Vec<String> = env::args().collect();
   println!("Usage: {} <day>",args[0]);
   process::exit(1);
}

// ======================================================================
// Day 1
// ======================================================================

fn day01() {
   println!("Day 1");
}
