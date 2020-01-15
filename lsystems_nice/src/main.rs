#![feature(clamp)]

use turtle::{Turtle, Angle};
use turtle::{rand::random, Color};
use l_systems::System;

fn main() {
    let mut turtle = Turtle::new();

    // Distance to move forward
    let mut d = 6.0;
    
    // Number of iterations
    let n = 5;

    // If you look in lib.rs, you'll see several pre-built patterns.
    // Feel free to have a dig around and look for them.
    // 
    //   let mut system = l_systems::known_patterns::koch_island();
    let mut system = l_systems::known_patterns::tree_f();
    for _ in 0..n {
        system.apply_rules();
    }
    turtle.hide();
    turtle.pen_down();
    turtle.set_speed(25);
    let start = turtle.position().clone();

    for i in 0..1 {
        d = 0.5*d;
        println!("{:?}", system.rules);
        println!("{:?}", system.axiom);
        turtle.pen_up();
        turtle.go_to(start);
        turtle.pen_down();
        turtle.set_pen_color("#222222");
        // turtle.set_pen_color(random::<Color>().with_alpha((1.0/i as f64).clamp(0.0, 1.0)));

        let mut stack: Vec<(turtle::Point, turtle::Angle)> = vec![];
        for step in system.axiom.chars() {
            match step {
                'F' => turtle.forward(d),
                'f' => { turtle.pen_up(); turtle.forward(d); turtle.pen_down() },
                '-' | '−' => turtle.right(system.turn_angle.into()),
                '+' => turtle.left(system.turn_angle.into()),
                '[' => stack.push((turtle.position(), turtle.heading())),
                ']' => {
                    let (oldpos, oldheading) = stack.pop().unwrap();
                    turtle.pen_up();
                    turtle.go_to(oldpos);
                    turtle.set_heading(oldheading);
                    turtle.pen_down();
                },
                _ => println!("skip: {}", step),
            }
        }

        turtle.drawing().save_svg("lsystem.svg");
    }

}
