use std::cmp::Ordering;
use itertools::Itertools;
use crate::ultimate_tic_tac_toe::ai::{SCORE, ZOBRIST, State, Action, SCORE_WIN, Timer, MatchResult};
use eprintln as log;

pub fn calc_action(state: &State, timer: &Timer, logging: bool) -> Action {
    if state.small_lose | state.small_win == 0 {
        return Action { b: 4, s: 4, anywhere: false, score: 0. };
    }
    let mut root = Node::new(state.clone());
    root.expand();
    let mut iter = 0;
    loop {
        if iter % 100 == 0 && timer.elapsed() {
            break
        }
        iter += 1;
        root.evaluate();
    }
    if logging {
        log!("{} iter", iter);
    }
    let mut actions = state.valid_actions();
    let mut max = 0;
    let mut best_action = actions[0];
    for i in 0..actions.len() {
        let child = &root.children[i];
        if max < child.n {
            max = child.n;
            best_action = actions[i]
        }
    }
    best_action
}

const EXPAND_THRESHOLD: usize = 10;

struct Node {
    state: State,
    w: f32,
    n: usize,
    children: Vec<Node>,
}

impl Node {
    fn new(state: State) -> Self {
        Node {
            state,
            w: 0.,
            n: 0,
            children: vec![],
        }
    }

    fn evaluate(&mut self) -> f32 {
        if self.state.finished() {
            let value = match self.state.winner().unwrap() {
                MatchResult::Win => 1.,
                MatchResult::Lose => -1.,
                MatchResult::Draw => 0.,
            };
            self.w += value;
            self.n += 1;
            value
        } else if self.children.is_empty() {
            let value = self.playout();
            self.w += value;
            self.n += 1;
            if self.n == EXPAND_THRESHOLD {
                self.expand();
            }
            value
        } else {
            let next = self.next_child_node();
            let value = -self.children[next].evaluate();
            self.w += value;
            self.n += 1;
            value
        }
    }

    fn expand(&mut self) {
        for action in self.state.valid_actions() {
            self.children.push(Node::new(self.state.advanced(&action)));
        }
    }

    fn next_child_node(&self) -> usize {
        let mut t = 0;
        for (i, child) in self.children.iter().enumerate() {
            if child.n == 0 {
                return i;
            }
            t += child.n;
        }
        let mut best_value = f32::MIN;
        let mut best = 0;
        for (i, child) in self.children.iter().enumerate() {
            let ucb1 = child.ucb1(t);
            if best_value < ucb1 {
                best_value = ucb1;
                best = i;
            }
        }
        best
    }

    fn playout(&self) -> f32 {
        let mut state = self.state.clone();
        let mut value = 1.;
        loop {
            let action = state.random_action();
            state.advance_self(&action);
            value = -value;
            return match state.winner() {
                Some(MatchResult::Win) => value,
                Some(MatchResult::Lose) => -value,
                Some(MatchResult::Draw) => 0.,
                None => continue,
            }
        }
    }

    fn ucb1(&self, total: usize) -> f32 {
        if self.n == 0 {
            f32::MAX
        } else {
            -self.w / self.n as f32 + (2. * (total as f32).ln() / (self.n as f32)).sqrt()
        }
    }
}
