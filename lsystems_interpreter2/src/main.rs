use turtle::*;

fn main() {
    let n = 5;
    let distance = 6.0; // +
    let angle = 22.5; // +

    let mut fred = Turtle::new();
//    let mut pattern = String::from("F+F+F+F");
    let mut pattern = String::from("X");

    let mut rules = Vec::<(u8, String)>::new();
//    rules.push((b'F', "FF-F".into()));
    rules.push((b'F', "FF".into())); // +
    rules.push((b'X', "F-[[X]+X]+F[+FX]-X".into())); // +


    for _ in 0..n {
        let mut next_pattern = String::new();
        'patterns: for p in pattern.bytes() {
            for (matcher, rewrite) in &rules {
                println!("p: {}, matcher: {}", p, matcher);
                if *matcher == p {
                    next_pattern.push_str(rewrite);
                    continue 'patterns;
                }
            }

            next_pattern.push(p.into());
        }
        pattern = next_pattern;
    }

    let mut stack: Vec<(turtle::Point, turtle::Angle)> = vec![]; // +
    for character in pattern.chars() {
        match character {
            'F' => fred.forward(distance),
            'f' => {fred.pen_down(); fred.forward(distance); fred.pen_up();}
            '+' => fred.left(angle),   // +
            '-' => fred.right(angle),  // +
            '[' => stack.push((fred.position(), fred.heading())), // +
            ']' => { // +
                let (oldpos, oldheading) = stack.pop().unwrap();
                fred.pen_up();
                fred.go_to(oldpos);
                fred.set_heading(oldheading);
                fred.pen_down();
            },
            _ => continue,
        }
    }
}