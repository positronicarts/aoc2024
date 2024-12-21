use std::collections::HashMap;

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
    cache: HashMap<((i32, i32), (i32, i32)), i64>,
}

impl Robot {
    fn move_to(&mut self, target: char) -> i64 {
        let start_x = self.x;
        let start_y = self.y;

        if self.keypad.keys[self.y as usize][self.x as usize] == '.' {
            panic!("Cheated!!");
        }
        if self.controller.is_none() {
            // Can move directly
            // Find the target coordinates, then the manhattan distance from here to there
            // print!("{}", target);
            return 1;
        } else {
            // Check the cache
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

            if let Some(cost) = self.cache.get(&((start_x, start_y), (tx, ty))) {
                // println!(".");
                self.x = tx;
                self.y = ty;
                return *cost;
            }

            let mut cost: i64 = 0;
            let controller_box = self.controller.as_mut().unwrap();
            let controller: &mut Robot = controller_box.as_mut();

            // Generally prefer L than D then R then U

            // Start trying left, unless we'll get blocked
            while self.x > tx {
                if self.keypad.keys[self.y as usize][self.x as usize] == '.' {
                    panic!("Cheated!!");
                }

                if self.x > tx {
                    if self.keypad.keys[self.y as usize][tx as usize] != '.' {
                        cost += controller.move_to('<');
                        self.x -= 1;
                    } else {
                        break;
                    }
                }
            }

            // Then try down, unless it's blocked
            while self.y < ty {
                if self.keypad.keys[self.y as usize][self.x as usize] == '.' {
                    panic!("Cheated!!");
                }

                if self.y < ty {
                    if self.keypad.keys[ty as usize][self.x as usize] != '.' {
                        cost += (*controller).move_to('v');
                        self.y += 1;
                    } else {
                        break;
                    }
                }
            }

            // Right can't really be blocked
            while self.x < tx {
                if self.keypad.keys[self.y as usize][self.x as usize] == '.' {
                    panic!("Cheated!!");
                }

                if self.x < tx {
                    if self.keypad.keys[self.y as usize][tx as usize] != '.' {
                        cost += controller.move_to('>');
                        self.x += 1;
                    } else {
                        panic!("Huh?");
                    }
                }
            }

            // Up
            while self.y > ty {
                if self.keypad.keys[self.y as usize][self.x as usize] == '.' {
                    panic!("Cheated!!");
                }
                if self.y > ty {
                    if self.keypad.keys[ty as usize][self.x as usize] != '.' {
                        cost += (*controller).move_to('^');
                        self.y -= 1;
                    } else {
                        panic!("Huh?");
                    }
                }
            }

            // Left might have got blocked (we'll have sorted verticality by this point)
            while self.x > tx {
                if self.keypad.keys[self.y as usize][self.x as usize] == '.' {
                    panic!("Cheated!!");
                }
                if self.x > tx {
                    cost += controller.move_to('<');
                    self.x -= 1;
                }
            }

            cost += controller.move_to('A');

            // Update the cache
            self.cache.insert(((start_x, start_y), (tx, ty)), cost);
            return cost;
        }
    }
}

impl aoc24::DayInner<Day21, i64> for Day21 {
    fn day(&self) -> i32 {
        21
    }

    fn inner(&self, input: String) -> (i64, i64) {
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
            .clone()
            .map(|code| part1(&number_keypad, &input_keypad, code, 2))
            .sum();
        let part_2 = lines
            .map(|code| part1(&number_keypad, &input_keypad, code, 25))
            .sum();

        // And we're done!
        (part_1, part_2)
    }
}

fn part1(number_keypad: &KeyPad, input_keypad: &KeyPad, code: &str, num: i32) -> i64 {
    let human = Robot {
        x: 2,
        y: 0,
        keypad: input_keypad.clone(),
        controller: None,
        cache: HashMap::new(),
    };

    let mut current = human;
    for _ in 0..num {
        let next = Robot {
            x: 2,
            y: 0,
            keypad: input_keypad.clone(),
            controller: Some(Box::new(current)),
            cache: HashMap::new(),
        };
        current = next;
    }
    let mut final_robot = Robot {
        x: 2,
        y: 3,
        keypad: number_keypad.clone(),
        controller: Some(Box::new(current)),
        cache: HashMap::new(),
    };

    let mut button_presses = 0;
    for c in code.chars() {
        button_presses += final_robot.move_to(c);
    }

    let numerical_code: i64 = code[..code.len() - 1].parse().unwrap();
    let product = button_presses * numerical_code;
    println!(
        "{}: {} * {} = {}",
        code, button_presses, numerical_code, product
    );
    product
}
