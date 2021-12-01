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

fn get_increasing(edges: &[&str]) -> usize {
    edges
        .iter()
        .map(|str| str.parse::<u32>().unwrap())
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
mod day_1a_tests {
    use crate::*;

    #[test]
    fn counts_two_windows() {
        let data = vec!["1", "2", "1", "0", "1", "2", "3"];
        assert_eq!(get_increasing(&data), 4);
    }
}
