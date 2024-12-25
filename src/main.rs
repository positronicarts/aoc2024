mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use aoc24::Day;

fn main() {
    // day1::Day1 {}.run();
    // day2::Day2 {}.run();
    // day3::Day3 {}.run();
    // day4::Day4 {}.run();
    // day5::Day5 {}.run();
    // day6::Day6 {}.run();
    // day7::Day7 {}.run();
    // day8::Day8 {}.run();
    // day9::Day9 {}.run();
    // day10::Day10 {}.run();
    // day11::Day11 {}.run();
    // day12::Day12 {}.run();
    // day13::Day13 {}.run();
    // day14::Day14 {}.run();
    // day15::Day15 {}.run();
    // day16::Day16 {}.run();
    // day17::Day17 {}.run();
    // day18::Day18 {}.run();
    // day19::Day19 {}.run();
    // day20::Day20 {}.run();
    // day21::Day21 {}.run();
    // day22::Day22 {}.run();
    // day23::Day23 {}.run();
    // day24::Day24 {}.run();
    day25::Day25 {}.run();
}

#[cfg(test)]
mod test {
    use super::{
        day1, day10, day11, day12, day13, day14, day15, day16, day17, day18, day19, day2, day20,
        day21, day22, day23, day24, day25, day3, day4, day5, day6, day7, day8, day9, Day,
    };

    #[test]
    fn day1() {
        assert_eq!(day1::Day1 {}.test(), (11, 31));
    }

    #[test]
    fn day2() {
        assert_eq!(day2::Day2 {}.test(), (2, 4));
    }

    #[test]
    fn day3() {
        assert_eq!(day3::Day3 {}.test(), (161, 48));
    }

    #[test]
    fn day4() {
        assert_eq!(day4::Day4 {}.test(), (18, 9));
    }

    #[test]
    fn day5() {
        assert_eq!(day5::Day5 {}.test(), (143, 123));
    }

    #[test]
    fn day6() {
        assert_eq!(day6::Day6 {}.test(), (41, 6));
    }

    #[test]
    fn day7() {
        assert_eq!(day7::Day7 {}.test(), (3749, 11387));
    }

    #[test]
    fn day8() {
        assert_eq!(day8::Day8 {}.test(), (14, 34));
    }

    #[test]
    fn day9() {
        assert_eq!(day9::Day9 {}.test(), (1928, 2858));
    }

    #[test]
    fn day10() {
        assert_eq!(day10::Day10 {}.test(), (36, 81));
    }

    #[test]
    fn day11() {
        assert_eq!(day11::Day11 {}.test(), (55312, 65601038650482));
    }

    #[test]
    fn day12() {
        assert_eq!(day12::Day12 {}.test(), (1930, 1206));
    }

    #[test]
    fn day13() {
        assert_eq!(day13::Day13 {}.test(), (480, 875318608908));
    }

    #[test]
    fn day14() {
        assert_eq!(day14::Day14 {}.test(), (12, 0));
    }

    #[test]
    fn day15() {
        assert_eq!(day15::Day15 {}.test(), (10092, 9021));
    }

    #[test]
    fn day16() {
        // assert_eq!(day16::Day16 {}.test(), (7036, 45));
        assert_eq!(day16::Day16 {}.test(), (11048, 64));
    }

    #[test]
    fn day17() {
        assert_eq!(
            day17::Day17 {}.test(),
            ("5,7,3,0".to_string(), "117440".to_string())
        );
    }

    #[test]
    fn day18() {
        assert_eq!(day18::Day18 {}.test(), (22, 0));
    }

    #[test]
    fn day19() {
        assert_eq!(day19::Day19 {}.test(), (6, 16));
    }

    #[test]
    fn day20() {
        assert_eq!(day20::Day20 {}.test(), (44, 285));
    }

    #[test]
    fn day21() {
        assert_eq!(day21::Day21 {}.test(), (126384, 154115708116294));
    }

    #[test]
    fn day22() {
        assert_eq!(day22::Day22 {}.test(), (37990510, 23));
    }

    #[test]
    fn day23() {
        assert_eq!(day23::Day23 {}.test(), (7, 0));
    }

    #[test]
    fn day24() {
        assert_eq!(day24::Day24 {}.test(), (2024, 0));
    }

    #[test]
    fn day25() {
        assert_eq!(day25::Day25 {}.test(), (3, 0));
    }
}
