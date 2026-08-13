use std::io::Read;
pub fn run_bf(src: &str) -> Result<Vec<char>, String> {
    let mut input = std::io::stdin().bytes();
    let src_bytes = src.as_bytes();
    let mut mem :Vec<u8> = Vec::new();
    mem.push(0);
    let mut index = 0;
    let mut line: usize = 0;
    let mut ptr: u8;
    let mut output: Vec<char> = Vec::new();
    while line < src_bytes.len(){
        ptr = src_bytes[line];
        if ptr == 62 {
            index += 1;
            if index >= mem.len() {
                mem.push(0);
            }
        } else if ptr == 60 {
            if index == 0{
                return Err("the index is negative.".to_string());
            }
            index -= 1;
            if (mem[index+1] == 0) && (mem.len() == index + 2) {
                mem.pop();
            }
        } else if ptr == 43 {
            mem[index] = mem[index].wrapping_add(1);
        } else if ptr == 45 {
            mem[index] = mem[index].wrapping_sub(1);
        } else if ptr == 46 {
            output.push(mem[index] as char);
        } else if ptr == 44 {
            mem[index] = match input.next() {
                Some(Ok(byte)) => byte,
                _ => 0,
            };
        } else if ptr == 91 {
            if mem[index] == 0 {
                let mut nesting = 1;
                let tmpline = line;
                while nesting > 0 {
                    line += 1;
                    if line >= src_bytes.len() {
                        return Err(format!("Unexpected token \"[\" at the character {}. ", tmpline+1).to_string());
                    }
                    if src_bytes[line] == 91 {
                        nesting += 1;
                    } else if src_bytes[line] == 93 {
                        nesting -= 1;
                    }
                }
            }
        } else if ptr == 93 {
            if mem[index] != 0 {
                let mut nesting = 1;
                let tmpline = line;
                while nesting > 0 {
                    if line == 0 && nesting != 0{
                        return Err(format!("Unexpected token \"]\" at the character {}.", tmpline+1).to_string());
                    }
                    line -= 1;
                    if src_bytes[line] == 93 {
                        nesting += 1;
                    } else if src_bytes[line] == 91 {
                        nesting -= 1;
                    }
                }
            }
        }
        line += 1;
    }
    output.push('\n');
    Ok(output)
}



pub fn run_bf_with_input(src: &str, input: Vec<char>) -> Result<Vec<char>, String> {
    let src_bytes = src.as_bytes();
    let mut mem :Vec<u8> = Vec::new();
    mem.push(0);
    let mut index = 0;
    let mut line: usize = 0;
    let mut ptr: u8;
    let mut output: Vec<char> = Vec::new();
    let mut input_index = 0;
    while line < src_bytes.len(){
        ptr = src_bytes[line];
        if ptr == 62 {
            index += 1;
            if index >= mem.len() {
                mem.push(0);
            }
        } else if ptr == 60 {
            if index == 0{
                return Err("the index is negative.".to_string());
            }
            index -= 1;
            if (mem[index+1] == 0) && (mem.len() == index + 2) {
                mem.pop();
            }
        } else if ptr == 43 {
            mem[index] = mem[index].wrapping_add(1);
        } else if ptr == 45 {
            mem[index] = mem[index].wrapping_sub(1);
        } else if ptr == 46 {
            output.push(mem[index] as char);
        } else if ptr == 44 {
            mem[index] = if input_index < input.len() {
                let byte = input[input_index] as u8;
                input_index += 1;
                byte
            } else {
                0
            };
        } else if ptr == 91 {
            if mem[index] == 0 {
                let mut nesting = 1;
                let tmpline = line;
                while nesting > 0 {
                    line += 1;
                    if line >= src_bytes.len() {
                        return Err(format!("Unexpected token \"[\" at the character {}. ", tmpline+1).to_string());
                    }
                    if src_bytes[line] == 91 {
                        nesting += 1;
                    } else if src_bytes[line] == 93 {
                        nesting -= 1;
                    }
                }
            }
        } else if ptr == 93 {
            if mem[index] != 0 {
                let mut nesting = 1;
                let tmpline = line;
                while nesting > 0 {
                    if line == 0 && nesting != 0{
                        return Err(format!("Unexpected token \"]\" at the character {}.", tmpline+1).to_string());
                    }
                    line -= 1;
                    if src_bytes[line] == 93 {
                        nesting += 1;
                    } else if src_bytes[line] == 91 {
                        nesting -= 1;
                    }
                }
            }
        }
        line += 1;
    }
    output.push('\n');
    Ok(output)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_world() {
        let src = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let out = run_bf_with_input(src, vec![]).unwrap().into_iter().collect::<String>();
        assert_eq!(out, "Hello World!\n\n");
    }
}
