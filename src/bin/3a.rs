use aoc2021::core::get_data;
use std::collections::HashSet;

fn bit_partition(bits: &[u8]) -> (u8, u8) {
    let (ones, zeros): (Vec<u8>, Vec<u8>) = bits.iter().partition(|&&bit| bit == 1);

    if ones.len() < zeros.len() {
        return (0, 1);
    }

    (1, 0)
}

fn select_most_common_bit(bits: &[u8]) -> u8 {
    bit_partition(bits).0
}

fn select_least_common_bit(bits: &[u8]) -> u8 {
    bit_partition(bits).1
}

fn get_position_table(input: &[&str]) -> Vec<Vec<u8>> {
    let size = input[0].len();

    input.iter().fold(
        (0..size).map(|_| Vec::new()).collect(),
        |mut table, &line| {
            line.trim()
                .chars()
                .enumerate()
                .for_each(|(i, char)| table[i].push(char.to_digit(2).unwrap() as u8));

            table
        },
    )
}

fn filter_bits_by_position(bit_stream: &[u8], positions: &HashSet<usize>) -> Vec<u8> {
    bit_stream
        .iter()
        .enumerate()
        .filter(|(i, _)| !positions.contains(i))
        .map(|(_, &x)| x)
        .collect()
}

fn get_removed_positions_from_table(
    table: &[Vec<u8>],
    partition_fn: &dyn Fn(&[u8]) -> u8,
) -> HashSet<usize> {
    table
        .iter()
        .fold(HashSet::new(), |mut removed, bit_stream| {
            let filtered_stream: Vec<u8> = filter_bits_by_position(bit_stream, &removed);
            let select_bit = partition_fn(&filtered_stream);

            for (i, &bit) in bit_stream.iter().enumerate() {
                if removed.len() == bit_stream.len() - 1 {
                    break;
                }

                if bit != select_bit {
                    removed.insert(i);
                }
            }

            removed
        })
}

fn compute_rating(input: &[&str], partition_fn: &dyn Fn(&[u8]) -> u8) -> u32 {
    let table = get_position_table(input);
    let removed = get_removed_positions_from_table(&table, partition_fn);

    let (_, rating) = input
        .iter()
        .map(|&x| u32::from_str_radix(x.trim(), 2).unwrap())
        .enumerate()
        .find(|(i, _)| !removed.contains(i))
        .unwrap();

    rating
}

fn compute_life_support_rating(input: &[&str]) -> u32 {
    let generator_rating = compute_rating(input, &select_most_common_bit);
    let co2_scrubber_rating = compute_rating(input, &select_least_common_bit);

    generator_rating * co2_scrubber_rating
}

fn compute_power_usage(input: &[&str]) -> u32 {
    let table = get_position_table(input);
    let (gamma, epsilon): (u32, u32) = table.iter().fold((0, 0), |(gamma, epsilon), bit_stream| {
        let (most_common, least_common) = bit_partition(bit_stream);

        (
            (gamma << 1) | most_common as u32,
            (epsilon << 1) | least_common as u32,
        )
    });

    gamma * epsilon
}

fn main() {
    let input = get_data("3a");
    let data: Vec<&str> = input.lines().collect();

    println!("Power Consumption: {}", compute_power_usage(&data));
    println!(
        "Life Support Rating: {}",
        compute_life_support_rating(&data)
    );
}

#[cfg(test)]
mod day_3a_tests {
    use crate::*;
    const INPUT: &'static str = "00100
                            11110
                            10110
                            10111
                            10101
                            01111
                            00111
                            11100
                            10000
                            11001
                            00010
                            01010";

    #[test]
    fn test_compute_power_usage() {
        let lines: Vec<&str> = INPUT.lines().collect();
        assert_eq!(compute_power_usage(&lines), 198)
    }

    #[test]
    fn computes_generator_rating() {
        let lines: Vec<&str> = INPUT.lines().collect();

        assert_eq!(compute_rating(&lines, &select_most_common_bit), 23)
    }

    #[test]
    fn computes_co2_rating() {
        let lines: Vec<&str> = INPUT.lines().collect();

        assert_eq!(compute_rating(&lines, &select_least_common_bit), 10)
    }

    #[test]
    fn computes_life_support_rating() {
        let lines: Vec<&str> = INPUT.lines().collect();

        assert_eq!(compute_life_support_rating(&lines), 230)
    }

    #[test]
    fn does_part_one() {
        let input = get_data("3a");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(compute_power_usage(&data), 4191876)
    }

    #[test]
    fn does_part_two() {
        let input = get_data("3a");
        let data: Vec<&str> = input.lines().collect();

        assert_eq!(compute_life_support_rating(&data), 3414905)
    }
}
