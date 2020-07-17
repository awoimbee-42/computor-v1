use std::error::Error;

#[derive(Debug, Clone)]
pub enum LexItem<'a> {
    Paren(Vec<LexItem<'a>>),
    Op(&'a str),
    Val(&'a str),
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
    Err("No matching closing parenthese".into())
}

fn eat_whitespace(index: &mut usize, bytes: &[u8]) {
    while *index < bytes.len() && bytes[*index] <= b' ' {
        *index += 1
    }
}
pub fn lex<'a>(input: &'a str) -> Vec<LexItem<'a>> {
    let mut lexed = Vec::new();

    let bytes = input.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        eat_whitespace(&mut index, bytes);
        if matches!(bytes[index], b'(') {
            let end = index + find_matching_prenthese(&input[index..]).unwrap();
            let inside_par = &input[index + 1..end];
            lexed.push(LexItem::Paren(lex(inside_par)));
            index = end + 1;
        } else {
            let start = index;
            index += 1;
            while index < bytes.len() && matches!(bytes[index], b'0'..=b'9' | b'e' | b'E' | b'.') {
                index += 1
            }
            lexed.push(LexItem::Val(&input[start..index]));
        }
        eat_whitespace(&mut index, bytes);
        if index == bytes.len() {
            break;
        };
        let op = if bytes[index] == b'*' && bytes[index + 1] == b'*' {
            index += 2;
            &input[index - 2..index]
        } else {
            index += 1;
            &input[index - 1..index]
        };
        lexed.push(LexItem::Op(op));
    }
    lexed
}
