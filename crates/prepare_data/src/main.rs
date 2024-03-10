use std::{fs::File, mem};

use game_data::{Game, UciMove};
use move_mappings::uci_move_to_id;
use shakmaty::{CastlingMode, Chess, Color, Position};

struct Visitor {
    invalid: bool,
    white_elo: Option<u16>,
    black_elo: Option<u16>,
    chess: Chess,
    moves: Vec<u16>,
}

impl Visitor {
    fn new() -> Self {
        Self {
            invalid: false,
            white_elo: None,
            black_elo: None,
            chess: Chess::new(),
            moves: Vec::new(),
        }
    }
}

impl pgn_reader::Visitor for Visitor {
    type Result = Option<Game>;

    fn begin_game(&mut self) {
        *self = Visitor::new();
    }

    fn header(&mut self, key: &[u8], value: pgn_reader::RawHeader<'_>) {
        match key {
            b"WhiteElo" => {
                self.white_elo = Some(value.decode_utf8_lossy().parse().unwrap());
            }
            b"BlackElo" => self.black_elo = Some(value.decode_utf8_lossy().parse().unwrap()),
            b"Termination" if value.as_bytes() != b"Normal" => self.invalid = true,
            _ => {}
        }
    }

    fn end_headers(&mut self) -> pgn_reader::Skip {
        pgn_reader::Skip(self.invalid)
    }

    fn san(&mut self, san_plus: pgn_reader::SanPlus) {
        let the_move = san_plus.san.to_move(&self.chess).unwrap();
        let uci = the_move.to_uci(CastlingMode::Standard);
        let mut uci_move = UciMove(uci);

        uci_move.sanitize_promotion();
        if self.chess.turn() == Color::Black {
            uci_move.flip();
        }

        let id = uci_move_to_id(uci_move).unwrap();
        self.moves.push(id);

        assert!(self.chess.is_legal(&the_move));
        self.chess.play_unchecked(&the_move);
    }

    fn begin_variation(&mut self) -> pgn_reader::Skip {
        pgn_reader::Skip(true)
    }

    fn end_game(&mut self) -> Self::Result {
        let moves = mem::replace(&mut self.moves, Vec::new());
        if !self.invalid {
            Some(Game::new(
                self.white_elo.unwrap(),
                self.black_elo.unwrap(),
                moves,
            ))
        } else {
            None
        }
    }
}

fn main() -> anyhow::Result<()> {
    let output_file = File::create("./data/data.fitz.zst")?;
    let mut writer = zstd::Encoder::new(output_file, 0)?.auto_finish();

    let mut games_count = 0;
    for path in glob::glob("./data/*.pgn.zst")? {
        let file = File::open(path?)?;
        let reader = zstd::Decoder::new(file)?;
        let pgn_reader = pgn_reader::BufferedReader::new(reader);

        let mut visitor = Visitor::new();
        for result in pgn_reader.into_iter(&mut visitor).flatten().flatten() {
            bincode::serialize_into(&mut writer, &result)?;

            games_count += 1;
            if games_count % 100_000 == 0 {
                println!("Games processed: {}", games_count);
            }
        }
    }
    println!("Total games processed: {}", games_count);

    Ok(())
}
