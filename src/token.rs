use std::iter::Peekable;

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
                        val.push(byte);
                        break;
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
    }
}

pub trait TokenExt {
    fn tokenize(&self) -> TokenIter;
}

impl TokenExt for String {
    fn tokenize(&self) -> TokenIter {
        TokenIter { s: self.to_owned() }
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