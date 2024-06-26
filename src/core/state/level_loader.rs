use bevy::log::{info, warn};

use crate::core::{event::GameEvent, state::game_event_handler::StateEventHandler};

use super::{GameState, Piece, PieceType};

pub trait StateLevelLoader {
    fn load_level(&mut self, level_id: usize);
}

pub const NUM_LEVELS: usize = 5;

impl GameState {
    pub const LEVELS: [&'static str; NUM_LEVELS] = [
        include_str!("../../../levels/tutorial.txt"),
        include_str!("../../../levels/level1.txt"),
        include_str!("../../../levels/level2.txt"),
        include_str!("../../../levels/level3.txt"),
        include_str!("../../../levels/level4.txt"),
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

        self.level_message = ld.intro;

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

    let intro = lines.next().unwrap().to_owned();

    let seed: u64 = lines
        .next()
        .unwrap()
        .parse()
        .expect("parse seed from level file");

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(|line| {
            line.split(',')
                .map(|item| item.parse::<usize>().expect("parse level to usize"))
        })
        .collect::<Vec<_>>();
    debug_assert_eq!(numbers.len(), 3);

    let pieces = lines
        .flat_map(|line| {
            line.split(',')
                .map(|i| match i {
                    "0" | "00" => Piece::Empty,
                    "1" | "01" => Piece::Player0(PieceType::Hound),
                    "2" | "02" => Piece::Player0(PieceType::Swordsman),
                    "3" | "03" => Piece::Player0(PieceType::Bowman),
                    "11" => Piece::Player1(PieceType::Hound),
                    "12" => Piece::Player1(PieceType::Swordsman),
                    "13" => Piece::Player1(PieceType::Bowman),
                    "99" => Piece::Obstacle(PieceType::Wall),
                    v => {
                        panic!(
                            "Failed to load text file - found {v}, expected 0,1,2,3,11,12,13 or 99"
                        )
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    debug_assert_eq!(pieces.len(), 64);

    LevelData {
        seed,
        intro,
        num_triangles: numbers[0],
        num_circles: numbers[1],
        num_squares: numbers[2],
        pieces,
    }
}

pub struct LevelData {
    pub intro: String,
    pub seed: u64,
    pub num_triangles: usize,
    pub num_squares: usize,
    pub num_circles: usize,
    pub pieces: Vec<Piece>,
}
