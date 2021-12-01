use aoc2021::core::get_data;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Edge {
    Increase,
    Decrease,
}

impl Edge {
    pub fn from_tuple((previous_reading, current_reading): (u32, u32)) -> Edge {
        if previous_reading < current_reading {
            return Edge::Increase;
        }

        Edge::Decrease
    }
}

fn sum_three_tuple((a, b, c): (u32, u32, u32)) -> u32 {
    a + b + c
}

fn get_increasing(edges: &[&str]) -> usize {
    edges
        .iter()
        .map(|str| str.parse::<u32>().unwrap())
        .tuple_windows::<(_, _, _)>()
        .map(sum_three_tuple)
        .tuple_windows()
        .map(Edge::from_tuple)
        .filter(|x| x == &Edge::Increase)
        .count()
}

fn main() {
    let input = get_data("1a");
    let data: Vec<&str> = input.split('\n').collect();

    println!("Increasing: {}", get_increasing(&data));
}

#[cfg(test)]
mod day_1b_tests {
    use crate::*;

    #[test]
    fn counts_three_windows() {
        let data = vec!["1", "2", "1", "0", "1", "2", "3"];
        //               4    3    2    3    6
        assert_eq!(get_increasing(&data), 2);
    }
}
