extern crate rs9cc;

use std::env;
use std::error::Error;
use std::process;

use rs9cc::token::*;
use rs9cc::node::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("{:?}", args);
        eprintln!("引数の個数が正しくありません");
        if args.len() != 2 {
            eprintln!("{:?}", args);
            eprintln!("引数の個数が正しくありません");
            loop {
                eprintln!("結合しますか？ [y/n]");
                let s = {
                    let mut s = String::new(); // バッファを確保
                    std::io::stdin().read_line(&mut s).unwrap(); // 一行読む。失敗を無視
                    s.trim().to_owned() // 改行コードが末尾にくっついてくるので削る
                };
    
                if &s == "y" {
                    args = vec![args[0].clone(), args[1..].join("")];
                    break;
                }
                process::exit(1);
            }
        }
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