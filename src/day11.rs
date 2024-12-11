use std::{collections::HashMap, usize};

pub struct Day11;

struct Cache {
    // Cache from (value, depth) -> number of stones
    cache: HashMap<(u64, usize), u64>,
}

impl Cache {
    fn get(&mut self, input: (u64, usize)) -> u64 {
        if let Some(val) = self.cache.get(&input) {
            return *val;
        }

        let (val, depth) = input;

        if depth == 0 {
            return 1;
        }

        if val == 0 {
            let res = self.get((1, depth - 1));
            self.cache.insert(input, res);
            return res;
        }

        let val_str = val.to_string();
        let len = val_str.len();

        if len % 2 == 0 {
            let lhs = &val_str[..len / 2];
            let rhs = &val_str[len / 2..];

            let res = self.get((lhs.parse().unwrap(), depth - 1))
                + self.get((rhs.parse().unwrap(), depth - 1));
            self.cache.insert(input, res);
            return res;
        }

        let res = self.get((val * 2024, depth - 1));
        self.cache.insert(input, res);
        return res;
    }
}

impl aoc24::DayInner<Day11, u64> for Day11 {
    fn day(&self) -> i32 {
        11
    }

    fn inner(&self, input: String) -> (u64, u64) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let rocks: Vec<u64> = input
            .split_ascii_whitespace()
            .map(|d| d.parse().unwrap())
            .collect();

        let mut cache: Cache = Cache {
            cache: HashMap::new(),
        };

        let sum1 = rocks.iter().map(|r| cache.get((*r, 25))).sum();
        let sum2 = rocks.iter().map(|r| cache.get((*r, 75))).sum();

        // And we're done!
        (sum1, sum2)
    }
}
