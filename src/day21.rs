use itertools::Itertools;
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
    fn move_str(&mut self, path: &str) -> i64 {
        let mut cost = 0;
        for c in path.chars() {
            cost += self.move_to(c);
        }
        cost
    }

    fn move_to(&mut self, target: char) -> i64 {
        let start_x = self.x;
        let start_y = self.y;

        if self.keypad.keys[self.y as usize][self.x as usize] == '.' {
            panic!("Cheated!!");
        }
        if self.controller.is_none() {
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

            // let mut cost: i64 = 0;
            let controller_box = self.controller.as_mut().unwrap();
            let controller: &mut Robot = controller_box.as_mut();

            // Generally prefer L than D then R then U

            // Try all the different paths...
            // Get path elements
            let mut path = String::new();
            while self.x > tx {
                path += "<";
                self.x -= 1;
            }
            while self.x < tx {
                path += ">";
                self.x += 1;
            }
            while self.y > ty {
                path += "^";
                self.y -= 1;
            }
            while self.y < ty {
                path += "v";
                self.y += 1;
            }

            // Get different permutations of the path
            let mut cost = -1;

            for path in path.chars().permutations(path.len()) {
                let path = path.iter().collect::<String>() + "A";

                // Check the path is valid
                let mut test_x = start_x;
                let mut test_y = start_y;

                let mut invalid = false;
                for c in path.chars() {
                    if self.keypad.keys[test_y as usize][test_x as usize] == '.' {
                        invalid = true;
                    }
                    if c == 'A' {
                        break;
                    }
                    if c == '<' {
                        test_x -= 1;
                    } else if c == '>' {
                        test_x += 1;
                    } else if c == '^' {
                        test_y -= 1;
                    } else if c == 'v' {
                        test_y += 1;
                    }
                }
                if invalid {
                    continue;
                }

                let this_cost = controller.move_str(&path);
                if cost == -1 || this_cost < cost {
                    cost = this_cost;
                }
            }

            if cost > 0 {
                self.cache.insert(((start_x, start_y), (tx, ty)), cost);
                return cost;
            }
        }
        panic!("Negative cost!");
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
