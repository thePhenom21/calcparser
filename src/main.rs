use std::io;
use std::io::{BufReader, read_to_string};
use std::iter::Scan;
use std::ops::Add;
use std::process::exit;
use std::str::Chars;
use crate::TokenType::{DIGIT, EOF, OPERATOR};

#[derive(PartialEq)]
enum TokenType {
    DIGIT,
    OPERATOR,
    EOF,
}

struct Token {
    t : TokenType,
    value : u32
}

fn main() {
    loop {
        let mut s = String::new();
        _ = io::stdin().read_line(&mut s);
        let size = s.len();
        let mut ch = s.chars();

        let mut p = 0;

        let mut current_token = next_token(&mut ch,size,p);

        let mut left = current_token.1;
        if left.t != DIGIT  {
            panic!("parse error");
        }

        current_token = next_token(&mut ch,size,current_token.0);

        let mut op = current_token.1;
        if op.t != OPERATOR {
            panic!("parse error")
        }

        current_token = next_token(&mut ch,size,current_token.0);

        let mut right = current_token.1;
        if right.t != DIGIT {
            panic!("parse error")
        }

        println!("{}",left.value + right.value);

    }

}



fn next_token(c: &mut Chars, size : usize, mut p: usize) -> (usize,Token){

    if p >= size {
        exit(0);
    }

    let ch = c.next().unwrap();

    if !ch.is_digit(10) && ch != '+' {
        return next_token(c,size,p+1);
    }

    if ch.is_digit(10) {
        return (p+1,Token{t:DIGIT,value:ch.to_digit(10).unwrap()});
    }

    if ch == '+' {
        return (p+1,Token{t:OPERATOR,value:0});
    }

    if ch == ';'{
        return (p+1,Token{t:EOF,value:0});
    }

    panic!("Not parseable");
}



