use std::env;
use std::error::Error;
use std::process;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    TkReserved,
    TkNum,
}
#[derive(Clone)]
pub struct Token {
    token_kind: TokenKind,
    val: String,
}

impl Token {
    pub fn new(token_kind: TokenKind, val: String) -> Self {
        Self {
            token_kind,
            val
        }
    }
}

pub struct TokenIter {
    s: String,
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // trim whitespace etc.,
        self.s = self.s.trim().to_string();
        // dbg!(&self.s);
        
        if self.s.is_empty() {
            return None
        }

        let mut bytes = std::collections::VecDeque::from(self.s.to_owned().as_bytes().to_vec());
        
        let mut val = vec![];
        let token_kind = if bytes[0].is_ascii_digit() {
            TokenKind::TkNum
        } else {
            TokenKind::TkReserved
        };

        match token_kind {
            TokenKind::TkNum => {
                while let Some(byte) = bytes.pop_front() {
                    if byte.is_ascii_digit() {
                        val.push(byte)
                    } else {
                        bytes.push_front(byte);
                        break;
                    }
                }
            },
            TokenKind::TkReserved => {
                while let Some(byte) = bytes.pop_front() {
                    if !byte.is_ascii_digit() && byte != b' ' {
                        val.push(byte)
                    } else {
                        bytes.push_front(byte);
                        break;
                    }
                }
            }
        }

        unsafe {
            self.s = String::from_utf8_unchecked(bytes.into_iter().collect());
            Some(Token::new(token_kind, String::from_utf8_unchecked(val)))
        }
        // if bytes[0].is_ascii_digit() {
        //     let mut val: Vec<u8> = vec![];
        //     let mut idx = 0;
        //     for i in 0..bytes.len() {
        //         if !bytes[i].is_ascii_digit() { idx = i + 1; break; }                
        //         val.push(bytes[i]);

        //         if i == bytes.len() - 1 {
        //             idx = i + 1
        //         }
        //     }
            
        //     self.s = unsafe {
        //         if bytes.len() > idx {
        //             String::from_utf8_unchecked(bytes[idx..].to_vec())
        //         } else {
        //             String::new()
        //         }
        //     };
        //     Some(Token::new(TokenKind::TkNum, unsafe {
        //         String::from_utf8_unchecked(val.to_vec())
        //     }))
        // } else {
        //     self.s = if bytes.len() > 1 {
        //         unsafe { String::from_utf8_unchecked(bytes[1..].to_vec()) }
        //     } else {
        //         String::new()
        //     };
        //     Some(Token::new(TokenKind::TkReserved, unsafe {
        //         String::from_utf8_unchecked([bytes[0]].to_vec())
        //     }))
        // }
    }
}

trait TokenExt {
    fn tokenize(&self) -> TokenIter;
}

impl TokenExt for String {
    fn tokenize(&self) -> TokenIter {
        TokenIter { s: self.to_owned() }
    }
}

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
    
    let mut tokenizer = formula.tokenize();
    let first = tokenizer.next();
    match first {
        Some(t) => {
            match t.token_kind {
                TokenKind::TkNum => {
                    println!("  mov rax, {}", t.val)
                },
                TokenKind::TkReserved => { panic!("最初は数字である必要があります")}
            }
        },
        None => {panic!("文字を入力してください")}
    }

    loop {
        match tokenizer.next() {
            Some(t) => {
                if t.val == "+" {
                    println!("  add rax, {}", tokenizer.next().unwrap().val)
                } else {
                    println!("  sub rax, {}", tokenizer.next().unwrap().val) 
                }
            },
            None => {break;}
        }
    }
    println!("  ret");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let s = "13 + 2 - 3".to_string();
        let vals = vec!["13", "+", "2", "-", "3"].iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let kinds = vec![TokenKind::TkNum, TokenKind::TkReserved, TokenKind::TkNum, TokenKind::TkReserved, TokenKind::TkNum];
        
        let mut dvals = vec![];
        let mut dkinds = vec![];
        for t in s.tokenize() {
            dvals.push(t.val);
            dkinds.push(t.token_kind)
        }

        assert_eq!(vals, dvals);
        assert_eq!(kinds, dkinds);
    }
}