use itertools::Itertools;
use std::{
    char,
    collections::{HashMap, HashSet},
    ops,
};

pub struct Day8;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl ops::Add<&Coordinate> for &Coordinate {
    type Output = Coordinate;

    fn add(self, _rhs: &Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<&Coordinate> for &Coordinate {
    type Output = Coordinate;

    fn sub(self, _rhs: &Coordinate) -> Coordinate {
        Coordinate {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::Mul<i32> for &Coordinate {
    type Output = Coordinate;

    fn mul(self, _rhs: i32) -> Coordinate {
        Coordinate {
            x: self.x * _rhs,
            y: self.y * _rhs,
        }
    }
}

impl Coordinate {
    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        self.x >= 0 && self.x < x && self.y >= 0 && self.y < y
    }
}

impl aoc24::DayInner<Day8, usize> for Day8 {
    fn day(&self) -> i32 {
        8
    }

    fn inner(&self, input: String) -> (usize, usize) {
        // Read data - make sure we have a blank line at the end to check the final entries.
        let lines: Vec<&str> = input.lines().collect();

        let mut locations = HashMap::<char, Vec<Coordinate>>::new();
        let mut antinodes = HashSet::<Coordinate>::new();
        let mut resonant_antinodes = HashSet::<Coordinate>::new();

        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        let max_dimension = width.max(height);

        for (yy, line) in lines.iter().enumerate() {
            for (xx, cc) in line.chars().enumerate() {
                if cc == '.' {
                    continue;
                }
                let coord = Coordinate {
                    x: xx as i32,
                    y: yy as i32,
                };
                locations.entry(cc).or_insert(Vec::new()).push(coord);
            }
        }

        for list in locations.values() {
            for pairs in list.iter().combinations(2) {
                let diff = pairs[0] - pairs[1];

                for ii in 0..max_dimension {
                    let antinode = pairs[0] + &(&diff * ii);
                    if antinode.is_in_bounds(width, height) {
                        if ii == 1 {
                            antinodes.insert(antinode.clone());
                        }
                        resonant_antinodes.insert(antinode);
                    } else {
                        break;
                    }
                }
                for ii in 0..max_dimension {
                    let antinode = pairs[1] - &(&diff * ii);
                    if antinode.is_in_bounds(width, height) {
                        if ii == 1 {
                            antinodes.insert(antinode.clone());
                        }
                        resonant_antinodes.insert(antinode);
                    } else {
                        break;
                    }
                }
            }
        }

        // And we're done!
        (antinodes.len(), resonant_antinodes.len())
    }
}
