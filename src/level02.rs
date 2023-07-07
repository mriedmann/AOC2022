use std::{cmp::Ordering};
use itertools::*;

use crate::api::{InputFile, Solver};

pub struct Solver02 {}

#[derive(Debug)]
#[derive(Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl From<u32> for Choice {
    fn from(value: u32) -> Self {
        match value {
            1 => Choice::Rock,
            2 => Choice::Paper,
            3 => Choice::Scissors,
            _ => panic!("invalid value for choice")
        }
    }
}

impl Ord for Choice {
    
    fn cmp(&self, other: &Self) -> Ordering {
        let pair = (*self, *other);
        match pair {
            (a,b) if (a as u32) == (b as u32) => Ordering::Equal,
            (Choice::Rock, Choice::Scissors) => Ordering::Greater,
            (Choice::Scissors, Choice::Rock) => Ordering::Less,
            (a,b) if (a as u32) < (b as u32) => Ordering::Less,
            (a,b) if (a as u32) > (b as u32) => Ordering::Greater,
            _ => panic!("unexpected")
        }
    }
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Choice {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Choice {}

impl std::ops::Add<u32> for Choice {
    type Output = Choice;

    fn add(self, rhs: u32) -> Choice {
        Choice::from((self as u32 - 1 + rhs) % 3 + 1)
    }
}

impl std::ops::Sub<u32> for Choice {
    type Output = Choice;

    fn sub(self, rhs: u32) -> Self::Output {
        Choice::from((self as u32 + 3 - 1 - rhs) % 3 + 1)
    }
}

enum GameOutcome {
    Loose,
    Draw,
    Win
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Game {
    your_choice: Choice,
    enemy_choice: Choice
}

fn calculate_points(game: Game) -> u32 {
    let outcome: GameOutcome = match game {
        Game { enemy_choice: y, your_choice: x } if x == y => GameOutcome::Draw,
        Game { enemy_choice: y, your_choice: x } if x > y => GameOutcome::Win,
        Game { enemy_choice: y, your_choice: x } if x < y => GameOutcome::Loose,
        _ => panic!("NOPE")
    };
    let win_score = match outcome {
        GameOutcome::Loose => 0,
        GameOutcome::Draw => 3,
        GameOutcome::Win => 6,
    };
    return win_score + game.your_choice as u32;
}

fn convert_choice_char_to_choice(choice_string: char) -> Choice {
    match choice_string {
        'A' | 'X' => Choice::Rock,
        'B' | 'Y' => Choice::Paper,
        'C' | 'Z' => Choice::Scissors,
        _ => panic!("unexpected choice char")
    }
}

fn convert_outcome_char_to_outcome(outcome_char: char) -> GameOutcome {
    match outcome_char {
        'X' => GameOutcome::Loose, 
        'Y' => GameOutcome::Draw,
        'Z' => GameOutcome::Win,
        _ => panic!("unexpected outcome char")
    }
}

fn convert_line_to_choices(line: String) -> (Choice, Choice) {
    let mut parts = line.split_ascii_whitespace();
    let enemy_choice = convert_choice_char_to_choice(parts.next().unwrap().chars().at_most_one().unwrap().unwrap());
    let your_choice = convert_choice_char_to_choice(parts.next().unwrap().chars().at_most_one().unwrap().unwrap());
    (enemy_choice, your_choice)
}

fn convert_line_to_enemy_choice_and_target_outcome(line: String) -> (Choice, GameOutcome) {
    let mut parts = line.split_ascii_whitespace();
    let enemy_choice = convert_choice_char_to_choice(parts.next().unwrap().chars().at_most_one().unwrap().unwrap());
    let target_outcome = convert_outcome_char_to_outcome(parts.next().unwrap().chars().at_most_one().unwrap().unwrap());
    (enemy_choice, target_outcome)
}

fn convert_enemy_choice_and_target_outcome_to_game(enemy_choice: Choice, target_outcome: GameOutcome) -> Game {
    let your_choice = match (enemy_choice, target_outcome) {
        (x, GameOutcome::Win) => x + 1,
        (x, GameOutcome::Loose) => x - 1,
        (x, GameOutcome::Draw) => x,
    };
    Game { enemy_choice: enemy_choice, your_choice: your_choice }
}

impl Solver for Solver02 {
    fn solve_a(input_file: impl InputFile) -> u32 {
        input_file
            .into_iter()
            .map(|line| convert_line_to_choices(line))
            .map(|(enemy_choice,your_choice)| Game { enemy_choice: enemy_choice, your_choice: your_choice })
            .map(|game| calculate_points(game))
            .sum::<u32>()
    }

    fn solve_b(input_file: impl InputFile) -> u32 {
        input_file
            .into_iter()
            .map(|line| convert_line_to_enemy_choice_and_target_outcome(line))
            .map(|(enemy_choice,target_outcome)| convert_enemy_choice_and_target_outcome_to_game(enemy_choice, target_outcome))
            .map(|game| calculate_points(game))
            .sum::<u32>()
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{TextInputFile, FakeInputFile};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn input_02() -> FakeInputFile {
        let raw_input = "
        A Y
        B X
        C Z".trim_start();
        FakeInputFile::new(raw_input)
    }

    fn test_calculate_score_case(enemy_choice: Choice, your_choice: Choice, expected: u32) -> Result<(), String> {
        let result = calculate_points(Game { your_choice: your_choice, enemy_choice: enemy_choice });
        if result != expected {
            Err(format!(
                "{:?} + {:?} result: {}, expected: {}",
                enemy_choice, your_choice, result, expected
            ))
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_calculate_score() -> Result<(), String> {
        [
            (Choice::Rock, Choice::Paper, 8), 
            (Choice::Paper, Choice::Rock, 1), 
            (Choice::Scissors, Choice::Scissors, 6)
        ]
            .iter()
            .try_for_each(|(a, b, expected)| test_calculate_score_case(*a, *b, *expected))?;

        Ok(())
    }

    #[test]
    fn test_choice_add() {
        assert_eq!(Choice::Rock + (1 as u32), Choice::Paper);
        assert_eq!(Choice::Paper + (1 as u32), Choice::Scissors);
        assert_eq!(Choice::Scissors + (1 as u32), Choice::Rock);
    }

    #[test]
    fn test_choice_sub() {
        assert_eq!(Choice::Rock - (1 as u32), Choice::Scissors);
        assert_eq!(Choice::Paper - (1 as u32), Choice::Rock);
        assert_eq!(Choice::Scissors - (1 as u32), Choice::Paper);
    }

    #[test]
    fn test_order() {
        assert!(Choice::Paper > Choice::Rock);
        assert!(Choice::Paper < Choice::Scissors);
        assert!(Choice::Paper == Choice::Paper);
        assert!(Choice::Rock == Choice::Rock);
        assert!(Choice::Rock > Choice::Scissors);
        assert!(Choice::Rock < Choice::Paper);
        assert!(Choice::Scissors < Choice::Rock);
        assert!(Choice::Scissors == Choice::Scissors);
        assert!(Choice::Scissors > Choice::Paper);
    }

    #[test]
    fn test_convert_lint_to_game_2(){
        assert!(convert_enemy_choice_and_target_outcome_to_game(Choice::Rock, GameOutcome::Draw) == Game { your_choice: Choice::Rock, enemy_choice: Choice::Rock});
        assert!(convert_enemy_choice_and_target_outcome_to_game(Choice::Scissors, GameOutcome::Draw) == Game { your_choice: Choice::Scissors, enemy_choice: Choice::Scissors});
        assert!(convert_enemy_choice_and_target_outcome_to_game(Choice::Rock, GameOutcome::Win) == Game { your_choice: Choice::Paper, enemy_choice: Choice::Rock});
        assert!(convert_enemy_choice_and_target_outcome_to_game(Choice::Rock, GameOutcome::Loose) == Game { your_choice: Choice::Scissors, enemy_choice: Choice::Rock});
        assert!(convert_enemy_choice_and_target_outcome_to_game(Choice::Scissors, GameOutcome::Win) == Game { your_choice: Choice::Rock, enemy_choice: Choice::Scissors});
        assert!(convert_enemy_choice_and_target_outcome_to_game(Choice::Scissors, GameOutcome::Loose) == Game { your_choice: Choice::Paper, enemy_choice: Choice::Scissors});
    }

    #[test]
    fn test_02a() {
        let input_file = input_02();
        let solution = Solver02::solve_a(input_file);
        assert_eq!(solution, 15);
    }

    #[test]
    fn test_02a_real() {
        let input_file = TextInputFile::new(2);
        let solution = Solver02::solve_a(input_file);
        assert_eq!(solution, 10718);   
    }

    #[test]
    fn test_02b() {
        let input_file = input_02();
        let solution = Solver02::solve_b(input_file);
        assert_eq!(solution, 12);
    }

    #[test]
    fn test_02b_real() {
        let input_file = TextInputFile::new(2);
        let solution = Solver02::solve_b(input_file);
        assert_eq!(solution, 14652);   
    }
}
