use aoc2021::core::get_data;
use std::convert::TryFrom;
use std::iter::FromIterator;

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instruction(Direction, u32);

impl Instruction {
    pub fn move_sub(&self, (horizontal, depth): (u32, u32)) -> (u32, u32) {
        match self.0 {
            Direction::Forward => (horizontal + self.1, depth),
            Direction::Up => (horizontal, depth - self.1),
            Direction::Down => (horizontal, depth + self.1),
        }
    }
}

impl<'a> FromIterator<&'a str> for Instruction {
    fn from_iter<I: IntoIterator<Item = &'a str>>(iter: I) -> Self {
        let mut i = iter.into_iter();
        Instruction(
            Direction::try_from(i.next().unwrap()).unwrap(),
            i.next().unwrap().parse::<u32>().unwrap(),
        )
    }
}

fn into_instruction_set(raw: &[&str]) -> Vec<Instruction> {
    raw.iter().map(|line| line.split(' ').collect()).collect()
}

fn follow_instructions(instructions: &[Instruction]) -> (u32, u32) {
    instructions
        .iter()
        .fold((0, 0), |acc, instruction| instruction.move_sub(acc))
}

fn solve(raw: &[&str]) -> u32 {
    let (x, y) = follow_instructions(&into_instruction_set(raw));

    x * y
}

fn main() {
    let input = get_data("2a");
    let data: Vec<&str> = input.split('\n').collect();

    println!("{}", solve(&data));
}

#[cfg(test)]
mod day_2a_tests {
    use crate::*;

    #[test]
    fn example() {
        let result = solve(&[
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]);

        assert_eq!(result, 150);
    }
}
