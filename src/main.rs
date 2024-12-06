mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use aoc24::Day;

fn main() {
    // day1::Day1 {}.run();
    // day2::Day2 {}.run();
    // day3::Day3 {}.run();
    // day4::Day4 {}.run();
    // day5::Day5 {}.run();
    day6::Day6 {}.run();
}

#[cfg(test)]
mod test {
    use super::{day1, day2, day3, day4, day5, day6, Day};

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
}
