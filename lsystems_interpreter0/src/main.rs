use turtle::*;

fn main() {
    let mut fred = Turtle::new();
    let pattern = "F+F+F+F";

    for character in pattern.chars() {
        match character {
            'F' => fred.forward(50.0),
            '+' => fred.left(90.0),
            _ => continue,
        }
    }
}