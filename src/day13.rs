use num::complex::Complex;
use regex::Regex;

pub struct Day13;

impl aoc24::DayInner<Day13, i64> for Day13 {
    fn day(&self) -> i32 {
        13
    }

    fn inner(&self, input: String) -> (i64, i64) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines: Vec<&str> = input.lines().collect();

        let mut button_a: Complex<i64> = Complex::new(0, 0);
        let mut button_b: Complex<i64> = Complex::new(0, 0);
        let mut prize: Complex<i64> = Complex::new(0, 0);

        let part2_offset = 10000000000000;
        let part2_diff: Complex<i64> = Complex::new(part2_offset, part2_offset);

        let coordinate_regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let prize_regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();

        let mut sum1: i64 = 0;
        let mut sum2: i64 = 0;

        // Part 1
        for ii in 0..lines.len() {
            if ii % 4 == 0 {
                let colon_split = lines[ii].split(": ").collect::<Vec<&str>>();
                let coordinates = coordinate_regex.captures(colon_split[1]).unwrap();
                button_a = Complex::new(
                    coordinates[1].parse::<i64>().unwrap(),
                    coordinates[2].parse::<i64>().unwrap(),
                );
            }
            if ii % 4 == 1 {
                let colon_split = lines[ii].split(": ").collect::<Vec<&str>>();
                let coordinates = coordinate_regex.captures(colon_split[1]).unwrap();
                button_b = Complex::new(
                    coordinates[1].parse::<i64>().unwrap(),
                    coordinates[2].parse::<i64>().unwrap(),
                );
            }
            if ii % 4 == 2 {
                let colon_split = lines[ii].split(": ").collect::<Vec<&str>>();
                let coordinates = prize_regex.captures(colon_split[1]).unwrap();
                prize = Complex::new(
                    coordinates[1].parse::<i64>().unwrap(),
                    coordinates[2].parse::<i64>().unwrap(),
                );
            }
            if ii % 4 == 3 {
                if !lines[ii].is_empty() {
                    panic!("Expected blank line at end of entry");
                }

                // Part 1!
                let mut min_match = calc(button_a, button_b, prize);
                if let Some(cost) = min_match {
                    sum1 += cost;
                }

                // Part 2
                prize += part2_diff;
                min_match = calc(button_a, button_b, prize);
                if let Some(cost) = min_match {
                    sum2 += cost;
                }
            }
        }

        // And we're done!
        (sum1, sum2)
    }
}

fn calc(button_a: Complex<i64>, button_b: Complex<i64>, prize: Complex<i64>) -> Option<i64> {
    let button_a_cost = 3;
    let button_b_cost = 1;

    let top = prize.re * button_a.im - prize.im * button_a.re;
    let bottom = button_b.re * button_a.im - button_b.im * button_a.re;

    if bottom == 0 {
        panic!("Bottom is zero");
    }

    if top % bottom == 0 {
        let b = top / bottom;

        if (prize.re - b * button_b.re) % button_a.re != 0 {
            return None;
        }

        let a = (prize.re - b * button_b.re) / button_a.re;

        if a >= 0 && b >= 0 {
            let cost = a * button_a_cost + b * button_b_cost;
            return Some(cost);
        }
    }

    return None;
}
