use std::collections::HashMap;

pub struct Day25;

#[derive(Debug)]
struct Key {
    heights: Vec<usize>,
}

#[derive(Debug)]
struct Lock {
    heights: Vec<usize>,
}

fn overlap(key: &Key, lock: &Lock) -> bool {
    for ii in 0..key.heights.len() {
        if key.heights[ii] + lock.heights[ii] > 5 {
            return true;
        }
    }
    false
}

impl aoc24::DayInner<Day25, i32> for Day25 {
    fn day(&self) -> i32 {
        25
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        let mut block = Vec::<String>::new();

        let mut locks = Vec::<Lock>::new();
        let mut keys = Vec::<Key>::new();

        for line in lines {
            if line.is_empty() {
                println!("Block: {:?}", block);

                // Process the block
                if block[0] == "#####" {
                    let mut lock = Lock {
                        heights: Vec::new(),
                    };
                    for xx in 0..block[0].len() {
                        let mut found = false;
                        for yy in 1..block.len() {
                            if block[yy].chars().nth(xx).unwrap() == '.' {
                                lock.heights.push(yy - 1);
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            lock.heights.push(5);
                        }
                    }
                    locks.push(lock);
                } else {
                    let mut key = Key {
                        heights: Vec::new(),
                    };
                    for xx in 0..block[0].len() {
                        let mut found = false;
                        for yy in 1..(block.len() - 1) {
                            if block[block.len() - 1 - yy].chars().nth(xx).unwrap() == '.' {
                                key.heights.push(yy - 1);
                                found = true;
                                break;
                            }
                        }
                        if !found {
                            key.heights.push(5);
                        }
                    }
                    keys.push(key);
                }
                block.clear();
                continue;
            }

            block.push(line.to_string());
        }

        println!("Keys: {:?}", keys);
        println!("Locks: {:?}", locks);

        let mut fits = 0;
        for key in keys.iter() {
            for lock in locks.iter() {
                if !overlap(key, lock) {
                    fits += 1;
                }
            }
        }   

        // And we're done!
        (fits, 0)
    }
}
