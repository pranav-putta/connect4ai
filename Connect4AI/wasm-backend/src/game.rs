use std::cmp::{min, max};
use wasm_bindgen::JsValue;
use std::collections::HashMap;

pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const WIN: usize = 4;

pub type Board = [[bool; COLS]; ROWS];
pub type Heights = [usize; COLS];
pub type TranspositionTable = HashMap<(u64, u64), i8>;

enum Player {
    AI,
    HUMAN,
}

impl Player {
    /// gets the token value for the specified Player
    fn token(self) -> u32 {
        return match self {
            Player::AI => { 0 }
            Player::HUMAN => { 1 }
        };
    }
}

/// Game structure
pub struct Game {
    pub ai: Board,
    pub hum: Board,
    pub heights: Heights,

    col_order: Heights,
    transposition_table: TranspositionTable,
    hash: (u64, u64),
    two_powers_table: HashMap<u8, u64>,
}

impl Game {
    pub fn new(matrix: Vec<Vec<u32>>) -> Game {
        let mut hum: Board = Default::default();
        let mut ai: Board = Default::default();
        let mut heights: Heights = Default::default();

        // generate bit boards
        for r in 0..ROWS {
            for c in 0..COLS {
                hum[r][c] = false;
                ai[r][c] = false;

                if matrix[r][c] == Player::AI.token() {
                    heights[c] += 1;
                    ai[r][c] = true;
                } else if matrix[r][c] == Player::HUMAN.token() {
                    heights[c] += 1;
                    hum[r][c] = true;
                }
            }
        }

        let mut col_order: [usize; COLS] = Default::default();
        for j in 0..COLS {
            let i = j as i32;
            col_order[j] = ((ROWS as i32 / 2) + (1 - 2 * (i % 2)) * (i + 1) / 2) as usize; // initialize the column exploration order, starting with center columns
        }

        let transposition_table: TranspositionTable = HashMap::new();

        // calculate initial hash
        let hash = (Game::hash_board(&ai), Game::hash_board(&hum));

        // initialize two powers table
        let mut two_powers_table: HashMap<u8, u64> = HashMap::new();
        let mut latest = 1;
        for i in 0..(ROWS * COLS) {
            two_powers_table.insert(i as u8, latest);
            latest *= 2;
        }

        Game { ai, hum, heights, col_order, transposition_table, hash, two_powers_table }
    }

    fn hash_board(board: &Board) -> u64 {
        let mut sum = 0;

        let mut p = 1;
        for i in 0..ROWS {
            for j in 0..COLS {
                sum += match board[i][j] {
                    true => { 1 }
                    false => { 0 }
                } * p;
                p *= 2;
            }
        }

        sum
    }

    /// creates a new Game structure from a JS input
    pub fn from(raw_board: JsValue) -> Game {
        let matrix: Vec<Vec<u32>> = raw_board.into_serde().unwrap();
        Game::new(matrix)
    }

    /// insert token at column
    ///
    /// # Arguments
    /// * `col` column to insert at
    fn insert(&mut self, player: &Player, col: usize) -> bool {
        let board = match player {
            Player::AI => { &mut self.ai }
            Player::HUMAN => { &mut self.hum }
        };

        // check if the column is full, otherwise insert was impossible
        if self.heights[col] == ROWS {
            return false;
        }
        // update board and heights
        board[ROWS - self.heights[col] - 1][col] = true;
        self.heights[col] += 1;

        // update hash
        let num = ((ROWS - self.heights[col]) * COLS + col) as u8;
        let cur_ai = self.hash.0;
        let cur_hum = self.hash.1;
        match player {
            Player::AI => {
                self.hash = (cur_ai + *self.two_powers_table.get(&num).unwrap(), cur_hum)
            }
            Player::HUMAN => {
                self.hash = (cur_ai, cur_hum + *self.two_powers_table.get(&num).unwrap())
            }
        }

        true
    }

    /// remove top token at column
    ///
    /// # Arguments
    /// * `player` specified Player
    /// * `col` column to remove at
    fn remove(&mut self, player: &Player, col: usize) {
        let board = match player {
            Player::AI => { &mut self.ai }
            Player::HUMAN => { &mut self.hum }
        };

        // update hash
        let num = ((ROWS - self.heights[col]) * COLS + col) as u8;
        let cur_ai = self.hash.0;
        let cur_hum = self.hash.1;
        match player {
            Player::AI => {
                self.hash = (cur_ai - *self.two_powers_table.get(&num).unwrap(), cur_hum)
            }
            Player::HUMAN => {
                self.hash = (cur_ai, cur_hum - *self.two_powers_table.get(&num).unwrap())
            }
        }

        self.heights[col] -= 1;
        board[ROWS - self.heights[col] - 1][col] = false;
    }

    /// checks winner for specified player
    ///
    /// # Arguments
    /// * `player` player to check
    fn check_winner(&mut self, player: &Player) -> bool {
        let board = match player {
            Player::AI => { &mut self.ai }
            Player::HUMAN => { &mut self.hum }
        };

        for r in 0..ROWS {
            for c in 0..COLS {
                if (r > ROWS - WIN && c > COLS - WIN) || !board[r][c] {
                    // no need to check through positions
                    continue;
                }
                let mut ct = 1;

                // check right
                if c <= COLS - WIN {
                    for i in 1..WIN {
                        if !board[r][c + i] {
                            break;
                        }
                        ct += 1;
                    }


                    if ct == WIN {
                        return true;
                    }
                }

                // check down
                if r <= ROWS - WIN {
                    ct = 1;
                    for i in 1..WIN {
                        if !board[r + i][c] {
                            break;
                        }
                        ct += 1;
                    }

                    if ct == WIN {
                        return true;
                    }
                }

                // check r-d diagonal
                if r <= ROWS - WIN && c <= COLS - WIN {
                    ct = 1;
                    for i in 1..WIN {
                        if !board[r + i][c + i] {
                            break;
                        }
                        ct += 1;
                    }

                    if ct == WIN {
                        return true;
                    }
                }

                // check r-u diagonal
                if r >= (WIN - 1) && c <= COLS - WIN {
                    ct = 1;
                    for i in 1..WIN {
                        if !board[r - i][c + i] {
                            break;
                        }
                        ct += 1;
                    }

                    if ct == WIN {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    /// calculates minimax score array
    pub fn minimax(&mut self, max_depth: u32) -> [i8; COLS] {
        let mut scores: [i8; COLS] = Default::default();

        for i in 0..COLS {
            let col = self.col_order[i];
            let inserted = self.insert(&Player::AI, col);
            if inserted {
                // perform score calculation
                scores[col] = self.minimax_util(false, 1, max_depth, i8::MIN, i8::MAX);
                self.remove(&Player::AI, col);
            } else {
                // if not insertable, definitely don't choose this one
                scores[col] = i8::MIN;
            }
        }

        scores
    }

    fn minimax_util(&mut self, maximizer: bool, depth: u32, max_depth: u32, mut alpha: i8, mut beta: i8) -> i8 {
        // check if state has already been stored
        let state = self.hash;
        if self.transposition_table.contains_key(&state) {
            return *self.transposition_table.get(&state).unwrap();
        }

        let (player, opponent) = &match maximizer {
            true => { (Player::AI, Player::HUMAN) }
            false => { (Player::HUMAN, Player::AI) }
        };

        // before playing current player, check if opponent has already won
        if self.check_winner(opponent) {
            return (((ROWS * COLS) as u32 - depth) / 2) as i8 * if maximizer { -1 } else { 1 };
        }

        // base case
        if depth >= max_depth {
            return 0;
        }

        // otherwise recurse and find best score
        let mut best = if maximizer { i8::MIN } else { i8::MAX };
        for i in 0..COLS {
            let col = self.col_order[i];
            let inserted = self.insert(player, col);

            // if insert wasn't possible, ignore this subtree
            if inserted {
                let score = self.minimax_util(!maximizer, depth + 1, max_depth, alpha, beta);

                if maximizer {
                    best = max(best, score);
                    alpha = max(alpha, best);
                    if beta <= alpha {
                        self.remove(player, col);
                        return best;
                    }
                } else {
                    best = min(best, score);
                    beta = min(beta, best);
                    if beta <= alpha {
                        self.remove(player, col);
                        return best;
                    }
                }
                self.remove(player, col);
            }
        }
        // store state in memory
        self.transposition_table.insert(state, best);
        best
    }
}
