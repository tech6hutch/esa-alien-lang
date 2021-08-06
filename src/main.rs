mod util;

use ansi_term::{ANSIString, ANSIStrings, Color};
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
            if let Some(input) = args
                .collect::<Vec<String>>()
                .join(" ")
                .with_end_trimmed()
                .non_empty()
            {
                translate_input(&input);
                return;
            }

            loop {
                let input = ask_multi_lines(">");
                if input.is_empty() { break; }

                translate_input(&input);
            }
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
        println!("{}", Color::Red.paint("[Some alien letters in this message have yet to be deciphered.]"));
    }
    println!("{}", output);
}

fn get_help(exe_name: &str) -> String {
    format!("usage: {} translate <input>", exe_name)
}

fn parse_and_translate(input: &str) -> Result<String, String> {
    let chars_to_english = {
        use Color::{White, Red, Yellow};

        let mut pairs: Vec<(&str, char, Color)> = vec![
            // Whitespace
            (" ", ' ', White),
            ("\n", '\n', White),
            // Known
            ("r", 'A', White),
            ("_|_", 'E', White),
            ("^v", 'G', White),
            (":.", 'H', White),
            ("|", 'I', White),
            ("`|", 'L', White),
            ("n", 'N', White),
            ("`o", 'R', White),
            ("d", 'T', White),
            ("+", 'U', White),
            ("S", 'W', White),
            // Likely
            ("o", 'B', Yellow),
            ("t", 'C', Yellow),
            ("?", 'D', Yellow),
            ("^", 'F', Yellow),
            ("y", 'K', Yellow),
            ("H", 'M', Yellow),
            ("ロ", 'O', Yellow),
            ("ï", 'P', Yellow),
            ("🚬", 'Q', Yellow),
            ("|:", 'S', Yellow),
            ("E", 'V', Yellow),
            ("D", 'Y', Yellow),
            // Unknown
            (">", '_', Red),
        ];
        pairs.sort_unstable_by(|(a, _, _), (b, _, _)| b.len().cmp(&a.len()));
        pairs
    };

    let mut s = input;
    let mut output: Vec<ANSIString> = Vec::new();
    while s.len() > 0 {
        let (len, c, color) = chars_to_english.iter().copied()
            .find_map(|(k, v, color)| s.starts_with(k).then(|| (k.len(), v, color)))
            .ok_or_else(|| format!(
                "unknown char or char sequence starting at position {}", input.len() - s.len()
            ))?;
        output.push(color.paint(c.to_string()));
        s = &s[len..];
    }
    Ok(ANSIStrings(&output).to_string())
}
