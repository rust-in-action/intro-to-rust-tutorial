use turtle::*;

fn main() {
    let n = 3;
    let mut fred = Turtle::new();
    let mut pattern = String::from("F+F+F+F");

    let mut rules = Vec::<(u8, String)>::new();
    rules.push((b'F', "FF-F".into()));

    for _ in 0..n {
        let mut next_pattern = String::new();
        'patterns: for p in pattern.bytes() {
            for (matcher, rewrite) in &rules {
                if (*matcher == p) {
                    next_pattern.push_str(rewrite);
                    continue 'patterns;
                }
            }

            next_pattern.push(p.into());
        }
        pattern = next_pattern;
    }

    for character in pattern.chars() {
        match character {
            'F' => fred.forward(50.0),
            '+' => fred.left(90.0),
            '-' => fred.right(90.0),
            _ => continue,
        }
    }
}