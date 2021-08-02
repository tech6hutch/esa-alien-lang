mod util;

use std::{env, path::Path};
use util::*;

fn main() {
    let mut args = env::args();

    let exe_name = args.next()
        .as_ref().map(Path::new).and_then(Path::file_name)
        .expect("no exe name found")
        .to_str()
        .expect("env::args already ensured this is valid Unicode")
        .to_owned();

    match args.next().as_deref().unwrap_or("translate") {
        "help" => {
            eprintln!("{}", get_help(&exe_name));
        }
        "translate" => {
            let mut input = args.collect::<Vec<String>>().join(" ");
            input.truncate(input.trim_end().len());
            translate_input(&input.non_empty().unwrap_or_else(|| ask("text to translate:")));
        }
        subcmd => {
            eprintln!("unknown arg '{}'", subcmd);
        }
    }
}

fn translate_input(input: &str) {
    let input = input.trim();
    if input.is_empty() {
        eprintln!("no input provided");
        return;
    }

    let output = match parse_and_translate(&input) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };
    if output.contains('_') {
        println!("[Some alien letters in this message have yet to be deciphered.]");
    }
    println!("{}", output);
}

fn get_help(exe_name: &str) -> String {
    format!("usage: {} translate <input>", exe_name)
}

fn parse_and_translate(input: &str) -> Result<String, String> {
    let chars_to_english = {
        let mut pairs: Vec<(&str, char)> = vec![
            (" ", ' '),
            ("r", 'A'),
            ("_|_", 'E'),
            ("^v", 'G'),
            (":.", 'H'),
            ("|", 'I'),
            ("`|", 'L'),
            ("n", 'N'),
            ("`o", 'R'),
            ("d", 'T'),
            ("+", 'U'),
            ("S", 'W'),
            // Unknown
            ("|:", '_'),
            ("?", '_'),
            ("ロ", '_'),
            ("t", '_'),
            ("H", '_'),
            ("D", '_'),
            ("o", '_'),
            ("^", '_'),
            ("E", '_'),
            ("y", '_'),
            ("ï", '_'),
            (">", '_'),
            ("🚬", '_'),
        ];
        pairs.sort_unstable_by(|(a, _), (b, _)| b.len().cmp(&a.len()));
        pairs
    };

    let mut s = input;
    let mut output = String::new();
    while s.len() > 0 {
        let (len, c) = chars_to_english.iter().copied()
            .find_map(|(k, v)| s.starts_with(k).then(|| (k.len(), v)))
            .ok_or_else(|| format!(
                "unknown char or char sequence starting at position {}", input.len() - s.len()
            ))?;
        output.push(c);
        s = &s[len..];
    }
    Ok(output)
}
