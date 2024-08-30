use crate::Token::{Dec, In, Inc, Left, Loop, Out, Right};

fn main() {
    let mut cursor = 0usize;
    println!("{:?}",lex_parse("[+]",&mut cursor));
}




fn lex_parse(s:&str,i:&mut usize)->Vec<Token>{
    let mut result = vec![];
    let mut v:Vec<char> = s.chars().collect();
    loop {
        if *i == v.len(){
            break;
        }
        match v[*i] {
            '+' => result.push(Inc),
            '-' =>result.push(Dec),
            '>' =>result.push(Right),
            '<' =>result.push(Left),
            ',' =>result.push(In),
            '.' =>result.push(Out),
            '[' =>  { let mut inr = 1usize;
                result.push(Loop(lex_parse(&s[*i..],&mut inr)));
                *i+=inr;
            },
            ']' => {return result }
            _ => {}

        }
        *i+= 1;
    }


    result
}
#[derive(Debug)]
enum Token{
    Inc,
    Dec,
    Right,
    Left,
    In,
    Out,
    Loop(Vec<Token>)

}