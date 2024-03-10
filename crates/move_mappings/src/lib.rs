use game_data::UciMove;
use quickphf::PhfMap;

pub static ID_MOVE_MAP: PhfMap<u16, UciMove> =
    include!(concat!(env!("OUT_DIR"), "/id_move_map.rs"));
pub static MOVE_ID_MAP: PhfMap<UciMove, u16> =
    include!(concat!(env!("OUT_DIR"), "/move_id_map.rs"));

pub fn uci_move_to_id(uci_move: UciMove) -> Option<u16> {
    MOVE_ID_MAP.get(&uci_move).copied()
}

pub fn id_to_uci_move(id: u16) -> Option<UciMove> {
    ID_MOVE_MAP.get(&id).cloned()
}
