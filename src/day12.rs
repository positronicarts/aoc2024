use std::{collections::HashSet, hash::Hash};

pub struct Day12;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    closed_set: HashSet<(usize, usize)>,
}

impl Grid {
    fn get_measurements(&self, shape: &HashSet<(usize, usize)>) -> (usize, usize, usize) {
        let mut perim = 0;
        let mut area = 0;
        let mut corners = 0;

        let mut corner_locations: HashSet<(usize, usize)> = HashSet::new();

        let mut antishape: HashSet<(usize, usize)> = HashSet::new();
        for ii in 0..self.height {
            for jj in 0..self.width {
                if !shape.contains(&(jj, ii)) {
                    antishape.insert((jj, ii));
                }
            }
        }
        for (x, y) in shape {
            let mut sides = 0;
            area += 1;
            let mut vc = 0;
            let mut hc = 0;
            // println!("Checking ({},{})", x, y);
            if *x == 0 || !shape.contains(&(*x - 1, *y)) {
                // println!("{} {} is on the edge", x, y);
                perim += 1;
                sides += 1;
                hc += 1;
            }
            if *x == self.width - 1 || !shape.contains(&(*x + 1, *y)) {
                // println!("{} {} is on the edge", x, y);
                perim += 1;
                sides += 1;
                hc += 1;
            }
            if *y == 0 || !shape.contains(&(*x, *y - 1)) {
                // println!("{} {} is on the edge", x, y);
                perim += 1;
                sides += 1;
                vc += 1;
            }
            if *y == self.height - 1 || !shape.contains(&(*x, *y + 1)) {
                // println!("{} {} is on the edge", x, y);
                perim += 1;
                sides += 1;
                vc += 1;
            }

            if sides == 4 {
                println!("{} {} is a concave 4 corner", x, y);
                corners += 4;
                corner_locations.insert((*x, *y));
            }
            if sides == 3 {
                println!("{} {} is a concave 3 corner", x, y);
                corners += 2;
                corner_locations.insert((*x, *y));
            }
            if sides == 2 {
                println!("{} {} is a concave 2 corner", x, y);
                if vc == 1 && hc == 1 {
                    corner_locations.insert((*x, *y));
                    corners += 1;
                }
            }
        }

        println!("Concave sides {}", corners);

        for (x, y) in &antishape {
            let mut sides = 0;
            let mut vc = 0;
            let mut hc = 0;
            let mut adj_corners = 0;
            let mut up = false;
            let mut down = false;
            let mut left = false;
            let mut right = false;
            // area += 1;
            // println!("Checking ({},{})", x, y);
            if *x == 0 || !antishape.contains(&(*x - 1, *y)) {
                // println!("{} {} is on the edge", x, y);

                if *x > 0 && shape.contains(&(*x - 1, *y)) {
                    hc += 1;
                    left = true;
                    sides += 1;
                    if corner_locations.contains(&(*x - 1, *y)) {
                        adj_corners += 1;
                    }
                }
                // sides += 1;
            }
            if *x == self.width - 1 || !antishape.contains(&(*x + 1, *y)) {
                // println!("{} {} is on the edge", x, y);

                if *x < self.width - 1 && shape.contains(&(*x + 1, *y)) {
                    hc += 1;
                    sides += 1;
                    right = true;
                    if corner_locations.contains(&(*x + 1, *y)) {
                        adj_corners += 1;
                    }
                }
            }
            if *y == 0 || !antishape.contains(&(*x, *y - 1)) {
                // println!("{} {} is on the edge", x, y);

                if *y > 0 && shape.contains(&(*x, *y - 1)) {
                    vc += 1;
                    up = true;
                    sides += 1;
                    if corner_locations.contains(&(*x, *y - 1)) {
                        adj_corners += 1;
                    }
                }
            }
            if *y == self.height - 1 || !antishape.contains(&(*x, *y + 1)) {
                // println!("{} {} is on the edge", x, y);
                if *y < self.height - 1 && shape.contains(&(*x, *y + 1)) {
                    vc += 1;
                    down = true;
                    sides += 1;
                    if corner_locations.contains(&(*x, *y + 1)) {
                        adj_corners += 1;
                    }
                }
            }

            if sides == 4 {
                println!("{} {} is a convex 4 corner", x, y);
                corners += 4;
            }
            if sides == 3 {
                println!("{} {} is a convex 3 corner", x, y);
                corners += 2;
            }
            if sides == 2 {
                println!("{} {} is a convex 2 corner", x, y);
                if vc == 1 && hc == 1 {
                //     // Won't count if it's "blocked" - i.e. the opposite corner is also a corner
                //     if (up && right) {
                //         println!("Up right corner");
                //         if !antishape.contains(&(*x + 1, *y - 1)) {
                //             println!("Not blocked");
                //             corners += 1;
                //         }
                //     }
                //     if (up && left) {
                //         println!("Up left corner");
                //         if !antishape.contains(&(*x - 1, *y - 1)) {
                //             println!("Not blocked");
                //             corners += 1;
                //         }
                //     }
                //     if (down && right) {
                //         println!("Down right corner");
                //         if !antishape.contains(&(*x + 1, *y + 1)) {
                //             println!("Not blocked");
                //             corners += 1;
                //         }
                //     }
                //     if (down && left) {
                //         println!("Down left corner");
                //         if !antishape.contains(&(*x - 1, *y + 1)) {
                //             println!("Not blocked");
                //             corners += 1;
                //         }
                //     }
                    corners += 1;
                }
            }

            if (up && right) {
                // println!("Up right corner");
                if antishape.contains(&(*x + 1, *y - 1)) {
                    println!("Blocked up right");
                    corners -= 1;
                }
            }
            if (up && left) {
                // println!("Up left corner");
                if antishape.contains(&(*x - 1, *y - 1)) {
                    println!("Blocked up left");
                    corners -= 1;
                }
            }
            if (down && right) {
                // println!("Down right corner");
                if antishape.contains(&(*x + 1, *y + 1)) {
                    println!("Blocked down right");
                    corners -= 1;
                }
            }
            if (down && left) {
                // println!("Down left corner");
                if antishape.contains(&(*x - 1, *y + 1)) {
                    println!("Blocked down left");
                    corners -= 1;
                }
            }
        }

        (area, perim, corners)
    }

    fn fill_from(&mut self, x: usize, y: usize) -> (usize, usize, usize) {
        if self.closed_set.contains(&(x, y)) {
            return (0, 0, 0);
        }

        let c = self.grid[y][x];
        let mut shape: HashSet<(usize, usize)> = HashSet::new();
        let mut open_set: HashSet<(usize, usize)> = HashSet::new();
        open_set.insert((x, y));

        while !open_set.is_empty() {
            let (tx, ty) = open_set.iter().next().unwrap().clone();
            open_set.remove(&(tx, ty));

            if shape.contains(&(tx, ty)) {
                panic!("Already in shape");
            }
            shape.insert((tx, ty));

            if self.closed_set.contains(&(tx, ty)) {
                panic!("Already closed");
            }
            self.closed_set.insert((tx, ty));

            if tx > 0 {
                if self.grid[ty][tx - 1] == c
                    && !self.closed_set.contains(&(tx - 1, ty))
                    && !open_set.contains(&(tx - 1, ty))
                {
                    open_set.insert((tx - 1, ty));
                }
            }
            if tx < self.width - 1 {
                if self.grid[ty][tx + 1] == c
                    && !self.closed_set.contains(&(tx + 1, ty))
                    && !open_set.contains(&(tx + 1, ty))
                {
                    open_set.insert((tx + 1, ty));
                }
            }
            if ty > 0 {
                if self.grid[ty - 1][tx] == c
                    && !self.closed_set.contains(&(tx, ty - 1))
                    && !open_set.contains(&(tx, ty - 1))
                {
                    open_set.insert((tx, ty - 1));
                }
            }
            if ty < self.height - 1 {
                if self.grid[ty + 1][tx] == c
                    && !self.closed_set.contains(&(tx, ty + 1))
                    && !open_set.contains(&(tx, ty + 1))
                {
                    open_set.insert((tx, ty + 1));
                }
            }
        }

        let (area, perim, corners) = self.get_measurements(&shape);

        println!(
            "Shape {} area {} perim {} corners {}",
            c, area, perim, corners
        );

        (area, perim, corners)
    }
}

impl aoc24::DayInner<Day12, usize> for Day12 {
    fn day(&self) -> i32 {
        12
    }

    fn inner(&self, input: String) -> (usize, usize) {
        let lines: Vec<&str> = input.lines().collect();

        let mut grid: Grid = Grid {
            grid: lines.iter().map(|line| line.chars().collect()).collect(),
            width: lines[0].len(),
            height: lines.len(),
            closed_set: HashSet::new(),
        };

        println!("Grid: {:?}", grid);

        let mut sum = 0;
        let mut sum2 = 0;

        for ii in 0..grid.height {
            for jj in 0..grid.width {
                let (area, perim, corners) = grid.fill_from(jj, ii);
                sum += area * perim;
                sum2 += area * corners;
                // break;
            }
            // break;
        }

        // And we're done!
        (sum, sum2)
    }
}
