extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn brainfuck(item: TokenStream) -> TokenStream {
    let mut output = "".to_string();
    let mut buffer_size = 0;

    let t = item.into_iter().next().expect("Expected &str");
    // for t in item {
    let s = format!("{}", t);
    let mut s = s.chars();
    s.next();
    s.next_back();
    let s = s.as_str();

    if s.matches("[").count() == 0 {
        buffer_size = s.matches(">").count();
    } else {
        buffer_size = 30_000;
    }
    if buffer_size == 0 {
        buffer_size += 1;
    }

    if s.matches(",").count() > 0 {
        output.push_str("use std::io::Read;\n");
    }

    let mut chars_iter = s.chars().peekable();
    while let Some(char) = chars_iter.next() {
        match char {
                '+' => {
                    let mut amount = 1;
                    while chars_iter.next_if_eq(&'+').is_some() {
                        amount += 1;
                    }
                    output.push_str(format!("tape[ptr] += {};\n", amount).as_str());
                }
                '-' => {
                    let mut amount = 1;
                    while chars_iter.next_if_eq(&'-').is_some() {
                        amount += 1;
                    }
                    output.push_str(format!("tape[ptr] -= {};\n", amount).as_str());
                }
                '>' => output.push_str("ptr += 1;\n"),
                '<' => output.push_str("ptr -= 1;\n"),
                '.' => output.push_str("print!(\"{}\", tape[ptr] as char);\n"),
                ',' => output.push_str(
                    "tape[ptr] = std::io::stdin().bytes().next().and_then(|result| result.ok()).expect(\"Could not parse value into u8\");\n",
                ),
                '[' => output.push_str("while tape[ptr] != 0 {\n"),
                ']' => output.push_str("}\n"),
                _ => (),
            }
    }
    // }

    let mut output = format!(
        "{{
let mut ptr: usize= 0;
let mut tape: Vec<u8> = vec![0;{}];
{}",
        buffer_size,
        output.as_str()
    )
    .to_string();
    output.push_str("}");

    output.parse().unwrap()
}
