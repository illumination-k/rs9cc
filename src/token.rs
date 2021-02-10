use std::iter::Peekable;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct OpWords {
    op_words: HashSet<String>,
    max_length: usize,
}

impl OpWords {
    fn new(op_words: Vec<&str>) -> Self {
        let max_length = op_words.iter().map(|x| x.len()).max().unwrap();
        Self {
            op_words: op_words.into_iter().map(|x| x.to_string()).collect(),
            max_length: max_length,
        }
    }

    fn contains(&self, x: &str) -> bool {
        self.op_words.contains(x)
    }

    fn contains_u8(&self, x: &[u8]) -> bool {
        unsafe { self.contains(String::from_utf8_unchecked(x.to_vec()).as_ref()) }
    }


    fn ops(&self, len: usize) -> Vec<String> {
        self.op_words.iter().filter(|x| x.len() == len).map(|x| x.clone()).collect()
    }
}

impl Default for OpWords {
    fn default() -> Self {
        let op_words = vec!["+", "-", "/", "*", "==", "=!", ">=", "<=", "<", ">", "(", ")"];
        OpWords::new(op_words)
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    TkReserved,
    TkNum,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_kind: TokenKind,
    pub val: String,
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
    op_words: OpWords,
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
                // +-2
                // bytes [+-2] vec []
                // bytes [-2]  vec [+]
                // bytes [2] vec [+-]
                // break
                while let Some(byte) = bytes.pop_front() {
                    if !byte.is_ascii_digit() && byte != b' ' {
                        val.push(byte);
                        // break;
                    } else {
                        bytes.push_front(byte);
                        break;
                    }
                }
                //val.reverse();
                let mut now_length = self.op_words.max_length;
                while now_length > 0 {
                    if now_length <= val.len() {
                        // dbg!(unsafe {String::from_utf8_unchecked(val[..now_length].to_owned().to_vec())});
                        if self.op_words.contains_u8(&val[..now_length]) {
                            for &v in val[now_length..].iter().rev() {
                                bytes.push_front(v)
                            }
                            val = val[..now_length].to_vec();
                            break;
                        }
                    }
                    now_length -= 1;
                }
            }
        }

        unsafe {
            self.s = String::from_utf8_unchecked(bytes.into_iter().collect());
            Some(Token::new(token_kind, String::from_utf8_unchecked(val)))
        }
    }
}

pub trait TokenExt {
    fn tokenize(&self) -> TokenIter;
}

impl TokenExt for String {
    fn tokenize(&self) -> TokenIter {
        TokenIter {
            s: self.to_owned(),
            op_words: Default::default(),
        }
    }
}


pub fn consume(op: &str, iter: &mut Peekable<TokenIter>) -> bool {
    match iter.peek() {
        Some(t) => {
            if t.token_kind == TokenKind::TkReserved && &t.val == op {
                iter.next();
                true
            } else {
                false
            }
        },
        None => false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_op_words() {
        let mut op_words: OpWords = Default::default();
        assert!(op_words.contains("*"));
        assert!(op_words.contains("("));
        assert!(op_words.contains_u8(b"("));
        assert!(!op_words.contains_u8(b"!=)"));
    }
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