pub struct Day2;

fn row_is_safe(report: &Vec<i32>) -> bool {
    let mut not_asc = 0;
    let mut not_desc = 0;

    for ii in 1..report.len() {
        let diff = report[ii] - report[ii - 1];
        if diff < 1 || diff > 3 {
            not_asc += 1;
        }
        if -diff < 1 || -diff > 3 {
            not_desc += 1;
        }
    }

    if not_asc == 0 || not_desc == 0 {
        return true;
    }

    return false;
}

impl aoc24::DayInner<Day2, i32> for Day2 {
    fn day(&self) -> i32 {
        2
    }

    fn inner(&self, input: String) -> (i32, i32) {
        let mut safe1 = 0;
        let mut safe2 = 0;

        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines: Vec<&str> = input.lines().collect();

        let reports = lines
            .iter()
            .map(|l| l.split(" ").collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>()
            .iter()
            .map(|s| {
                s.iter()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        for report in reports {
            let mut done = false;

            if row_is_safe(&report) {
                safe1 += 1;
                safe2 += 1;
                continue;
            }

            for ii in 1..report.len() {
                let diff = report[ii] - report[ii - 1];
                if diff < 1 || diff > 3 {
                    let mut report_copy = report.clone();
                    report_copy.remove(ii);
                    if row_is_safe(&report_copy) {
                        safe2 += 1;
                        done = true;
                        break;
                    }

                    let mut report_copy = report.clone();
                    report_copy.remove(ii - 1);
                    if row_is_safe(&report_copy) {
                        safe2 += 1;
                        done = true;
                    }
                    break;
                }
            }

            if !done {
                for ii in 1..report.len() {
                    let diff = report[ii] - report[ii - 1];
                    if -diff < 1 || -diff > 3 {
                        let mut report_copy = report.clone();
                        report_copy.remove(ii);
                        if row_is_safe(&report_copy) {
                            safe2 += 1;
                            break;
                        }

                        let mut report_copy = report.clone();
                        report_copy.remove(ii - 1);
                        if row_is_safe(&report_copy) {
                            safe2 += 1;
                        }
                        break;
                    }
                }
            }
        }

        // And we're done!
        (safe1, safe2)
    }
}
