pub struct Day21;

#[derive(Clone)]
struct KeyPad {
    keys: Vec<Vec<char>>,
}

#[derive(Clone)]
struct Robot {
    x: i32,
    y: i32,
    keypad: KeyPad,
    controller: Option<Box<Robot>>,
}

impl Robot {
    fn move_to(&mut self, target: char) -> i32 {
        if self.controller.is_none() {
            // Can move directly
            // Find the target coordinates, then the manhattan distance from here to there
            print!("{}", target);
            return 1;
        } else {
            // Ruh-roh, need the controller to move to the direction first!
            let mut tx = -1;
            let mut ty = -1;
            for (y, row) in self.keypad.keys.iter().enumerate() {
                for (x, key) in row.iter().enumerate() {
                    if *key == target {
                        tx = x as i32;
                        ty = y as i32;
                    }
                }
            }

            if tx == -1 || ty == -1 {
                panic!("Target not found!");
            }

            let mut cost = 0;
            let controller_box = self.controller.as_mut().unwrap();
            let controller: &mut Robot = controller_box.as_mut();

            // Generally prefer L/R over U/D
            // The exception is if that will move us through a '.' on the keypad
            if self.keypad.keys[self.y as usize][tx as usize] != '.' {
                while self.x != tx {
                    if self.x < tx {
                        cost += controller.move_to('>');
                        self.x += 1;
                    }
                    if self.x > tx {
                        cost += controller.move_to('<');
                        self.x -= 1;
                    }
                }
            }
            // while self.x != tx {
            //     if self.x < tx {
            //         if self.keypad.keys[self.y as usize][self.x as usize + 1] != '.' {
            //             cost += controller.move_to('>');
            //             self.x += 1;
            //         } else {
            //             break;
            //         }
            //     } else if self.x > tx {
            //         if self.keypad.keys[self.y as usize][self.x as usize - 1] != '.' {
            //             cost += controller.move_to('<');
            //             self.x -= 1;
            //         } else {
            //             break;
            //         }
            //     }
            // }

            while self.y != ty {
                if self.y < ty {
                    cost += (*controller).move_to('v');
                    self.y += 1;
                } else {
                    cost += (*controller).move_to('^');
                    self.y -= 1;
                }
            }

            while self.x != tx {
                if self.x < tx {
                    cost += controller.move_to('>');
                    self.x += 1;
                } else if self.x > tx {
                    cost += controller.move_to('<');
                    self.x -= 1;
                }
            }

            cost += controller.move_to('A');

            return cost;
        }
    }
}

impl aoc24::DayInner<Day21, i32> for Day21 {
    fn day(&self) -> i32 {
        21
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        let number_keypad = {
            let mut keys = Vec::new();
            keys.push(vec!['7', '8', '9']);
            keys.push(vec!['4', '5', '6']);
            keys.push(vec!['1', '2', '3']);
            keys.push(vec!['.', '0', 'A']);
            KeyPad { keys }
        };

        let input_keypad = {
            let mut keys = Vec::new();
            keys.push(vec!['.', '^', 'A']);
            keys.push(vec!['<', 'v', '>']);
            KeyPad { keys }
        };

        let part_1 = lines
            .map(|code| part1(&number_keypad, &input_keypad, code))
            .sum();
        // let part_1 = part1(&number_keypad, &input_keypad, "379A");
        // And we're done!
        (part_1, 0)
    }
}

fn part1(number_keypad: &KeyPad, input_keypad: &KeyPad, code: &str) -> i32 {
    let robot_4 = Robot {
        x: 2,
        y: 9,
        keypad: input_keypad.clone(),
        controller: None,
    };
    let robot_3 = Robot {
        x: 2,
        y: 0,
        keypad: input_keypad.clone(),
        controller: Some(Box::new(robot_4)),
    };
    let robot_2 = Robot {
        x: 2,
        y: 0,
        keypad: input_keypad.clone(),
        controller: Some(Box::new(robot_3)),
    };
    let mut robot_1 = Robot {
        x: 2,
        y: 3,
        keypad: number_keypad.clone(),
        controller: Some(Box::new(robot_2)),
    };

    let mut button_presses = 0;
    for c in code.chars() {
        button_presses += robot_1.move_to(c);
        println!();
    }

    let numerical_code: i32 = code[..code.len() - 1].parse().unwrap();
    let product = button_presses * numerical_code;
    println!(
        "{} -> {} * {} = {}",
        code, button_presses, numerical_code, product
    );
    button_presses * numerical_code
}
