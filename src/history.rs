use shakmaty::{Bitboard, Chess, Position};

const HISTORY_LEN: usize = 8;
const BASE_PLANES: usize = 12 * HISTORY_LEN;
const TOTAL_PLANES: usize = BASE_PLANES + 4;

pub struct PositionHistory(pub [u64; TOTAL_PLANES], [u64; TOTAL_PLANES]);

impl PositionHistory {
    pub fn new() -> Self {
        PositionHistory(std::array::from_fn(|_| 0), std::array::from_fn(|_| 0))
    }

    pub fn update(&mut self, chess: &Chess) {
        let board = chess.board();
        let castle_rights = chess.castles().castling_rights();

        // Update whites history
        self.0.copy_within(12.., 0);

        self.0[BASE_PLANES - 12] = (board.white() | board.pawns()).0;
        self.0[BASE_PLANES - 11] = (board.white() | board.knights()).0;
        self.0[BASE_PLANES - 10] = (board.white() | board.bishops()).0;
        self.0[BASE_PLANES - 9] = (board.white() | board.rooks()).0;
        self.0[BASE_PLANES - 8] = (board.white() | board.queens()).0;
        self.0[BASE_PLANES - 7] = (board.white() | board.kings()).0;

        self.0[BASE_PLANES - 6] = (board.black() | board.pawns()).0;
        self.0[BASE_PLANES - 5] = (board.black() | board.knights()).0;
        self.0[BASE_PLANES - 4] = (board.black() | board.bishops()).0;
        self.0[BASE_PLANES - 3] = (board.black() | board.rooks()).0;
        self.0[BASE_PLANES - 2] = (board.black() | board.queens()).0;
        self.0[BASE_PLANES - 1] = (board.black() | board.kings()).0;

        self.0[BASE_PLANES + 0] = (castle_rights | Bitboard::WEST).0;
        self.0[BASE_PLANES + 1] = (castle_rights | Bitboard::EAST).0;
        self.0[BASE_PLANES + 2] = Bitboard::EMPTY.0;
        self.0[BASE_PLANES + 3] = Bitboard::FULL.0;

        // Update blacks history
        self.1.copy_within(12.., 0);

        self.0[BASE_PLANES - 12] = (board.black() | board.pawns()).flip_vertical().0;
        self.0[BASE_PLANES - 11] = (board.black() | board.knights()).flip_vertical().0;
        self.0[BASE_PLANES - 10] = (board.black() | board.bishops()).flip_vertical().0;
        self.0[BASE_PLANES - 9] = (board.black() | board.rooks()).flip_vertical().0;
        self.0[BASE_PLANES - 8] = (board.black() | board.queens()).flip_vertical().0;
        self.0[BASE_PLANES - 7] = (board.black() | board.kings()).flip_vertical().0;

        self.0[BASE_PLANES - 6] = (board.white() | board.pawns()).flip_vertical().0;
        self.0[BASE_PLANES - 5] = (board.white() | board.knights()).flip_vertical().0;
        self.0[BASE_PLANES - 4] = (board.white() | board.bishops()).flip_vertical().0;
        self.0[BASE_PLANES - 3] = (board.white() | board.rooks()).flip_vertical().0;
        self.0[BASE_PLANES - 2] = (board.white() | board.queens()).flip_vertical().0;
        self.0[BASE_PLANES - 1] = (board.white() | board.kings()).flip_vertical().0;

        self.0[BASE_PLANES + 0] = (castle_rights | Bitboard::WEST).flip_vertical().0;
        self.0[BASE_PLANES + 1] = (castle_rights | Bitboard::EAST).flip_vertical().0;
        self.0[BASE_PLANES + 2] = Bitboard::FULL.0;
        self.0[BASE_PLANES + 3] = Bitboard::FULL.0;
    }
}
