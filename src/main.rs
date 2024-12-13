mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
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
    day13::Day13 {}.run();
}

#[cfg(test)]
mod test {
    use super::{
        day1, day10, day11, day12, day13, day2, day3, day4, day5, day6, day7, day8, day9, Day,
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
        assert_eq!(day13::Day13 {}.test(), (480, 0));
    }
}
