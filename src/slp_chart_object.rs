use chrono::prelude::*;

struct GameSettings {
    is_teams: bool,
    is_pal: bool,
    stage_id: i32,
    stage_name: String,
}

struct GameMetadata {
    start_at: DateTime<Local>,
    last_frame: usize,
}

pub struct SlpChartObject {
    match_id: String,
    settings: GameSettings,
    metadata: GameMetadata,
}
