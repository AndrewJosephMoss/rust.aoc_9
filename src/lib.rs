use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    IResult,
};

#[derive(PartialEq, Debug)]
enum Move {
    Up(u16),
    Left(u16),
    Down(u16),
    Right(u16),
}

pub fn process_part_1(input: &str) -> usize {
    let mut uniq_pos = HashSet::<(i16, i16)>::from([(0, 0)]);
    let moves = get_moves(input);
    let mut lead: (i16, i16) = (0, 0);
    let mut tail: (i16, i16) = (0, 0);
    moves.iter().for_each(|mv| {
        let steps = get_move_steps(mv);
        let mv_delta = get_move_coord_delta(mv);
        for _ in 0..steps {
            lead = (lead.0 + mv_delta.0, lead.1 + mv_delta.1);
            if move_connected_knots(&mut lead, &mut tail) {
                uniq_pos.insert(tail.clone());
            }
        }
    });
    uniq_pos.len()
}

pub fn process_part_2(input: &str) -> usize {
    let mut rope: [(i16, i16); 10] = [(0, 0); 10];
    let mut uniq_pos = HashSet::<(i16, i16)>::from([(0, 0)]);
    let moves = get_moves(input);
    todo!()
}

/// Returns true if the tail knot moved.
fn move_connected_knots(lead: &(i16, i16), tail: &mut (i16, i16)) -> bool{
    if !knots_are_touching(lead, tail) {
        let pos_delta: (i16, i16) = (lead.0 - tail.0, lead.1 - tail.1);
        if pos_delta.0.abs() >= 2 {
            tail.1 = lead.1;
            tail.0 = if pos_delta.0 > 0 {
                lead.0 - 1
            } else {
                lead.0 + 1
            }
        } else {
            tail.0 = lead.0;
            tail.1 = if pos_delta.1 > 0 {
                lead.1 - 1
            } else {
                lead.1 + 1
            }
        }
        return true;
    }
    return false;
}

fn knots_are_touching(lead: &(i16, i16), tail: &(i16, i16)) -> bool {
    return (lead.0 - tail.0).abs() <= 1 && (lead.1 - tail.1).abs() <= 1;
}

fn get_move_coord_delta(mv: &Move) -> (i16, i16) {
    return match mv {
        Move::Up(_) => (0, 1),
        Move::Left(_) => (-1, 0),
        Move::Down(_) => (0, -1),
        Move::Right(_) => (1, 0),
    };
}

fn get_move_steps(mv: &Move) -> u16 {
    match mv {
        Move::Up(steps) => *steps,
        Move::Right(steps) => *steps,
        Move::Down(steps) => *steps,
        Move::Left(steps) => *steps,
    }
}

fn get_moves(input: &str) -> Vec<Move> {
    let moves = input
        .lines()
        .map(|line| parse_move(line).unwrap().1)
        .collect::<Vec<Move>>();
    moves
}

fn parse_move(line: &str) -> IResult<&str, Move> {
    let mut parser = nom::sequence::separated_pair(alpha1, tag(" "), digit1);
    let (input, pair) = parser(line)?;
    let steps = pair.1.parse::<u16>().unwrap();
    let mv = match pair.0 {
        "U" => Move::Up(steps),
        "R" => Move::Right(steps),
        "D" => Move::Down(steps),
        "L" => Move::Left(steps),
        _ => panic!("Failed to parse Move from: {:?}", pair),
    };
    Ok((input, mv))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_process_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let result = process_part_1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_move_connected_knots() {
        let mut lead: (i16, i16) = (0, 0);
        let mut tail = lead.clone();
        let result = move_connected_knots(&mut lead, &mut tail);
        assert!(!result);
        assert_eq!(tail, (0, 0));

        let mut lead: (i16, i16) = (0, 2);
        let mut tail = (0, 0);
        let result = move_connected_knots(&mut lead, &mut tail);
        assert!(result);
        assert_eq!(tail, (0, 1));

        let mut lead: (i16, i16) = (-1, -2);
        let mut tail = (0, 0);
        let result = move_connected_knots(&mut lead, &mut tail);
        assert!(result);
        assert_eq!(tail, (-1, -1));

        let mut lead: (i16, i16) = (0, 1);
        let mut tail = (1, 3);
        let result = move_connected_knots(&mut lead, &mut tail);
        assert!(result);
        assert_eq!(tail, (0, 2));
    }

    #[test]
    fn test_knots_are_touching() {
        let lead: (i16, i16) = (2, 2);
        let tail = lead.clone();
        let result = knots_are_touching(&lead, &tail);
        assert!(result);

        let lead: (i16, i16) = (-1, 3);
        let tail = (0, 0);
        let result = knots_are_touching(&lead, &tail);
        assert!(!result);

        let lead: (i16, i16) = (-1, -3);
        let tail = (5, 4);
        let result = knots_are_touching(&lead, &tail);
        assert!(!result);

        let lead: (i16, i16) = (3, 3);
        let tail = (2, 3);
        let result = knots_are_touching(&lead, &tail);
        assert!(result);

        let lead: (i16, i16) = (3, 3);
        let tail = (4, 4);
        let result = knots_are_touching(&lead, &tail);
        assert!(result);

        let lead: (i16, i16) = (3, 3);
        let tail = (2, 2);
        let result = knots_are_touching(&lead, &tail);
        assert!(result);
    }

    #[test]
    fn test_get_moves() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let moves = get_moves(&input);
        assert_eq!(
            moves,
            vec![
                Move::Right(4),
                Move::Up(4),
                Move::Left(3),
                Move::Down(1),
                Move::Right(4),
                Move::Down(1),
                Move::Left(5),
                Move::Right(2)
            ]
        );
    }

    #[test]
    fn test_parse_moev() {
        let input = "R 4";
        let mv = parse_move(&input).unwrap().1;
        assert_eq!(mv, Move::Right(4));

        let input = "U 2";
        let mv = parse_move(&input).unwrap().1;
        assert_eq!(mv, Move::Up(2));

        let input = "L 1";
        let mv = parse_move(&input).unwrap().1;
        assert_eq!(mv, Move::Left(1));

        let input = "D 16";
        let mv = parse_move(&input).unwrap().1;
        assert_eq!(mv, Move::Down(16));
    }
}
