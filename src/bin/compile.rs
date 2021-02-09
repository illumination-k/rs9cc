extern crate rs9cc;

use std::env;
use std::error::Error;
use std::process;

use rs9cc::token::*;
use rs9cc::node::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }
    let formula = args[1].clone();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    
    let mut tokenizer = formula.tokenize().peekable();

    let node = expr(&mut tokenizer);
    gen(&node);
    println!("  pop rax");
    println!("  ret");
    Ok(())
}