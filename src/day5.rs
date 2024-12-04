pub struct Day5;

struct Rule {
    l: i32,
    r: i32,
}

fn is_valid(page: &Vec<i32>, rules: &Vec<Rule>) -> (bool, usize, usize) {
    for rule in rules {
        let l_index = page.iter().position(|&p| p == rule.l);
        let r_index = page.iter().position(|&p| p == rule.r);

        if let Some(l_index) = l_index {
            if let Some(r_index) = r_index {
                if l_index > r_index {
                    return (false, l_index, r_index);
                }
            }
        }
    }

    (true, 0, 0)
}

impl aoc24::DayInner<Day5, i32> for Day5 {
    fn day(&self) -> i32 {
        5
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();
        let mut rules: Vec<Rule> = Vec::new();
        let mut pages: Vec<Vec<i32>> = Vec::new();
        let mut invalid_pages: Vec<Vec<i32>> = Vec::new();

        let mut in_rule_block = true;
        for line in lines {
            if line.is_empty() {
                in_rule_block = false;
                continue;
            }

            if in_rule_block {
                // Parse rules
                let parts: Vec<&str> = line.split("|").collect();
                let l: i32 = parts[0].parse().unwrap();
                let r: i32 = parts[1].parse().unwrap();
                rules.push(Rule { l, r });
            } else {
                // Parse messages
                let parts: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
                pages.push(parts);
            }
        }

        // Check
        let mut mid_sum = 0;
        for page in pages {
            let (valid, _, _) = is_valid(&page, &rules);
            if valid {
                mid_sum += page[(page.len() - 1) / 2];
            } else {
                invalid_pages.push(page);
            }
        }

        let mut reordered_invalid_pages_mid = 0;
        for page in invalid_pages.iter_mut() {
            loop {
                let (valid, l, r) = is_valid(&page, &rules);
                if valid {
                    reordered_invalid_pages_mid += page[(page.len() - 1) / 2];
                    break;
                }

                // Swap the two elements at l and r
                let temp = page[l];
                page[l] = page[r];
                page[r] = temp;
            }
        }

        // And we're done!
        (mid_sum, reordered_invalid_pages_mid)
    }
}
