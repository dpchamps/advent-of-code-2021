use aoc2021::core::get_data;
use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Debug)]
struct BingoBoard {
    id: Uuid,
    card: Vec<Vec<u32>>,
    index: HashMap<u32, Vec<(u8, u8)>>,
    marked: HashSet<(u8, u8)>,
}

impl TryFrom<&str> for BingoBoard {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let card: Vec<Vec<u32>> = value
            .lines()
            .map(|line| {
                line.trim()
                    .split_whitespace()
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        let mut index: HashMap<u32, Vec<(u8, u8)>> = HashMap::new();

        for i in 0..(5 * 5) {
            let x = i % 5;
            let y = i / 5;
            let num = card[y][x];
            let coord = (x as u8, y as u8);

            if let Some(list) = index.get_mut(&num) {
                list.push(coord);
            } else {
                index.insert(num, vec![coord]);
            }
        }

        assert_eq!(card.len(), 5);
        assert_eq!(card[0].len(), 5);

        Ok(BingoBoard {
            id: Uuid::new_v4(),
            card,
            marked: HashSet::new(),
            index,
        })
    }
}

impl BingoBoard {
    pub fn is_winner(&self) -> bool {
        let mut vertical_winners = [[0; 5]; 5];
        for y in 0..5 {
            let mut is_horizontal_winner = true;
            for x in 0..5 {
                let is_marked = self.marked.contains(&(x, y));
                is_horizontal_winner = is_horizontal_winner && is_marked;
                vertical_winners[x as usize][y as usize] = is_marked as i32;
            }

            if is_horizontal_winner {
                return true;
            }
        }

        for vertical_row in vertical_winners.iter() {
            if vertical_row.iter().sum::<i32>() == 5 {
                return true;
            }
        }

        false
    }

    pub fn mark(&mut self, num: &u32) {
        if let Some(coords) = self.index.get(num) {
            for coord in coords.iter() {
                self.marked.insert(*coord);
            }
        }
    }

    pub fn calculate_score(&self, last_called: u32) -> u32 {
        let mut unmarked_sum = 0;

        for i in 0..25 {
            let x = i % 5;
            let y = i / 5;
            let num = self.card[y][x];
            let coord = (x as u8, y as u8);

            if !self.marked.contains(&coord) {
                unmarked_sum += num
            }
        }

        unmarked_sum * last_called
    }
}

fn select_first_winning_board_score(input: &str) -> u32 {
    let mut start = input.split("\n\n");
    let stream = start
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap());
    let mut boards: Vec<BingoBoard> = start
        .map(|input| BingoBoard::try_from(input).unwrap())
        .collect();

    for digit in stream {
        for board in boards.iter_mut() {
            board.mark(&digit);

            if board.is_winner() {
                return board.calculate_score(digit);
            }
        }
    }

    unreachable!()
}

fn select_last_winning_board_score(input: &str) -> u32 {
    let mut start = input.split("\n\n");
    let stream = start
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap());
    let mut boards: Vec<BingoBoard> = start
        .map(|input| BingoBoard::try_from(input).unwrap())
        .collect();
    let mut winning_boards: HashSet<Uuid> = HashSet::new();
    let n_boards = boards.len();

    for digit in stream {
        for board in boards.iter_mut() {
            if winning_boards.contains(&board.id) {
                continue;
            }
            board.mark(&digit);
            let is_winner = board.is_winner();

            if is_winner && winning_boards.len() == n_boards - 1 {
                return board.calculate_score(digit);
            }

            if is_winner {
                winning_boards.insert(board.id);
            }
        }
    }

    unreachable!()
}

fn main() {
    let input = get_data("4a");

    println!(
        "First Winning Score: {}",
        select_first_winning_board_score(&input)
    );
    println!(
        "Last Winning Score: {}",
        select_last_winning_board_score(&input)
    );
}

#[cfg(test)]
mod day_4a_tests {
    use crate::*;

    const INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn returns_false_when_not_winner() {
        let card_input = " 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6";

        let mut board = BingoBoard::try_from(card_input).unwrap();

        board.mark(&3);
        board.mark(&15);
        board.mark(&2);
        board.mark(&22);

        assert_eq!(board.is_winner(), false);
    }

    #[test]
    fn returns_true_when_horizontal_winner() {
        let card_input = " 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6";

        let mut board = BingoBoard::try_from(card_input).unwrap();

        board.mark(&3);
        board.mark(&15);
        board.mark(&0);
        board.mark(&2);
        board.mark(&22);

        assert_eq!(board.is_winner(), true);
    }

    #[test]
    fn returns_true_when_vertical_winner() {
        let card_input = " 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6";

        let mut board = BingoBoard::try_from(card_input).unwrap();

        board.mark(&3);
        board.mark(&9);
        board.mark(&19);
        board.mark(&20);
        board.mark(&14);

        assert_eq!(board.is_winner(), true);
    }

    #[test]
    fn returns_winning_score_card() {
        assert_eq!(select_first_winning_board_score(&INPUT), 4512);
    }

    #[test]
    fn returns_last_winning_score_card() {
        assert_eq!(select_last_winning_board_score(&INPUT), 1924);
    }
}
