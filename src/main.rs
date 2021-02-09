use std::env;
use std::error::Error;
use std::process;

mod node;
mod token;

use crate::token::*;
use crate::node::*;

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
    // let first = tokenizer.next();
    // match first {
    //     Some(t) => {
    //         match t.token_kind {
    //             TokenKind::TkNum => {
    //                 println!("  mov rax, {}", t.val)
    //             },
    //             TokenKind::TkReserved => { panic!("最初は数字である必要があります")}
    //         }
    //     },
    //     None => {panic!("文字を入力してください")}
    // }

    // loop {
    //     if tokenizer.peek().is_none() { break; }
        
    //     if consume("+", &mut tokenizer) {
    //         println!("  add rax, {}", tokenizer.next().unwrap().val)
    //     } else if consume("-", &mut tokenizer) {
    //         println!("  sub rax, {}", tokenizer.next().unwrap().val) 
    //     } else {
    //         panic!("not supported format!")
    //     }
    // }
    let node = expr(&mut tokenizer);
    gen(&node);
    println!("  pop rax");
    println!("  ret");
    Ok(())
}