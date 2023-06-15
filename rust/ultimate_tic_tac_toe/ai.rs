use crate::log;

static mut SCORE: [f32; 0x40000] = [-1000.; 0x40000];

pub unsafe fn init() {
    for me in (0..0x200 as u16).rev() {
        let mut op = !me & 0x1ff;
        loop {
            let mask = (me as usize) << 9 | op as usize;
            if is_win(me) {
                SCORE[mask] = 1.;
            } else if is_win(op) {
                SCORE[mask] = -1.;
            } else if !is_win(!me & 0x1ff) && !is_win(!op & 0x1ff) {
                SCORE[mask] = 0.;
            } else {
                let mut valid_me = Vec::new();
                let mut valid_op = Vec::new();
                for i in 0..9 {
                    if (me >> i | op >> i) & 1 == 1 {
                        continue;
                    }
                    valid_me.push(SCORE[(mask | 1 << i + 9) as usize]);
                    valid_op.push(SCORE[(mask | 1 << i) as usize]);
                }
                valid_me.sort_by(|a, b| b.partial_cmp(a).unwrap());
                valid_op.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mut value_me = 0.;
                let mut value_op = 0.;
                let mut w = 1.;
                let mut w_sum = 0.;
                for (a, b) in valid_me.iter().zip(valid_op.iter()) {
                    value_me += a * w;
                    value_op += b * w;
                    w_sum += w;
                    w /= 4.;
                }
                SCORE[mask] = ((value_me + value_op) / w_sum / 2. as f32).clamp(-1., 1.);
            }
            if op == 0 {
                break;
            }
            op = (op - 1) & !me;
        }
    }
}

fn get_small(table: u128, b: u8) -> u16 {
    (table >> b * 9 & 0b111111111) as u16
}

fn is_win(table: u16) -> bool {
    table & 0b100100100 == 0b100100100 || table & 0b010010010 == 0b010010010 ||
    table & 0b001001001 == 0b001001001 || table & 0b111000000 == 0b111000000 ||
    table & 0b000111000 == 0b000111000 || table & 0b000000111 == 0b000000111 ||
    table & 0b100010001 == 0b100010001 || table & 0b001010100 == 0b001010100
}

pub fn alpha_beta_action(state: &State, depth: u8) -> Option<Action> {
    let mut alpha = f32::MIN;
    let mut best: Option<Action> = None;
    for action in state.valid_actions() {
        let next = state.advanced(&action);
        let score = -alpha_beta(&next, f32::MIN, -alpha, depth);
        log!("{:?}, {}, {}", action, score, next.calc_score());
        if alpha < score {
            alpha = score;
            best = Some(action);
        }
    }
    log!("{:?}", best);
    best
}

pub fn alpha_beta(state: &State, mut alpha: f32, beta: f32, depth: u8) -> f32 {
    if depth == 0 || state.finished() {
        return state.calc_score();
    }
    for action in state.valid_actions() {
        let next = state.advanced(&action);
        let score = -alpha_beta(&next, -beta, -alpha, depth - 1);
        if alpha < score {
            alpha = score;
        }
        if alpha >= beta {
            return alpha;
        }
    }
    alpha
}

#[derive(Debug)]
pub struct Action {
    pub b: u8,
    pub s: u8,
}

#[derive(Copy, Clone)]
pub struct State {
    pub small_win: u128,
    pub small_lose: u128,
    pub big_win: u16,
    pub big_lose: u16,
    pub big_draw: u16,
    pub last_big: i8,
}

impl State {
    pub fn advanced(&self, action: &Action) -> State {
        let small_win = self.small_lose;
        let mut small_lose = self.small_win;
        let big_win = self.big_lose;
        let mut big_lose = self.big_win;
        let mut big_draw = self.big_draw;
        small_lose |= 1_u128 << (action.b * 9 + action.s);
        if is_win(get_small(small_lose, action.b)) {
            big_lose |= 1 << action.b;
        } else if get_small(small_win | small_lose, action.b) == 0x1ff {
            big_draw |= 1 << action.b;
        }
        let free_big = (big_win | big_lose | big_draw) >> action.s & 1 == 1;
        let last_big = if free_big { -1 } else { action.s as i8 };
        State {
            small_win,
            small_lose,
            big_win,
            big_lose,
            big_draw,
            last_big,
        }
    }

    pub fn valid_actions(&self) -> Vec<Action> {
        if self.last_big != -1 {
            let small_win = get_small(self.small_win, self.last_big as u8);
            let small_lose = get_small(self.small_lose, self.last_big as u8);
            let remain = !small_win & !small_lose & 0x1ff;
            let mut actions = Vec::with_capacity(9);
            for s in 0..9 {
                if remain >> s & 1 == 1 {
                    actions.push(Action { b: self.last_big as u8, s })
                }
            }
            actions
        } else {
            let remain_big = !self.big_win & !self.big_lose & !self.big_draw & 0x1ff;
            let remain_small = !self.small_win & !self.small_lose & 0x1ffffffffffffffffffff;
            let mut actions = Vec::with_capacity(81);
            for b in 0..9 {
                if remain_big >> b & 1 == 0 {
                    continue;
                }
                for s in 0..9 {
                    if remain_small >> (b * 9 + s) & 1 == 1 {
                        actions.push(Action { b, s })
                    }
                }
            }
            actions
        }
    }

    pub fn calc_score(&self) -> f32 {
        if is_win(self.big_win) {
            return f32::MAX;
        }
        if is_win(self.big_lose) {
            return f32::MIN;
        }
        if self.big_win | self.big_lose | self.big_draw == 0x1ff {
            let count_win = self.big_win.count_ones();
            let count_lose = self.big_lose.count_ones();
            return if count_win > count_lose {
                f32::MAX
            } else if count_win < count_lose {
                f32::MIN
            } else {
                0.
            }
        }
        let mut score = 0.;
        let mut draw_mask = 0_u16;
        let mut small_scores = [0.; 9];
        unsafe {
            score += SCORE[(self.big_win as usize) << 9 | self.big_lose as usize];
        }
        for i in 0..9 {
            let win = get_small(self.small_win, i as u8);
            let lose = get_small(self.small_lose, i as u8);
            if !is_win(!win & 0x1ff) && !is_win(!lose & 0x1ff) {
                draw_mask |= 1 << i;
            } else {
                unsafe {
                    small_scores[i] = SCORE[(win as usize) << 9 | lose as usize];
                }
                score += small_scores[i] * [3., 2., 3., 2., 4., 2., 3., 2., 3.][i];
                small_scores[i] = (small_scores[i] + 1.) / 2.;
            }
        }
        fn calc(a: f32, b: f32, c: f32) -> f32 {
            let p1 = a * b * c;
            let p2 = (1. - a) * (1. - b) * (1. - c);
            return if p1 > p2 { p1 } else { -p2 }
        }
        let line_scores = [
            if draw_mask & 0b000000111 == 0 { calc(small_scores[0], small_scores[1], small_scores[2]) } else { 0. },
            if draw_mask & 0b000111000 == 0 { calc(small_scores[3], small_scores[4], small_scores[5]) } else { 0. },
            if draw_mask & 0b111000000 == 0 { calc(small_scores[6], small_scores[7], small_scores[8]) } else { 0. },
            if draw_mask & 0b001001001 == 0 { calc(small_scores[0], small_scores[3], small_scores[6]) } else { 0. },
            if draw_mask & 0b010010010 == 0 { calc(small_scores[1], small_scores[4], small_scores[7]) } else { 0. },
            if draw_mask & 0b100100100 == 0 { calc(small_scores[2], small_scores[5], small_scores[8]) } else { 0. },
            if draw_mask & 0b100010001 == 0 { calc(small_scores[0], small_scores[4], small_scores[8]) } else { 0. },
            if draw_mask & 0b001010100 == 0 { calc(small_scores[2], small_scores[4], small_scores[6]) } else { 0. }
        ];
        score += line_scores[0] + line_scores[1] + line_scores[2] +line_scores[3] +
                line_scores[4] + line_scores[5] + line_scores[6] + line_scores[7];
        let min = line_scores[0].min(line_scores[1]).min(line_scores[2]).min(line_scores[3])
                .min(line_scores[4]).min(line_scores[5]).min(line_scores[6]).min(line_scores[7]);
        let max = line_scores[0].max(line_scores[1]).max(line_scores[2]).max(line_scores[3])
                .max(line_scores[4]).max(line_scores[5]).max(line_scores[6]).max(line_scores[7]);
        if min < 0. {
            score += min * 7.;
        }
        if max > 0. {
            score += max * 7.;
        }
        score
    }

    fn finished(&self) -> bool {
        if self.big_win | self.big_lose | self.big_draw == 0x1ff {
            return true;
        }
        return is_win(self.big_win) || is_win(self.big_lose);
    }
}
