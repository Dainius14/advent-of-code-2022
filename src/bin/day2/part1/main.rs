use std::{fs, io::{self, BufRead}};

fn main() {
    // let contents = fs::read_to_string("data/day2_input.txt")
    //     .expect("Should have been able to read file");
    
    // let lines = contents.split('\n');

    let file = fs::File::open("data/day2_input.txt").expect("Should be able to open the file");
    let reader = io::BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {

        let line_str = line.unwrap();
        let line_bytes = line_str.as_bytes();
        let enemy_attack_char = line_bytes[0];
        let my_attack_char = line_bytes[2];

        let enemy_attack = get_attack_type_from_enemy(&enemy_attack_char);
        let my_attack = get_attack_type_from_me(&my_attack_char);

        let game_outcome = get_game_outcome(enemy_attack, my_attack);

        let points_for_game = get_points_for_game_outcome(game_outcome);
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

fn get_attack_type_from_me(str: &u8) -> AttackType {
    match str {
        b'X' => AttackType::Rock,
        b'Y' => AttackType::Paper,
        b'Z' => AttackType::Sciccors,
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

fn get_game_outcome(enemy_attack: AttackType, my_attack: AttackType) -> GameOutcome {
    if enemy_attack == my_attack {
        GameOutcome::Equal
    }

    else if my_attack == AttackType::Rock {
        if enemy_attack == AttackType::Sciccors {
            GameOutcome::Win
        }
        else {
            GameOutcome::Loss
        }
    }

    else if my_attack == AttackType::Paper {
        if enemy_attack == AttackType::Sciccors {
            GameOutcome::Loss
        }
        else {
            GameOutcome::Win
        }
    }

    else if enemy_attack == AttackType::Paper {
        GameOutcome::Win
    }
    else {
        GameOutcome::Loss
    }
}

fn get_points_for_game_outcome(outcome: GameOutcome) -> i32 {
    match outcome {
        GameOutcome::Win => 6,
        GameOutcome::Equal => 3,
        GameOutcome::Loss => 0,
    }
}