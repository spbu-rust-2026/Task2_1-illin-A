fn main() {
    let mut string = String::from("MATMEX LUCHSHE VSEH");
    println!("{}", process(&mut string));
}

fn process(_text: &mut String) -> &str {
    let start = _text.find(|c: char| !c.is_whitespace()).unwrap_or(0);
    let end = _text[start..]
        .find(char::is_whitespace)
        .unwrap_or(_text[start..].len())
        + start;
    _text.push('!');
    &_text[start..end]
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
