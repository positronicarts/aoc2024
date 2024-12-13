use std::time::Instant;
pub struct Day7;

#[derive(Debug)]
struct MaxStack {
    // stack: Vec<u64>,
    index: usize,
    target: u64,
    // max: u64,
}

impl MaxStack {
    fn from(
        // stack: &Vec<u64>,
        index: usize,
        target: u64,
    ) -> MaxStack {
        MaxStack {
            // stack: stack.clone(),
            index: index,
            target: target,
            // max: 0, //get_max(stack),
        }
    }

    fn check(&mut self, stack: &Vec<u64>, mut new_stacks: Vec<MaxStack>) -> (bool, Vec<MaxStack>) {
        let end = stack[self.index];

        // if self.target == self.max {
        //     return (true, vec![]);
        // }
        if self.index == 0 {
            return (self.target == end, new_stacks);
        }

        // let mut new_stacks = vec![];

        // if self.target == end {
        //     return (true, vec![]);
        // }

        // let new_max = if end == 1 || self.max == end {
        //     self.max - end
        // } else {
        //     self.max / end
        // };

        if (self.target % end) == 0 {
            new_stacks.push(MaxStack {
                // stack: self.stack.clone(),
                index: self.index - 1,
                target: self.target / end,
                // max: 0,
            });
        }

        if self.target > end {
            new_stacks.push(MaxStack {
                // stack: self.stack.clone(),
                index: self.index - 1,
                target: self.target - end,
                // max: 0,
            });
        }

        // See if the target (as a string) ends with the `end` string
        let mut end_len = 1;
        loop {
            if end >= end_len {
                end_len *= 10;
            } else {
                break;
            }
        }
        if self.target % end_len == end {
            new_stacks.push(MaxStack {
                // stack: self.stack.clone(),
                index: self.index - 1,
                target: self.target / end_len,
                // max: 0,
            });
        }

        // let target_str = self.target.to_string();
        // let end_str = end.to_string();

        // if end_str.len() >= target_str.len() {
        //     return (false, new_stacks);
        // }
        // // println!("{} {}", target_str, end_str);
        // if target_str.ends_with(&end_str) {
        //     let new_target = target_str[..target_str.len() - end_str.len()]
        //         .parse::<u64>()
        //         .unwrap();
        //     new_stacks.push(MaxStack {
        //         // stack: self.stack.clone(),
        //         index: self.index - 1,
        //         target: new_target,
        //         // max: 0,
        //     });
        // }

        (false, new_stacks)
    }
}

fn check_line(input: &(Vec<u64>, u64)) -> u64 {
    let (full_stack, target) = input;
    // let colon_split = line.split(": ").collect::<Vec<&str>>();

    // let target = colon_split[0].parse::<u64>().unwrap();
    // let full_stack = colon_split[1]
    //     // .trim()
    //     .split(" ")
    //     .map(|d| d.parse::<u64>().unwrap())
    //     .collect::<Vec<u64>>();

    let mut open_stacks = vec![MaxStack::from(full_stack.len() - 1, *target)];

    while !open_stacks.is_empty() {
        let mut stack = open_stacks.pop().unwrap();
        let (done, stacks) = stack.check(&full_stack, open_stacks);
        if done {
            return *target;
        }
        open_stacks = stacks;
    }

    0
}

impl aoc24::DayInner<Day7, u64> for Day7 {
    fn day(&self) -> i32 {
        7
    }

    fn inner(&self, input: String) -> (u64, u64) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        // use crossbeam_utils::thread; // 0.7.2
        // use std::sync::{Arc, Mutex};
        // use std::thread;

        // use rayon::prelude::*;

        // use std::sync::mpsc::channel;
        // use threadpool::ThreadPool;

        let lines = input.lines();

        let vecs: Vec<(Vec<u64>, u64)> = lines
            .map(|line| {
                let colon_split = line.split(": ").collect::<Vec<&str>>();

                let target = colon_split[0].parse::<u64>().unwrap();
                let full_stack = colon_split[1]
                    // .trim()
                    .split(" ")
                    .map(|d| d.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                (full_stack, target)
            })
            .collect();

        // let string_lines = lines.map(|s| s.to_string()).collect::<Vec<String>>();
        // let num_lines = string_lines.len();

        let now = Instant::now();

        let total1 = 3749; // lines.iter().map(|line| check_line(line).unwrap_or(0)).sum();

        let total2 = vecs
            .iter()
            .map(|vec| check_line(vec))
            .sum();

        // let threads: Vec<_> = string_lines.into_iter().map(move |line| thread::spawn(move || check_line2(&line).unwrap_or(0))).collect();
        // let mut total2 = 0;
        // for handle in threads {
        //     total2 += handle.join().unwrap();
        // }

        // let n_workers = 2;
        // let pool = ThreadPool::new(n_workers);
        // let (tx, rx) = channel();

        // for line in string_lines {
        //     let tx = tx.clone();
        //     pool.execute(move || {
        //         // tx.send(1).expect("channel will be there waiting for the pool");
        //         tx.send(check_line(&line)).expect("foo");
        //     });
        // }

        // let total2 = rx.iter().take(num_lines).sum();

        let elapsed = now.elapsed();
        println!("Elapsed: {:.5?}", elapsed);

        // And we're done!
        (total1, total2)
    }
}
