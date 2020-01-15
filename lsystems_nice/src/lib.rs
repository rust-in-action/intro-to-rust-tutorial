// ω (omega)
// ø / δ (delta): turn angle

pub enum Instruction {
    MoveForward,
    PaintForward,
    Turn(f32),
    Save,
    Restore,
    // Used for debugging
    Noop(char),
}

// structs hold data.
//#[derive(Debug, Eq, PartialEq, Clone)]
type Axiom = String;

#[derive(Debug, PartialEq, Clone)]
pub struct System {
    pub rules: Vec<Rule>,
    pub axiom: Axiom,
    pub turn_angle: f32,
}

impl Default for System {
    fn default() -> Self {
        System {
            rules: vec![],
            axiom: Axiom::from(""),
            turn_angle: 90.0,
        }
    }
}

impl System {
    pub fn apply_rules(&mut self) {
        let mut new_axiom = Axiom::new();
        'chars: for character in self.axiom.chars() {
            for rule in &self.rules {
                if rule.trigger == character {
                    new_axiom.push_str(&rule.replacement);
                    continue 'chars;
                }
            }
            new_axiom.push(character);
        }
        self.axiom = new_axiom;
    }

//    fn interpret(&self) -> Vec<Instruction> {
//        use Instruction::*;
//        let mut inst = Vec::<Instruction>::with_capacity(self.axiom.len());
//        for character in self.axiom.chars() {
//            let next = match character {
//                'F' => PaintForward,
//                'f' => MoveForward,
//                '+' => Turn(self.turn_angle),
//                '-' => Turn(-self.turn_angle),
//                '[' => Save,
//                ']' => Restore,
//                _ => Noop(character),
//            };
//            inst.push(next);
//        }
//        inst
//    }
}

//struct Turtle {
//    heading: f32,
//    x: i32,
//    y: i32,
//}
//
//impl Turtle {
//    fn render(&mut self, sys: &System) {
//        let todo = sys.interpret();
//
//    }
//}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rule {
    trigger: char,
    replacement: String,
}

#[allow(unused)]
pub mod known_patterns {
    use super::{Rule,System};
    use crate::Axiom;

    /// Algorithmic Beauty of Plants, p7
    pub fn koch_island() -> System {
        System {
            axiom: Axiom::from("F-F-F-F"),
            rules: vec![
                Rule { trigger: 'F', replacement: "F-F+F+FF-F-F+F".into() },
            ],
            ..Default::default()
        }
    }

    /// Algorithmic Beauty of Plants, p12
    ///
    /// broken :(
    pub fn hex_gosper_curve() -> System {
        System {
            axiom: Axiom::from("L"),
            rules: vec![
                Rule { trigger: 'L', replacement: "FL+FR++FR-FL--FLFL-FR+".into()},
                Rule { trigger: 'R', replacement: "-FL+FRFR++FR+FL---FL-FR".into()}
            ],
            turn_angle: 60.0,
        }
    }

    pub fn fass_a() -> System {
        System {
            axiom: Axiom::from("-L"),
            rules: vec![
                Rule { trigger: 'L', replacement: "LF+RFR+FL-F-LFLFL-FRFR+".into()},
                Rule { trigger: 'R', replacement: "-LFLF+RFRFR+F+RF-LFL-FR".into()}
            ],
            turn_angle: 90.0,
        }
    }

    pub fn fass_b() -> System {
        System {
            axiom: Axiom::from("-L"),
            rules: vec![
                Rule { trigger: 'L', replacement: "LFLF+RFR+FLFL-FRF-LFL-FR+F+RF-LFL-FRFRFR+".into()},
                Rule { trigger: 'R', replacement: "-LFLFLF+RFR+FL-F-LF+RFR+FLF+RFRF-LFL-FRFR".into()}
            ],
            turn_angle: 90.0,
        }
    }

    /// https://nb.paulbutler.org/l-systems/
    pub fn hilbert_curve() -> System {
        System {
            axiom: Axiom::from("L"),
            rules: vec![
                Rule { trigger: 'L', replacement: "-RF+LFL+FR-".into()},
                Rule { trigger: 'R', replacement: "+LF-RFR-FL+".into()}
            ],
            ..Default::default()
        }
    }

    /// Algorithmic Beauty of Plants,
    /// p 15, fig 1.24
    pub fn tree_f() -> System {
        System {
            axiom: Axiom::from("X"),
            rules: vec![
                Rule { trigger: 'X', replacement: "F-[[X]+X]+F[+FX]-X".into()},
                Rule { trigger: 'F', replacement: "FF".into()}
            ],
            turn_angle: 22.5,
        }
    }
}

// Rust lessons
// traits:
// - iteration
//   - turn the System into an iterator
//   - add size hint

#[test]
fn ffff() {
    let mut system: System = System {
        axiom: Axiom::from("F+F+F+F"),
        rules: vec![Rule { trigger: 'F', replacement: "F+F-F-FF+F+F-F".into()}],
        turn_angle: 90.0,
    };

    system.apply_rules();
    assert_eq!(system.axiom, Axiom::from("F+F-F-FF+F+F-F+F+F-F-FF+F+F-F+F+F-F-FF+F+F-F+F+F-F-FF+F+F-F"));
}

#[test]
fn abba() {
    let mut system: System = System {
        axiom: Axiom::from("abba"),
        rules: vec![Rule { trigger: 'b', replacement: "bab".into() }],
        turn_angle: 90.0,
    };

    system.apply_rules();
    assert_eq!(system.axiom, Axiom::from("ababbaba"));

    system.apply_rules();
    assert_eq!(system.axiom, Axiom::from("ababababbabababa"));
}