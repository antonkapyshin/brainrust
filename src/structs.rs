use std::io::{Write, Read};

pub enum SimpleConstruct {
    LeftShift,
    RightShift,
    Increment,
    Decrement,
    GetChar,
    PutChar,
    Loop(Vec<SimpleConstruct>)
}

impl SimpleConstruct {
    pub fn interpret(self: &Self, arr: &mut [u8; 30000], index: &mut usize) {
        match self {
            Self::LeftShift => {
                *index -= 1;
            }
            Self::RightShift => {
                *index += 1;
            }
            Self::Increment => {
                arr[*index] += 1;
            }
            Self::Decrement => {
                arr[*index] -= 1;
            }
            Self::GetChar => {
                let char = std::io::stdin().bytes().next().and_then(|result| result.ok());
                arr[*index] = char.expect("Could not read a char.");
            }
            Self::PutChar => {
                match std::io::stdout().write(&arr[*index..=*index]) {
                    Ok(written) => {
                        assert_eq!(written, 1);
                    }
                    Err(_) => {
                        panic!("Could not write a char.");
                    }
                };
            }
            Self::Loop(body) => {
                loop {
                    if arr[*index] == 0 {
                        break;
                    }
                    for elem in body.iter() {
                        elem.interpret(arr, index);
                    }
                }
            }
        }
    }
}
