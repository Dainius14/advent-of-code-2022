use std::{fs, io::{self, BufRead}};

fn main() {
    let file = fs::File::open("data/day2_input.txt").expect("Should be able to open the file");
    let reader = io::BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {

        let line_str = line.unwrap();
        let line_bytes = line_str.as_bytes();
        let enemy_attack_char = line_bytes[0];
        let supposed_outcome_char = line_bytes[2];

        let enemy_attack = get_attack_type_from_enemy(&enemy_attack_char);
        let supposed_outcome = get_supposed_game_outcome(&supposed_outcome_char);

        let my_attack = get_my_attack(enemy_attack, supposed_outcome);

        let points_for_game = get_points_for_game_outcome(supposed_outcome);
        let points_for_attack = get_points_for_attack(my_attack);

        total_score += points_for_attack + points_for_game;
    }

    println!("Score: {total_score}");
}

#[derive(PartialEq, Clone, Copy)]
enum AttackType {
    Rock,
    Paper,
    Sciccors,
}

#[derive(PartialEq, Clone, Copy)]
enum GameOutcome {
    Win,
    Equal,
    Loss,
}

fn get_attack_type_from_enemy(str: &u8) -> AttackType {
    match str {
        b'A' => AttackType::Rock,
        b'B' => AttackType::Paper,
        b'C' => AttackType::Sciccors,
        _ => unreachable!(),
    }
}

fn get_supposed_game_outcome(str: &u8) -> GameOutcome {
    match str {
        b'X' => GameOutcome::Loss,
        b'Y' => GameOutcome::Equal,
        b'Z' => GameOutcome::Win,
        _ => unreachable!(),
    }
}

fn get_points_for_attack(attack_type: AttackType) -> i32 {
    match attack_type {
        AttackType::Rock => 1,
        AttackType::Paper => 2,
        AttackType::Sciccors => 3,
    }
}

fn get_my_attack(enemy_attack: AttackType, outcome: GameOutcome) -> AttackType {
    if outcome == GameOutcome::Equal {
        enemy_attack
    }
    
    else if outcome == GameOutcome::Win {
        if enemy_attack == AttackType::Rock {
            AttackType::Paper
        }
        else if enemy_attack == AttackType::Paper {
            AttackType::Sciccors
        }
        else {
            AttackType:: Rock
        }
    }

    else {
        if enemy_attack == AttackType::Rock {
            AttackType::Sciccors
        }
        else if enemy_attack == AttackType::Paper {
            AttackType::Rock
        }
        else {
            AttackType::Paper
        }
    }

}

fn get_points_for_game_outcome(outcome: GameOutcome) -> i32 {
    match outcome {
        GameOutcome::Win => 6,
        GameOutcome::Equal => 3,
        GameOutcome::Loss => 0,
    }
}