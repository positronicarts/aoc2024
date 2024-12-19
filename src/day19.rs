use std::collections::HashMap;

pub struct Day19;

impl aoc24::DayInner<Day19, u64> for Day19 {
    fn day(&self) -> i32 {
        19
    }

    fn inner(&self, input: String) -> (u64, u64) {
        let mut lines = input.lines();
        let seeds = lines.next().unwrap().split(", ").collect::<Vec<_>>();
        lines.next(); // Skip blank line

        let mut count1 = 0;
        let mut count2 = 0;
        let mut yes_no: HashMap<&str, u64> = HashMap::new();

        while let Some(line) = lines.next() {
            fn check<'a>(
                line: &'a str,
                yes_no: &mut HashMap<&'a str, u64>,
                seeds: &Vec<&'a str>,
            ) -> u64 {
                if let Some(b) = yes_no.get(line) {
                    return *b;
                }

                let mut count = 0;
                for seed in seeds.iter() {
                    if line == *seed {
                        count += 1;
                    } else if line.starts_with(*seed) {
                        let r = check(&line[seed.len()..], yes_no, &seeds);
                        count += r;
                    }
                }
                yes_no.insert(line, count);
                count
            }

            let matched = check(line, &mut yes_no, &seeds);
            count1 += (matched > 0) as u64;
            count2 += matched as u64;
        }

        // And we're done!
        (count1, count2)
    }
}
