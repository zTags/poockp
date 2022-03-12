use colored::Colorize;

use std::io::{stdin, stdout, Write};

pub fn question(query: &str, hint: &str, default: &str) -> String {
    let mut ans = String::new();
    
    print!("{} {} ", query.cyan(), format!("({})?", hint).bright_black());

    stdout().flush().unwrap_or(());
    stdin().read_line(&mut ans).expect("invalid input provided");

    // shoutouts to random stranger who provided me this code i think it works
    if let Some('\n') = ans.chars().next_back() {
        ans.pop();
    }

    if let Some('\r') = ans.chars().next_back() {
        ans.pop();
    }

    return if ans.len() == 0 { default.to_string() } else { ans };
}