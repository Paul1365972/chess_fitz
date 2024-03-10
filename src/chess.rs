use game_data::Game;
use move_mappings::id_to_uci_move;
use shakmaty::{Chess, Color, Position};
use std::io::{self, Read};

pub fn next_turn(chess: &mut Chess, move_id: u16) {
    let mut uci_move = id_to_uci_move(move_id).unwrap();
    if chess.turn() == Color::Black {
        uci_move.flip();
    }
    uci_move.unsanitize_promotion(chess);
    let the_move = uci_move.0.to_move(chess).unwrap();
    chess.play_unchecked(&the_move);
}

pub fn next_game<R: Read>(reader: &mut R) -> anyhow::Result<Option<Game>> {
    let result: Result<Game, _> = bincode::deserialize_from(reader);
    return match result {
        Err(err) => match *err {
            bincode::ErrorKind::Io(err) if err.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
            _ => Err(err.into()),
        },
        Ok(game) => Ok(Some(game)),
    };
}
