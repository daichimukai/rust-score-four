use rand::Rng;

use crate::action::Action;
use crate::board::{Board, BoardStatus};
use crate::color::{Color, Color::Black, Color::White};

/// Player Trait.
#[allow(unused_variables)]
pub trait Player {
    fn new(color: Color) -> Self;
    fn step(&self, board: &Board) -> Action;
    fn end(&self, board: &Board) {}
}

/// Almost random player.
pub struct PlayerRandom {
    color: Color,
    goal: BoardStatus,
}

impl Player for PlayerRandom {
    fn new(color: Color) -> Self {
        PlayerRandom {
            color: color,
            goal: match color {
                White => BoardStatus::WhiteWin,
                Black => BoardStatus::BlackWin,
            },
        }
    }

    fn step(&self, board: &Board) -> Action {
        random_or_win(board, self.goal)
    }
}

/// Primitive Monte-Carlo Player.
pub struct PlayerMC {
    color: Color,
    goal: BoardStatus,
}

impl Player for PlayerMC {
    fn new(color: Color) -> Self {
        PlayerMC {
            color: color,
            goal: match color {
                White => BoardStatus::WhiteWin,
                Black => BoardStatus::BlackWin,
            },
        }
    }

    fn step(&self, board: &Board) -> Action {
        let actions = board.possible_actions();
        assert!(actions.len() > 0);
        let mut ret = actions[0];
        let mut max: isize = 0;
        for i in 0..actions.len() {
            let mut board = board.clone();
            let action = actions[i];
            board.step(action);

            let mut score: isize = 0;
            for _ in 0..50 {
                let result = self.simulate(&board);
                assert_ne!(result, BoardStatus::Ongoing);

                if result == self.goal {
                    score += 1;
                } else if result != BoardStatus::Draw {
                    score -= 1;
                }
            }

            if score > max {
                max = score;
                ret = action;
            }
        }

        ret
    }
}

impl PlayerMC {
    #[allow(unused_must_use)]
    fn simulate(&self, board: &Board) -> BoardStatus {
        let mut board = board.clone();
        while board.status() == BoardStatus::Ongoing {
            let action = random_or_win(&board, self.goal);
            board.step(action);
        }

        board.status()
    }
}

fn random_or_win(board: &Board, goal: BoardStatus) -> Action {
    let mut board = board.clone();
    let actions = board.possible_actions();
    for action in actions.iter() {
        let result = board.step(*action);
        assert!(result.is_ok());

        if board.status() == goal {
            return *action;
        }
    }

    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, actions.len());
    actions[index]
}
