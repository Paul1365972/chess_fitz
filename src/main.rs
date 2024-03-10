mod chess;
mod history;

use crate::history::PositionHistory;
use shakmaty::Chess;
use std::fs::File;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let file = File::open("./data/data.fitz.zst")?;
    let mut reader = zstd::Decoder::new(file)?;

    let mut games = 0;
    while let Some(game) = chess::next_game(&mut reader)? {
        let mut chess = Chess::new();
        let mut history = PositionHistory::new();

        for move_id in game.moves {
            history.update(&chess);
            chess::next_turn(&mut chess, move_id);
            // Do training here
            // Input: History Tensor, Output: Move Id Vector
        }

        games += 1;
        if games % 100_000 == 0 {
            println!("Games processed: {}", games);
        }
    }

    Ok(())
}
