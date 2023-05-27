use futures::{select, FutureExt};
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    fn prompt(s: &str) -> String;
}

#[wasm_bindgen(module = "/io.js")]
extern "C" {
    fn detectButtonClickEvent(id: &str) -> Promise;
    fn cleanConsole();
    fn outputChar(c: char);
    fn outputStr(str: &str);
    fn showProgram(str: &str);
    fn markUpProgram(pragram: &str, idx: usize);
    fn cleanMemoryMap();
    fn writeMemoryMap(idx: usize, val: u8);
}

async fn detect_button_click_event(id: &str) {
    let _ = JsFuture::from(detectButtonClickEvent(id)).await;
}

fn handle_inc_dec(idx: usize, min: usize, max: usize, is_inc: bool) -> usize {
    if is_inc {
        if idx == max {
            min
        } else {
            idx + 1
        }
    } else {
        if idx == min {
            max
        } else {
            idx - 1
        }
    }
}

#[wasm_bindgen]
pub fn run(input: &str) {
    let mut cur = 0;
    let mut mem: [u8; 256] = [0; 256];
    let mut idx: usize = 0;
    let mut output = String::new();
    cleanConsole();
    cleanMemoryMap();
    showProgram(input);

    loop {
        match input.chars().nth(cur) {
            Some('+') => {
                mem[idx] = handle_inc_dec(mem[idx] as usize, 0, 255, true) as u8;
                cur += 1;
            }
            Some('-') => {
                mem[idx] = handle_inc_dec(mem[idx] as usize, 0, 255, false) as u8;
                cur += 1;
            }
            Some('>') => {
                idx = handle_inc_dec(idx, 0, 255, true);
                cur += 1;
            }
            Some('<') => {
                idx = handle_inc_dec(idx, 0, 255, false);
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
    }

    outputStr(&output);
    mem.iter().enumerate().for_each(|(i, v)| {
        writeMemoryMap(i, *v);
    });
    writeMemoryMap(idx, mem[idx]);
}

#[wasm_bindgen]
pub async fn step_run(input: &str) {
    let mut cur = 0;
    let mut mem: [u8; 256] = [0; 256];
    let mut idx = 0;
    cleanConsole();
    cleanMemoryMap();
    showProgram(input);

    loop {
        markUpProgram(input, cur);

        match input.chars().nth(cur) {
            Some('+') => {
                mem[idx] = handle_inc_dec(mem[idx] as usize, 0, 255, true) as u8;
                writeMemoryMap(idx, mem[idx]);
                cur += 1;
            }
            Some('-') => {
                mem[idx] = handle_inc_dec(mem[idx] as usize, 0, 255, false) as u8;
                writeMemoryMap(idx, mem[idx]);
                cur += 1;
            }
            Some('>') => {
                idx = handle_inc_dec(idx, 0, 255, true);
                writeMemoryMap(idx, mem[idx]);
                cur += 1;
            }
            Some('<') => {
                idx = handle_inc_dec(idx, 0, 255, false);
                writeMemoryMap(idx, mem[idx]);
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
                writeMemoryMap(idx, mem[idx]);
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
