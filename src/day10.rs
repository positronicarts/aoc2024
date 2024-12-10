use std::collections::HashSet;

pub struct Day10;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

struct Grid {
    grid: Vec<Vec<i32>>,
    width: i32,
    height: i32,
    peaks: HashSet<Coord>,
}

impl Grid {
    fn try_trails(&mut self, x: i32, y: i32, h: i32) -> usize {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return 0;
        }
        if self.grid[y as usize][x as usize] != h {
            return 0;
        }

        if h == 9 {
            self.peaks.insert(Coord { x, y });
            return 1;
        }

        let mut count = 0;
        count += self.try_trails(x + 1, y, h + 1);
        count += self.try_trails(x - 1, y, h + 1);
        count += self.try_trails(x, y - 1, h + 1);
        count += self.try_trails(x, y + 1, h + 1);

        return count;
    }
}

impl aoc24::DayInner<Day10, usize> for Day10 {
    fn day(&self) -> i32 {
        10
    }

    fn inner(&self, input: String) -> (usize, usize) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines = input.lines();

        // Create a grid to store the data.
        let mut grid = Grid {
            grid: lines
                .map(|line| {
                    line.chars()
                        .map(|d| d.to_digit(10).unwrap() as i32)
                        .collect()
                })
                .collect(),
            width: 0,
            height: 0,
            peaks: HashSet::new(),
        };
        grid.width = grid.grid[0].len() as i32;
        grid.height = grid.grid.len() as i32;

        // Part 1
        let mut count = 0;
        let mut peaks = 0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                count += grid.try_trails(x, y, 0);
                let trails = grid.peaks.len();

                if trails > 0 {
                    peaks += trails;
                }

                grid.peaks.clear();
            }
        }

        // And we're done!
        (peaks, count)
    }
}
