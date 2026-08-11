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
            if mem[index] == 255{
                mem[index] = 0;
            }else {
                mem[index] += 1;
            }
        } else if ptr == 45 {
            if mem[index] == 0{
                mem[index] = 255;
            }else {
                mem[index] -= 1;
            }
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
                while nesting > 0 {
                    line += 1;
                    if line >= src_bytes.len() {
                        break;
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
                while nesting > 0 {
                    if line == 0 {
                        break;
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
