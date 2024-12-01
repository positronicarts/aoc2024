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

        // Brute force loop, updating where necessary
        for line in lines {
            let split = line.split("   ").collect::<Vec<&str>>();
            let i1 = split[0].parse::<i32>().unwrap();
            let i2 = split[1].parse::<i32>().unwrap();

            vec1.push(i1);
            vec2.push(i2);
        }

        let mut sum_diff = 0;
        vec1.sort();
        vec2.sort();

        for ii in 0..vec1.len() {
            let diff = (vec1[ii] - vec2[ii]).abs();
            sum_diff += diff;
        }

        // Do the similarity score
        let mut sim_score = 0;

        for ii in 0..vec1.len() {
            let x = vec1[ii];
            let yc = vec2
                .iter()
                .filter(|y| **y == x)
                .collect::<Vec<&i32>>()
                .len() as i32;
            sim_score += x * yc;
        }

        // And we're done!
        (sum_diff, sim_score)
    }
}
