// http://rosettacode.org/wiki/Comma_quibbling
fn quibble(seq: &[&str]) -> String {
    match seq {
        [] => "{}".to_string(),
        [word] => format!("{{{}}}", word ),
        _ => format!("{{{} and {}}}", seq.init().connect(", "), seq.last().unwrap())
    }
}

#[cfg(not(test))]
fn main() {
    println!("{}", quibble([]));
    println!("{}", quibble(["ABC"]));
    println!("{}", quibble(["ABC", "DEF"]));
    println!("{}", quibble(["ABC", "DEF", "G", "H"]));
}

#[test]
fn output() {
    assert_eq!(quibble([]), "{}".to_string());
    assert_eq!(quibble(["ABC"]), "{ABC}".to_string());
    assert_eq!(quibble(["ABC", "DEF"]), "{ABC and DEF}".to_string());
    assert_eq!(quibble(["ABC", "DEF", "G", "H"]), "{ABC, DEF, G and H}".to_string());
}