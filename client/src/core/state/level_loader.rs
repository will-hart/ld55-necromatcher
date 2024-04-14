use bevy::log::warn;

use super::{GameState, Piece, PieceType};

pub trait StateLevelLoader {
    fn load_level(&mut self, level_id: usize);
}

impl GameState {
    pub const LEVELS: [&'static str; 1] = [include_str!("../../../../levels/level1.txt")];
}

impl StateLevelLoader for GameState {
    fn load_level(&mut self, level_id: usize) {
        if level_id >= Self::LEVELS.len() {
            warn!("Ignoring level loading request as {level_id} is greater than the length of the available LEVELS {}", Self::LEVELS.len());
            return;
        }
        let ((num_tris, num_circles, num_squares), pieces) =
            parse_level_file(Self::LEVELS[level_id]);

        self.num_triangles = num_tris;
        self.num_squares = num_squares;
        self.num_circles = num_circles;

        for (idx, piece) in pieces.into_iter().enumerate() {
            self.tiles[idx].piece = piece;
        }
    }
}

fn parse_level_file(data: &str) -> ((usize, usize, usize), Vec<Piece>) {
    let pieces = data
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|i| match i {
                    "0" => Piece::Empty,
                    "1" => Piece::Player0(PieceType::Circle),
                    "2" => Piece::Player0(PieceType::Square),
                    "3" => Piece::Player0(PieceType::Triangle),
                    "11" => Piece::Player1(PieceType::Circle),
                    "12" => Piece::Player1(PieceType::Square),
                    "13" => Piece::Player1(PieceType::Triangle),
                    v => {
                        panic!("Failed to load text file - found {v}, expected 0,1,2,3,11,12 or 13")
                    }
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    debug_assert_eq!(pieces.len(), 64);

    let numbers = data
        .lines()
        .take(1)
        .map(|line| {
            line.split(',')
                .map(|item| item.parse::<usize>().expect("parse level to usize"))
        })
        .flatten()
        .collect::<Vec<_>>();
    debug_assert_eq!(numbers.len(), 3);

    ((0, 0, 0), pieces)
}
