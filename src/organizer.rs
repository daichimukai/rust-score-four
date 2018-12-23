use action::Action;
use board::{Board, BoardStatus};
use color::Color;
use player::Player;

pub struct Organizer<T1: Player, T2: Player> {
    player_white: T1,
    player_black: T2,
    board: Board,
    turn: Color,
}

impl<T1: Player, T2: Player> Organizer<T1, T2> {
    pub fn new(player_white: T1, player_black: T2) -> Self {
        Organizer {
            player_white: player_white,
            player_black: player_black,
            board: Board::new(),
            turn: Color::White,
        }
    }

    pub fn reset(&mut self) {
        self.board = Board::new();
        self.turn = Color::White;
    }

    pub fn start(&mut self) {
        while self.board.status() == BoardStatus::Ongoing {
            let action: Action;
            match self.turn {
                Color::White => action = self.player_white.step(&self.board),
                Color::Black => action = self.player_black.step(&self.board),
            }
            self.board.step(action);
            self.turn = !self.turn
        }

        self.player_white.end(&self.board);
        self.player_black.end(&self.board);
    }

    pub fn report(&self) {
        match self.board.status() {
            BoardStatus::WhiteWin => println!("{}", "White win!"),
            BoardStatus::BlackWin => println!("{}", "Black win!"),
            BoardStatus::Draw => println!("{}", "Draw."),
            _ => panic!(""),
        }
    }
}
