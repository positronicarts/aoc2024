pub struct Day3;

impl aoc24::DayInner<Day3, i32> for Day3 {
    fn day(&self) -> i32 {
        3
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut enabled = true;
        let mut total_mult = 0;
        let mut enabled_mult = 0;

        let lines = input.lines();
        let mul_re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let combined_re = regex::Regex::new(r"(?<mul>mul\((\d+),(\d+)\))|(?<do>do\(\))|(?<dont>don\'t\(\))").unwrap();

        for line in lines {
            for cap in combined_re.captures_iter(line) {
                if let Some(mul_match) = cap.name("mul") {
                    let mul_cap = mul_re.captures(mul_match.as_str()).unwrap();
                    let x = mul_cap[1].parse::<i32>().unwrap();
                    let y = mul_cap[2].parse::<i32>().unwrap();
                    total_mult += x * y;

                    if enabled {
                        enabled_mult += x * y;
                    }
                }
                if cap.name("do").is_some() {
                    enabled = true;
                }

                if cap.name("dont").is_some() {
                    enabled = false;
                }
            }
        }

        // And we're done!
        (total_mult, enabled_mult)
    }
}
