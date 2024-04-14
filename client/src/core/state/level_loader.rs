use bevy::log::{info, warn};

use crate::core::{event::GameEvent, state::game_event_handler::StateEventHandler};

use super::{GameState, Piece, PieceType};

pub trait StateLevelLoader {
    fn load_level(&mut self, level_id: usize);
}

pub const NUM_LEVELS: usize = 2;

impl GameState {
    pub const LEVELS: [&'static str; NUM_LEVELS] = [
        include_str!("../../../../levels/level1.txt"),
        include_str!("../../../../levels/level2.txt"),
    ];
}

impl StateLevelLoader for GameState {
    fn load_level(&mut self, level_id: usize) {
        if level_id >= Self::LEVELS.len() {
            warn!("Ignoring level loading request as {level_id} is greater than the length of the available LEVELS {}", Self::LEVELS.len());
            return;
        }
        let ld = parse_level_file(Self::LEVELS[level_id]);

        let _ = self.apply_event(GameEvent::SeedRng { seed: ld.seed });

        // update with new level data
        self.num_triangles = ld.num_triangles;
        self.num_squares = ld.num_squares;
        self.num_circles = ld.num_circles;

        for (idx, piece) in ld.pieces.into_iter().enumerate() {
            self.tiles[idx].piece = piece;
        }

        info!("Loaded level {level_id}");
    }
}

/// These files are included in the binary at compile time,
/// I'm being a bit aggressive with asserts and expects here as it shouldn't matter
fn parse_level_file(data: &str) -> LevelData {
    let mut lines = data.lines();

    let seed: u64 = lines
        .next()
        .unwrap()
        .parse()
        .expect("parse seed from level file");

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|line| {
            line.split(',')
                .map(|item| item.parse::<usize>().expect("parse level to usize"))
        })
        .flatten()
        .collect::<Vec<_>>();
    debug_assert_eq!(numbers.len(), 3);

    let pieces = lines
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

    LevelData {
        seed,
        num_triangles: numbers[0],
        num_circles: numbers[1],
        num_squares: numbers[2],
        pieces,
    }
}

pub struct LevelData {
    pub seed: u64,
    pub num_triangles: usize,
    pub num_squares: usize,
    pub num_circles: usize,
    pub pieces: Vec<Piece>,
}
