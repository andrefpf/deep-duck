
use std::collections::HashMap;

use crate::board::Board;
use crate::pieces::Piece;
use crate::pieces::Color;
use crate::engine::Evaluation;
use crate::pieces::PieceKind;

const ZOBRIST_SIZE: usize = 64*7*3;

pub struct ZobristCache {
    random_values: [u32; ZOBRIST_SIZE],
    data: HashMap<u32, Evaluation>,
}

impl ZobristCache {
    pub fn new() -> Self {
        
        
        ZobristCache {
            random_values: [(); ZOBRIST_SIZE].map(|_| rand::random()),  // random values for every position
            data: HashMap::new(),
        }
    }

    pub fn insert(&mut self, board: &Board, eval: Evaluation) {
        self.data.insert(self.zobrist_hash(board), eval);
    }

    pub fn get(&mut self, board: &Board) -> Option<Evaluation> {
        self.data.get(&self.zobrist_hash(board)).copied()
    }

    fn zobrist_hash(&self, board: &Board) -> u32 {
        let mut hash: u32 = 0;

        for i in board.ocuppied_squares().map(|x| self.zobrist_value(*x)) {
            hash ^= i;
        }

        hash
    }

    fn zobrist_value(&self, piece: Piece) -> u32 {
        let color_index = match piece.color {
            Color::White => 0,
            Color::Black => 1,
            Color::Yellow => 2
        };

        let kind_index = match piece.kind {
            PieceKind::Pawn => 0,
            PieceKind::Rook => 1,
            PieceKind::Knight => 2,
            PieceKind::Bishop => 3,
            PieceKind::Queen => 4,
            PieceKind::King => 5,
            PieceKind::Duck => 6,
        };

        let pos_index = piece.pos.0 + 8 * piece.pos.1;
        let index = pos_index + color_index*64 + kind_index*64*2;
        self.random_values[index as usize]
    }
}