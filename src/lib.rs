use futures::{select, FutureExt};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn prompt(s: &str) -> String;
}

#[wasm_bindgen(module = "/io.js")]
extern "C" {
    fn detectButtonClickEvent(id: &str) -> Promise;
    fn cleanOutput();
    fn outputChar(c: char);
    fn outputStr(str: &str);
    fn showProgram(str: &str);
    fn markUpProgram(pragram: &str, idx: usize);
    fn cleanMemoryMap();
    fn getMemDisplayRadix() -> usize;
    fn writeMemoryMap(idx: usize, val: u8, radix: usize);
}

async fn detect_button_click_event(id: &str) {
    let _ = JsFuture::from(detectButtonClickEvent(id)).await;
}

// return (new_idx, is_overflow)
fn handle_inc_dec(idx: usize, min: usize, max: usize, is_inc: bool) -> (usize, bool) {
    if is_inc {
        if idx == max {
            (min, true)
        } else {
            (idx + 1, false)
        }
    } else {
        if idx == min {
            (max, true)
        } else {
            (idx - 1, false)
        }
    }
}

fn write_whole_memory_map(mem: &[u8], ptridx: usize, radix: usize) {
    mem.iter().enumerate().for_each(|(i, v)| {
        writeMemoryMap(i, *v, radix);
    });
    writeMemoryMap(ptridx, mem[ptridx], radix);
}

#[wasm_bindgen]
pub fn run(input: &str, bit: u32, allow_ptr_overflow: bool) {
    let mut cur = 0;
    let mut mem: [u8; 256] = [0; 256];
    let mut idx = 0;
    let mut output = String::new();
    let mut read_count = 0;
    cleanOutput();
    cleanMemoryMap();
    showProgram(input);

    loop {
        match input.chars().nth(cur) {
            Some('+') => {
                let (i, _) = handle_inc_dec(mem[idx] as usize, 0, 2usize.pow(bit) - 1, true);
                mem[idx] = i as u8;
                cur += 1;
            }
            Some('-') => {
                let (i, _) = handle_inc_dec(mem[idx] as usize, 0, 2usize.pow(bit) - 1, false);
                mem[idx] = i as u8;
                cur += 1;
            }
            Some('>') => {
                let (i, is_overflow) = handle_inc_dec(idx, 0, 255, true);
                if !allow_ptr_overflow && is_overflow {
                    alert("Error: Pointer Overflow.");
                    break;
                }
                idx = i;
                cur += 1;
            }
            Some('<') => {
                let (i, is_overflow) = handle_inc_dec(idx, 0, 255, false);
                if !allow_ptr_overflow && is_overflow {
                    alert("Error: Pointer Overflow.");
                    break;
                }
                idx = i;
                cur += 1;
            }
            Some('[') => {
                if mem[idx] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        cur += 1;
                        match input.chars().nth(cur) {
                            Some('[') => depth += 1,
                            Some(']') => depth -= 1,
                            None => break,
                            _ => (),
                        }
                    }
                }
                cur += 1;
            }
            Some(']') => {
                if mem[idx] != 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        cur -= 1;
                        match input.chars().nth(cur) {
                            Some('[') => depth -= 1,
                            Some(']') => depth += 1,
                            None => break,
                            _ => (),
                        }
                    }
                }
                cur += 1;
            }
            Some('.') => {
                output.push(mem[idx] as char);
                cur += 1;
            }
            Some(',') => {
                mem[idx] = prompt("Enter a single ASCII character")
                    .chars()
                    .nth(0)
                    .unwrap() as u8;
                cur += 1;
            }
            None => break,
            _ => {
                cur += 1;
            }
        }

        read_count += 1;
        if read_count == 100_000_000 {
            alert("Error: Too many instructions.");
            break;
        }
    }

    outputStr(&output);
    let radix = getMemDisplayRadix();
    write_whole_memory_map(&mem, idx, radix);
}

#[wasm_bindgen]
pub async fn step_run(input: &str, bit: u32, allow_overflow: bool) {
    let mut cur = 0;
    let mut mem: [u8; 256] = [0; 256];
    let mut idx = 0;
    cleanOutput();
    cleanMemoryMap();
    showProgram(input);

    loop {
        let radix = getMemDisplayRadix();

        markUpProgram(input, cur);

        match input.chars().nth(cur) {
            Some('+') => {
                let (i, _) = handle_inc_dec(mem[idx] as usize, 0, 2usize.pow(bit) - 1, true);
                mem[idx] = i as u8;
                write_whole_memory_map(&mem, idx, radix);
                cur += 1;
            }
            Some('-') => {
                let (i, _) = handle_inc_dec(mem[idx] as usize, 0, 2usize.pow(bit) - 1, false);
                mem[idx] = i as u8;
                write_whole_memory_map(&mem, idx, radix);
                cur += 1;
            }
            Some('>') => {
                let (i, is_overflow) = handle_inc_dec(idx, 0, 255, true);
                if !allow_overflow && is_overflow {
                    alert("Error: Pointer Overflow.");
                    break;
                }
                idx = i;
                write_whole_memory_map(&mem, idx, radix);
                cur += 1;
            }
            Some('<') => {
                let (i, is_overflow) = handle_inc_dec(idx, 0, 255, false);
                if !allow_overflow && is_overflow {
                    alert("Error: Pointer Overflow.");
                    break;
                }
                idx = i;
                write_whole_memory_map(&mem, idx, radix);
                cur += 1;
            }
            Some('[') => {
                if mem[idx] == 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        cur += 1;
                        match input.chars().nth(cur) {
                            Some('[') => depth += 1,
                            Some(']') => depth -= 1,
                            None => break,
                            _ => (),
                        }
                    }
                } else {
                    cur += 1;
                }
            }
            Some(']') => {
                if mem[idx] != 0 {
                    let mut depth = 1;
                    while depth > 0 {
                        cur -= 1;
                        match input.chars().nth(cur) {
                            Some('[') => depth -= 1,
                            Some(']') => depth += 1,
                            None => break,
                            _ => (),
                        }
                    }
                } else {
                    cur += 1;
                }
            }
            Some('.') => {
                outputChar(mem[idx] as char);
                cur += 1;
            }
            Some(',') => {
                mem[idx] = prompt("Enter a single ASCII character")
                    .chars()
                    .nth(0)
                    .unwrap() as u8;
                write_whole_memory_map(&mem, idx, radix);
                cur += 1;
            }
            None => break,
            _ => {
                cur += 1;
            }
        }

        select! {
            _ = detect_button_click_event("next").fuse() => (),
            _ = detect_button_click_event("terminate").fuse() => break,
        }
    }
}
