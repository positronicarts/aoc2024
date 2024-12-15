pub struct Day14;
use num::complex::Complex;

#[derive(Debug, Clone)]
struct Robot {
    position: Complex<i32>,
    velocity: Complex<i32>,
}

impl Robot {
    fn multi_step_bound(mut self, width: i32, height: i32, steps: i32) -> Robot {
        self.position += self.velocity * steps;
        self.position.re = self.position.re % width;
        self.position.im = self.position.im % height;
        self
    }
}

fn print_robots(robots: &Vec<Robot>, width: i32, height: i32) {
    let mut grid = vec![vec![0; width as usize]; height as usize];
    for r in robots.iter() {
        grid[r.position.im as usize][r.position.re as usize] += 1;
    }
    println!();
    for row in grid.iter() {
        for col in row.iter() {
            if *col > 0 {
                print!("{}", *col);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn max_consecutive_ones(robots: &Vec<Robot>, width: i32, height: i32) -> usize {
    let mut max: usize = 0;
    let mut grid = vec![vec![0; width as usize]; height as usize];
    for r in robots.iter() {
        grid[r.position.im as usize][r.position.re as usize] += 1;
    }

    let mut one: bool;
    for row in grid.iter() {
        one = false;
        let mut start = -1;
        for ii in 0..width as usize {
            if row[ii] == 1 {
                if !one {
                    start = ii as i32;
                }
                one = true;
            } else {
                if one && start > -1 {
                    let len = (ii as i32 - start) as usize;
                    if len > max {
                        max = len as usize;
                    }
                }
                one = false;
            }
        }
    }
    max
}

impl aoc24::DayInner<Day14, i32> for Day14 {
    fn day(&self) -> i32 {
        14
    }

    fn inner(&self, input: String) -> (i32, i32) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        let test = lines.clone().next().unwrap() == "p=0,4 v=3,-3";
        println!("{:?}", test);

        let width = if test { 11 } else { 101 };
        let height = if test { 7 } else { 103 };

        // Lines are of form p=x,y v=dx,dy
        let mut robots: Vec<Robot> = lines
            .map(|line| {
                let parts: Vec<&str> = line.split(" ").collect();
                let position: Vec<i32> = parts[0].split("=").collect::<Vec<&str>>()[1]
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                let velocity: Vec<i32> = parts[1].split("=").collect::<Vec<&str>>()[1]
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                Robot {
                    position: Complex::new(position[0], position[1]),
                    velocity: Complex::new(
                        (velocity[0] + width) % width,
                        (velocity[1] + height) % height,
                    ),
                }
            })
            .collect();

        print_robots(&robots, width, height);
        let moved_robots: Vec<Robot> = robots
            .clone()
            .into_iter()
            .map(|r| r.multi_step_bound(width, height, 100))
            .collect();
        print_robots(&moved_robots, width, height);

        let mut buckets = vec![0, 0, 0, 0];
        for r in moved_robots.iter() {
            if (r.position.re < width / 2) && (r.position.im < height / 2) {
                buckets[0] += 1;
            } else if (r.position.re < width / 2) && (r.position.im > height / 2) {
                buckets[1] += 1;
            } else if (r.position.re > width / 2) && (r.position.im < height / 2) {
                buckets[2] += 1;
            } else if (r.position.re > width / 2) && (r.position.im > height / 2) {
                buckets[3] += 1;
            }
        }

        let prod = buckets.iter().product::<i32>();
        println!("{}", prod);

        let mut count = 0;

        if !test {
            loop {
                count += 1;
                robots = robots
                    .into_iter()
                    .map(|r| r.multi_step_bound(width, height, 1))
                    .collect();
                if max_consecutive_ones(&robots, width, height) > 10 {
                    print!("{} ", count);
                    print_robots(&robots, width, height);
                    break;
                }
            }
        }

        // And we're done!
        (prod, count)
    }
}
