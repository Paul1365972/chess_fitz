use quickphf_codegen::ConstInstantiable;
use serde::{Deserialize, Serialize};
use shakmaty::{uci::Uci, Position, Rank, Role};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub white_elo: u16,
    pub black_elo: u16,
    pub moves: Vec<u16>,
}

impl Game {
    pub fn new(white_elo: u16, black_elo: u16, moves: Vec<u16>) -> Self {
        Self {
            white_elo,
            black_elo,
            moves,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UciMove(pub Uci);

impl UciMove {
    pub fn flip(&mut self) {
        match &mut self.0 {
            Uci::Normal { from, to, .. } => {
                *from = from.flip_vertical();
                *to = to.flip_vertical();
            }
            Uci::Put { to, .. } => {
                *to = to.flip_vertical();
            }
            Uci::Null => {}
        }
    }

    pub fn sanitize_promotion(&mut self) {
        if let Uci::Normal { promotion, .. } = &mut self.0 {
            if *promotion == Some(Role::Knight) {
                *promotion = None;
            }
        }
    }

    pub fn unsanitize_promotion<P: Position>(&mut self, position: &P) {
        if let Uci::Normal {
            from,
            to,
            promotion,
        } = &mut self.0
        {
            if position.our(Role::Pawn).contains(*from)
                && (to.rank() == Rank::First || to.rank() == Rank::Eighth)
                && *promotion == None
            {
                *promotion = Some(Role::Knight)
            }
        }
    }
}

impl ConstInstantiable for UciMove {
    fn fmt_const_new(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Uci::Normal {
            from,
            to,
            promotion,
        } = self.0
        {
            write!(
                f,
                "::game_data::UciMove(::shakmaty::uci::Uci::Normal {{ from: ::shakmaty::Square::{:?}, to: ::shakmaty::Square::{:?}, promotion: {} }})",
                from,
                to,
                promotion.map_or("None".to_owned(), |role| format!("Some(::shakmaty::Role::{:?})", role))
            )
        } else {
            unimplemented!()
        }
    }
}
