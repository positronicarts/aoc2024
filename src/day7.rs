pub struct Day7;

#[derive(Debug)]
struct MaxStack {
    stack: Vec<u64>,
    target: u64,
    max: u64,
}

impl MaxStack {
    fn from(stack: &Vec<u64>, target: u64) -> MaxStack {
        MaxStack {
            stack: stack.clone(),
            target: target,
            max: get_max(stack),
        }
    }

    fn check(&mut self) -> (bool, Vec<MaxStack>) {
        if self.target == self.max {
            return (true, vec![]);
        } else if self.stack.len() == 1 {
            return (false, vec![]);
        } else if self.target > self.max {
            return (false, vec![]);
        }

        let mut new_stacks = vec![];
        let end = self.stack.pop().unwrap();

        if self.target == end {
            return (true, vec![]);
        }

        let new_max = if end == 1 || self.max == end {
            self.max - end
        } else {
            self.max / end
        };

        if (self.target % end) == 0 {
            new_stacks.push(MaxStack {
                stack: self.stack.clone(),
                target: self.target / end,
                max: new_max,
            });
        }

        if self.target > end {
            new_stacks.push(MaxStack {
                stack: self.stack.clone(),
                target: self.target - end,
                max: new_max,
            });
        }

        (false, new_stacks)
    }

    fn check2(&mut self) -> (bool, Vec<MaxStack>) {
        if self.target == self.max {
            return (true, vec![]);
        } else if self.stack.len() == 1 {
            return (false, vec![]);
        }

        let mut new_stacks = vec![];
        let end = self.stack.pop().unwrap();

        if self.target == end {
            return (true, vec![]);
        }

        let new_max = if end == 1 || self.max == end {
            self.max - end
        } else {
            self.max / end
        };

        if (self.target % end) == 0 {
            new_stacks.push(MaxStack {
                stack: self.stack.clone(),
                target: self.target / end,
                max: new_max,
            });
        }

        if self.target > end {
            new_stacks.push(MaxStack {
                stack: self.stack.clone(),
                target: self.target - end,
                max: new_max,
            });
        }

        // See if the target (as a string) ends with the `end` string
        let target_str = self.target.to_string();
        let end_str = end.to_string();
        if target_str.ends_with(&end_str) {
            let new_target = target_str[..target_str.len() - end_str.len()]
                .parse::<u64>()
                .unwrap();
            new_stacks.push(MaxStack {
                stack: self.stack.clone(),
                target: new_target,
                max: new_max,
            });
        }

        (false, new_stacks)
    }
}

fn get_max(stack: &Vec<u64>) -> u64 {
    let mut max = stack[0];
    for i in 1..stack.len() {
        if stack[i] == 1 || max == 1 {
            max += stack[i];
        } else {
            max *= stack[i];
        }
    }
    max
}

fn check_line(line: &str) -> Option<u64> {
    let colon_split = line.split(":").collect::<Vec<&str>>();

    let target = colon_split[0].parse::<u64>().unwrap();
    let stack = colon_split[1]
        .trim()
        .split(" ")
        .map(|d| d.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut open_stacks = vec![MaxStack::from(&stack, target)];

    while open_stacks.len() > 0 {
        let mut stack = open_stacks.pop().unwrap();
        let (done, stacks) = stack.check();
        if done {
            return Some(target);
        }
        open_stacks.extend(stacks);
    }

    None
}

fn check_line2(line: &str) -> Option<u64> {
    let colon_split = line.split(":").collect::<Vec<&str>>();

    let target = colon_split[0].parse::<u64>().unwrap();
    let stack = colon_split[1]
        .trim()
        .split(" ")
        .map(|d| d.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut open_stacks = vec![MaxStack::from(&stack, target)];

    while open_stacks.len() > 0 {
        let mut stack = open_stacks.pop().unwrap();
        let (done, stacks) = stack.check2();
        if done {
            return Some(target);
        }
        open_stacks.extend(stacks);
    }

    None
}

impl aoc24::DayInner<Day7, u64> for Day7 {
    fn day(&self) -> i32 {
        7
    }

    fn inner(&self, input: String) -> (u64, u64) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines().collect::<Vec<&str>>();

        let total1 = lines.iter().map(|line| check_line(line).unwrap_or(0)).sum();
        let total2 = lines
            .iter()
            .map(|line| check_line2(line).unwrap_or(0))
            .sum();

        // And we're done!
        (total1, total2)
    }
}
