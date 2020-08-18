use std::error::Error;

#[derive(Debug, Clone)]
pub enum LexItem<'a> {
    Paren(Vec<LexItem<'a>>),
    Op(&'a str),
    Val(&'a str),
}

use std::str::Chars;
#[derive(Clone)]
struct PeekableChars<'a> {
    txt: &'a str,
    chars: Chars<'a>,
    curr: Option<char>,
}

impl<'a> PeekableChars<'a> {
    pub fn peek(&mut self) -> Option<&char> {
        self.curr.as_ref()
    }
    pub fn next(&mut self) -> Option<char> {
        let tmp = self.curr;
        self.txt = self.chars.as_str();
        self.curr = self.chars.next();
        tmp
    }
    pub fn nth(&mut self, n: usize) -> Option<char> {
        let mut res = self.curr;
        for _ in 0..=n {
            res = self.next();
        }
        res
    }
    pub fn as_str(&self) -> &'a str {
        self.txt
    }
}
impl<'a> From<Chars<'a>> for PeekableChars<'a> {
    fn from(mut c: Chars<'a>) -> Self {
        Self {
            txt: c.as_str(),
            curr: c.next(),
            chars: c,
        }
    }
}

mod tests {
    use super::*;
    const TEST_TXT: &str = "this is a test string";

    #[test]
    fn test_peekable_char_next() {
        let mut pc = PeekableChars::from(TEST_TXT.chars());
        assert_eq!(pc.next(), Some('t'));
        assert_eq!(pc.next(), Some('h'));
        assert_eq!(pc.next(), Some('i'));
        assert_eq!(pc.next(), Some('s'));
        assert_eq!(pc.as_str(), &TEST_TXT[4..]);
        assert_eq!(pc.peek(), Some(&' '));
    }
    #[test]
    fn test_peekable_char_nth() {
        let mut pc = PeekableChars::from(TEST_TXT.chars());
        assert_eq!(pc.nth(5), Some('i'));
        assert_eq!(pc.as_str(), &TEST_TXT[6..]);
        assert_eq!(pc.peek(), Some(&'s'));
    }
}

pub fn lex<'a>(input: &'a str) -> Result<Vec<LexItem<'a>>, Box<dyn Error>> {
    let mut lexed = Vec::new();
    let mut chars = PeekableChars::from(input.chars());

    while chars.peek().is_some() {
        if eat_whitespace(&mut chars) {
            return Err(format!("Value expected: {}", chars.as_str()).into());
        }
        if chars.peek().unwrap_or(&'a') == &'(' {
            let end = find_matching_prenthese(chars.as_str())?;
            let inside_par = &chars.as_str()[1..end];
            let inner_lexed = match lex(inside_par) {
                e @ Err(_) => return e,
                Ok(l) => l,
            };
            lexed.push(LexItem::Paren(inner_lexed));
            chars.nth(end);
        } else {
            let chars_old = chars.clone();
            let mut len = 0;
            loop {
                match chars.peek() {
                    Some(c) if c.is_alphanumeric() => len += c.len_utf8(),
                    _ => break,
                };
                chars.next();
            }
            if len == 0 {
                return Err(format!("number needed here: {}", chars_old.as_str()).into());
            }
            lexed.push(LexItem::Val(&chars_old.as_str()[..len]));
        }
        if eat_whitespace(&mut chars) {
            break;
        }
        let op_len = if chars.as_str().starts_with("**") {
            2
        } else {
            1
        };
        let op = {
            let tmp = &chars.txt[..op_len];
            chars.nth(op_len - 1);
            tmp
        };
        lexed.push(LexItem::Op(op));
    }
    Ok(lexed)
}

fn find_matching_prenthese(slice: &str) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for (_i, c) in slice.as_bytes().iter().enumerate() {
        match c {
            b'(' => count += 1,
            b')' => count -= 1,
            _ => (),
        }
        if count == 0 {
            let c: *const u8 = c;
            return Ok(c as usize - slice.as_ptr() as usize);
        }
    }
    Err(format!("No matching closing parenthese: `{}`", slice).into())
}

fn eat_whitespace(chars: &mut PeekableChars) -> bool {
    loop {
        match chars.peek() {
            None => return true,
            Some(c) if !c.is_whitespace() => return false,
            _ => chars.next(),
        };
    }
}
