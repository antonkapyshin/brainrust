use nom;
use crate::structs;

fn get_lexemes(input: &str) -> nom::IResult<&str, Vec<char>> {
    nom::multi::many1(nom::character::complete::anychar)(input)
}

fn get_construct(lexemes: &Vec<char>, index: usize) -> (Option<structs::SimpleConstruct>, usize) {
    let mut matched = 0;
    for i in index..lexemes.len() {
        matched += 1;
        match lexemes[i] {
            '[' => {
                let mut body = Vec::new();
                for j in i+1..lexemes.len() {
                    let current = lexemes[j];
                    if current == ']' {
                        matched += 1;
                        break;
                    }
                    let (inner_construct, inner_matched) = get_construct(lexemes, j);
                    matched += inner_matched;
                    body.push(inner_construct.expect("Expected closing ]"));
                }
                return (Some(structs::SimpleConstruct::Loop(body)), matched);
            }
            '<' => return (Some(structs::SimpleConstruct::LeftShift), matched),
            '>' => return (Some(structs::SimpleConstruct::RightShift), matched),
            '+' => return (Some(structs::SimpleConstruct::Increment), matched),
            '-' => return (Some(structs::SimpleConstruct::Decrement), matched),
            ',' => return (Some(structs::SimpleConstruct::GetChar), matched),
            '.' => return (Some(structs::SimpleConstruct::PutChar), matched),
            _ => {}
        }
    };
    (None, matched)
}

pub fn parse(input: &str) -> Vec<structs::SimpleConstruct> {
    let mut result = Vec::new();
    let mut index = 0;
    let lexemes = get_lexemes(input).expect("Could not parse.").1;
    loop {
        let (construct, matched) = get_construct(&lexemes, index);
        index += matched;
        match construct {
            Some(c) => {
                result.push(c);
            }
            None => {
                break
            }
        };
    }
    result
}
