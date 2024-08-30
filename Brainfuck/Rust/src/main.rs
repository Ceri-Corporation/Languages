use std::io::{Read, stdin};
use crate::Token::{Dec, In, Inc, Left, Loop, Out, Right};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = std::fs::read_to_string(&args[0]).unwrap();
    interpret(lex_parse(&code));
}


fn interpret(code:Vec<Token>){
    let mut mem = [0;30000];
    let mut cursor = 0;
    let mut input = if code.contains(&In) {
        let mut input_alt = String::new();
        stdin().read_line(&mut input_alt).unwrap();
        input_alt.bytes().collect()
    } else {
        vec![]
    };
    int_all(&mut mem, &mut cursor, &code, &mut input);



}
fn int_all(mem:&mut [u8;30000],cursor:&mut usize,code:&Vec<Token>,input:&mut Vec<u8> ){
    for c in code {
        match c {
            Inc => mem[*cursor] = mem[*cursor].wrapping_add(1),
            Dec => mem[*cursor] = mem[*cursor].wrapping_sub(1),
            Right => *cursor = (*cursor +1) % 30000,
            Left => *cursor = (*cursor + 29999) %30000,
            In => {mem[*cursor] = input[0]; mem[*cursor] = input.remove(0);}
            Out => {print!("{}",mem[*cursor] as char)}
            Loop(c) => {
                while mem[*cursor] != 0 {
                    int_all(mem,cursor,c,input)
                }}
        }
    }
}

fn lex_parse(s:&str)->Vec<Token>{
    let mut cursor = 0;
    lex_parse_alt(s,&mut cursor)
}
fn lex_parse_alt(s:&str,i:&mut usize)->Vec<Token>{
    let mut result = vec![];
    let  v:Vec<char> = s.chars().collect();
    while *i == v.len(){

        match v[*i] {
            '+' => result.push(Inc),
            '-' =>result.push(Dec),
            '>' =>result.push(Right),
            '<' =>result.push(Left),
            ',' =>result.push(In),
            '.' =>result.push(Out),
            '[' =>  { let mut inr = 1;
                result.push(Loop(lex_parse_alt(&s[*i..],&mut inr)));
                *i+=inr;
            },
            ']' => return result ,
            _ => {}

        }
        *i+= 1;
    }


    result
}
#[derive(Debug,PartialEq)]
enum Token{
    Inc,
    Dec,
    Right,
    Left,
    In,
    Out,
    Loop(Vec<Token>)

}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn lop(){
        assert_eq!(lex_parse("[+]><.,-"),vec![Loop(vec![Inc]),Right,Left,Out,In,Dec])
    }
}