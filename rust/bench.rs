mod ultimate_tic_tac_toe;

use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::io::copy;
use std::time::{Duration, Instant};
use crate::ultimate_tic_tac_toe::ai::*;
use crate::ultimate_tic_tac_toe::strategy_alpha_beta::{alpha_beta, calc_action as alpha_beta_action, calc_action};
use crate::ultimate_tic_tac_toe::strategy_alpha_beta_2::calc_action as alpha_beta_2_action;
use crate::ultimate_tic_tac_toe::strategy_alpha_beta_transpose::calc_action as alpha_beta_transpose_action;
use crate::ultimate_tic_tac_toe::strategy_mcts::calc_action as mcts_action;
use crate::ultimate_tic_tac_toe::strategy_random::calc_action as random_action;
use crate::ultimate_tic_tac_toe::strategy_negascout::calc_action as negascout_action;

fn main() {








    


    init();
    // let dataset_1 = load_dataset();
    // let dataset_2 = load_dataset_from_bin();
    // for (datapoint_1, datapoint_2) in dataset_1.iter().zip(dataset_2) {
    //     // println!("{}, {}", calc_eval(&datapoint.state), -alpha_beta(&datapoint.state, f32::MIN, f32::MAX, 0, 8, &Timer::new(&Duration::from_millis(10000))));
    //
    //     if datapoint_1.state.big_draw != datapoint_2.state.big_draw {
    //         println!("{:?}, {:?}, {:?}, {:?}", datapoint_1.visit_count, datapoint_1.state_value_mean, datapoint_1.state.last_big, datapoint_1.state.turn);
    //         print!("{:?}", datapoint_1.state);
    //         println!("{:?}, {:?}, {:?}, {:?}", datapoint_2.visit_count, datapoint_2.state_value_mean, datapoint_2.state.last_big, datapoint_2.state.turn);
    //         print!("{:?}", datapoint_2.state);
    //     }
    //     assert_eq!(datapoint_1.depth, datapoint_2.depth);
    //     assert_eq!(datapoint_1.visit_count, datapoint_2.visit_count);
    //     assert_eq!(datapoint_1.state_value_mean, datapoint_2.state_value_mean);
    //     assert_eq!(datapoint_1.state.small_win, datapoint_2.state.small_win);
    //     assert_eq!(datapoint_1.state.small_lose, datapoint_2.state.small_lose);
    //     assert_eq!(datapoint_1.state.big_win, datapoint_2.state.big_win);
    //     assert_eq!(datapoint_1.state.big_lose, datapoint_2.state.big_lose);
    //     assert_eq!(datapoint_1.state.big_draw, datapoint_2.state.big_draw);
    //     assert_eq!(datapoint_1.state.turn, datapoint_2.state.turn);
    //     assert_eq!(datapoint_1.state.last_big, datapoint_2.state.last_big);
    //     for (a1, a2) in datapoint_1.evaluated_actions.iter().zip(datapoint_2.evaluated_actions) {
    //         assert_eq!(a1.index, a2.index);
    //         assert_eq!(a1.visit_count, a2.visit_count);
    //         assert_eq!(a1.state_value_mean, a2.state_value_mean);
    //     }
    //     // for action in datapoint.evaluated_actions {
    //     //     println!("{:?}, {:?}, {:?}, {:?}", action.symbol, action.index, action.visit_count, action.state_value_mean);
    //     // }
    // }

    let dataset = load_dataset("subset_1_1000.bin");
    println!("{}", dataset.len());
    let mut same = 0;
    for (i, datapoint) in &dataset.iter().enumerate() {
        let mut actions1 = vec![];
        for action in datapoint.state.valid_actions_with_move_ordering() {
            let advanced = datapoint.state.advanced(&action);
            let timer = Timer::new(&Duration::from_secs(1));
            let eval = -alpha_beta(&advanced, f32::MIN, f32::MAX, 0, 7, &timer);
            actions1.push((action.b, action.s, eval));
        }
        let mut actions2 = vec![];
        for action in &datapoint.evaluated_actions {
            actions2.push((action.index / 9, action.index % 9, action.state_value_mean));
        }
        actions1.sort_unstable_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        actions2.sort_unstable_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        println!("{} {}", i, datapoint.state_value_mean);
        println!("{:?}", actions1);
        println!("{:?}", actions2);
        if actions1[0].0 == actions2[0].0 && actions1[0].1 == actions2[0].1 {
            same += 1;
        }
    }
    println!("{} / {}", same, dataset.len());



    // save_dataset(&dataset_1);
    // for (datapoint_1, datapoint_2) in dataset_1.iter().zip(dataset_1) {
    //     assert_eq!(datapoint_1.depth, datapoint_2.depth);
    //     assert_eq!(datapoint_1.visit_count, datapoint_2.visit_count);
    //     assert_eq!(datapoint_1.state_value_mean, datapoint_2.state_value_mean);
    //     assert_eq!(datapoint_1.state.small_win, datapoint_2.state.small_win);
    //     assert_eq!(datapoint_1.state.small_lose, datapoint_2.state.small_lose);
    //     assert_eq!(datapoint_1.state.big_win, datapoint_2.state.big_win);
    //     assert_eq!(datapoint_1.state.big_lose, datapoint_2.state.big_lose);
    //     assert_eq!(datapoint_1.state.big_draw, datapoint_2.state.big_draw);
    //     assert_eq!(datapoint_1.state.turn, datapoint_2.state.turn);
    //     assert_eq!(datapoint_1.state.last_big, datapoint_2.state.last_big);
    //     for (a1, a2) in datapoint_1.evaluated_actions.iter().zip(datapoint_2.evaluated_actions) {
    //         assert_eq!(a1.index, a2.index);
    //         assert_eq!(a1.visit_count, a2.visit_count);
    //         assert_eq!(a1.state_value_mean, a2.state_value_mean);
    //     }
    // }


    // let mut n = 0;
    // let mut w = 0;
    // for _ in 0..50 {
    //     let player_1: fn(&State) -> Action = |state| alpha_beta_action(state, &Timer::new(&Duration::from_millis(50)), false);
    //     let player_2: fn(&State) -> Action = |state| mcts_action(state, &Timer::new(&Duration::from_millis(250)), false);
    //     n += 2;
    //     println!("{} {:3} tries {:5.1} %", match play(&player_1, &player_2) {
    //         Winner::First => { w += 2; "1p  (first)" },
    //         Winner::Second => "2p (second)",
    //         Winner::Draw => { w += 1; "draw       " },
    //     }, n / 2, 100. * w as f32 / n as f32);
    //     n += 2;
    //     println!("{} {:3} tries {:5.1} %", match play(&player_2, &player_1) {
    //         Winner::First => "2p  (first)",
    //         Winner::Second => { w += 2; "1p (second)" },
    //         Winner::Draw => { w += 1; "draw       " },
    //     }, n / 2, 100. * w as f32 / n as f32);
    // }
}

fn play(player_1: &fn(&State) -> Action, player_2: &fn(&State) -> Action) -> Winner {
    let mut state = State::new();
    let mut turn = true;
    loop {
        let action = if turn {
            player_1(&state)
        } else {
            player_2(&state)
        };
        turn = !turn;
        state = state.advanced(&action);
        return match state.winner() {
            Some(MatchResult::Win) if turn => Winner::First,
            Some(MatchResult::Lose) if turn => Winner::Second,
            Some(MatchResult::Win) => Winner::Second,
            Some(MatchResult::Lose) => Winner::First,
            Some(MatchResult::Draw) => Winner::Draw,
            _ => continue,
        }
    }
}

fn parse_state(token: &[u8]) -> State {
    let mut small_win = 0_u128;
    let mut small_lose = 0_u128;
    let mut big_win = 0_u16;
    let mut big_lose = 0_u16;
    let mut big_draw = 0_u16;
    for i in 0..81 {
        match token[i] as char {
            '0' => {}
            '1' => {
                small_win |= 1_u128 << i
            }
            '2' => {
                small_lose |= 1_u128 << i
            }
            _ => unreachable!()
        }
    }
    for i in 0..9 {
        match token[i + 81] as char {
            '0' => {}
            '1' => {
                big_win |= 1 << i
            }
            '2' => {
                big_lose |= 1 << i
            }
            '3' => {
                big_draw |= 1 << i
            }
            _ => unreachable!()
        }
    }
    State {
        small_win,
        small_lose,
        big_win,
        big_lose,
        big_draw,
        last_big: (token[91] - '0' as u8) as u16,
        hash: 0,
        turn: token[90] == '2' as u8,
    }
}

use bytes::{Buf, BufMut, BytesMut};
use rand::seq::SliceRandom;

fn load_dataset(name: &str) -> Vec<Datapoint> {
    let start = Instant::now();
    let mut writer = BytesMut::new().writer();
    copy(&mut File::open(format!("../dataset/{}", name)).unwrap(), &mut writer).unwrap();
    let mut buf = writer.into_inner();
    let mut dataset = vec![];
    while buf.has_remaining() {
        let depth = buf.get_u8() as usize;
        let sign = if depth % 2 == 0 { -1. } else { 1. };
        let visit_count = buf.get_u8() as usize * 10000;
        let state_value_mean = buf.get_f32() * sign;
        let mut state = State {
            small_win: buf.get_u128(),
            small_lose: buf.get_u128(),
            big_win: 0,
            big_lose: 0,
            big_draw: 0,
            last_big: buf.get_u8() as u16,
            hash: 0,
            turn: depth % 2 == 1,
        };
        for i in 0..9 {
            if is_win(get_small(state.small_win, i)) {
                state.big_win |= 1 << i;
            } else if is_win(get_small(state.small_lose, i)) {
                state.big_lose |= 1 << i;
            } else if get_small(state.small_win | state.small_lose, i) == 0x1ff {
                state.big_draw |= 1 << i;
            }
        }
        let len = buf.get_u8() as usize;
        let mut evaluated_actions = Vec::with_capacity(len);
        for _ in 0..len {
            evaluated_actions.push(EvaluatedAction {
                index: buf.get_u8() as u16,
                visit_count: buf.get_u32() as usize,
                state_value_mean: buf.get_f32() * -sign,
            })
        }
        dataset.push(Datapoint {
            depth,
            state,
            visit_count,
            state_value_mean,
            evaluated_actions
        });
    }
    println!("{:?}", start.elapsed());
    dataset
}

fn save_dataset(dataset: &Vec<&Datapoint>, name: &str) {
    let mut buf = BytesMut::new();
    for datapoint in dataset {
        buf.put_u8(datapoint.depth as u8);
        buf.put_u8((datapoint.visit_count / 10000) as u8);
        buf.put_f32(datapoint.state_value_mean);
        buf.put_u128(datapoint.state.small_win);
        buf.put_u128(datapoint.state.small_lose);
        buf.put_u8(datapoint.state.last_big as u8);
        buf.put_u8(datapoint.evaluated_actions.len() as u8);
        for action in &datapoint.evaluated_actions {
            buf.put_u8(action.index as u8);
            buf.put_u32(action.visit_count as u32);
            buf.put_f32(action.state_value_mean);
        }
    }
    let mut file = File::create(format!("../dataset/{}", name)).unwrap();
    file.write(buf.as_ref());
}

// fn load_dataset() -> Vec<Datapoint> {
//     let start = Instant::now();
//     let mut dataset = vec![];
//     for (depth, file) in fs::read_dir("../stage2-nmcts").unwrap().enumerate() {
//         let entry = file.unwrap();
//         println!("{:?}", entry.file_name());
//         for datapoint in BufReader::new(File::open(entry.path()).unwrap()).lines() {
//             let unwrapped = datapoint.unwrap();
//             let tokens = unwrapped.split(['{', '}', ' ', ',']).collect::<Vec<_>>();
//             let evaluated_actions = tokens[6..].chunks_exact(4).map(|splitted| {
//                 EvaluatedAction {
//                     index: splitted[1].parse().unwrap(),
//                     visit_count: splitted[2].parse().unwrap(),
//                     state_value_mean: splitted[3].parse().unwrap(),
//                 }
//             }).collect();
//             let datapoint = Datapoint {
//                 depth,
//                 state: parse_state(tokens[1].as_bytes()),
//                 visit_count: tokens[2].parse().unwrap(),
//                 state_value_mean: tokens[3].parse().unwrap(),
//                 evaluated_actions
//             };
//             dataset.push(datapoint);
//         }
//     }
//     println!("{:?}", start.elapsed());
//     dataset
// }

struct Datapoint {
    depth: usize,
    state: State,
    visit_count: usize,
    state_value_mean: f32,
    evaluated_actions: Vec<EvaluatedAction>
}

struct EvaluatedAction {
    index: u16,
    visit_count: usize,
    state_value_mean: f32,
}

#[derive(Debug)]
pub enum Winner {
    First, Second, Draw
}
