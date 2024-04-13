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
        let pieces = parse_level_file(Self::LEVELS[level_id]);

        for (idx, piece) in pieces.into_iter().enumerate() {
            self.tiles[idx].piece = piece;
        }
    }
}

fn parse_level_file(data: &str) -> Vec<Piece> {
    let data = data
        .lines()
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

    debug_assert_eq!(data.len(), 64);
    data
}
