use std::collections::HashMap;

pub struct Day1;

impl aoc24::DayInner<Day1, i32> for Day1 {
    fn day(&self) -> i32 {
        1
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines: Vec<&str> = input.lines().collect();

        // Initiate local variables
        let mut vec1: Vec<i32> = Vec::new();
        let mut vec2: Vec<i32> = Vec::new();

        let mut buckets = HashMap::<i32, (i32, i32)>::new();

        for entries in lines
            .iter()
            .map(|l| l.split("   "))
            .map(Iterator::collect::<Vec<&str>>)
        {
            let parsed = entries
                .iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let (x, y) = (parsed[0], parsed[1]);

            buckets
                .entry(x)
                .and_modify(|(x, _y)| *x += 1)
                .or_insert((1, 0));
            buckets
                .entry(y)
                .and_modify(|(_x, y)| *y += 1)
                .or_insert((0, 1));

            vec1.push(x);
            vec2.push(y);
        }
        vec1.sort();
        vec2.sort();

        let mut sum_diff = 0;
        for ii in 0..vec1.len() {
            let diff = (vec1[ii] - vec2[ii]).abs();
            sum_diff += diff;
        }

        let sim_score = buckets.iter().map(|(k, (x, y))| k * x * y).sum();

        // And we're done!
        (sum_diff, sim_score)
    }
}
