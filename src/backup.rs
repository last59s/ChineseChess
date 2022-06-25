use crate::app::Game;
use ggez::graphics::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub red: HashMap<u8, (f32, f32)>,
    pub black: HashMap<u8, (f32, f32)>,
    pub state: bool,         // 状态值
    pub player: u16,         // 当前玩家
    pub player_color: Color, // 当前玩家颜色
    pub win: char,
}
impl Info {
    pub fn new(g: Game) -> Self {
        let mut red: HashMap<u8, (f32, f32)> = HashMap::new();
        let mut black: HashMap<u8, (f32, f32)> = HashMap::new();
        for i in 0..=15 {
            let r = g.red.get(&i).unwrap();
            let b = g.black.get(&i).unwrap();
            red.insert(i, (r.loc.x, r.loc.y));
            black.insert(i, (b.loc.x, b.loc.y));
        }
        Self {
            red,
            black,
            state: g.state,
            player: g.player,
            player_color: g.player_color,
            win: g.win,
        }
    }
    pub fn save(&self) -> std::io::Result<()> {
        let mut json_fils = File::create("./backup.json")?;
        let buf = serde_json::to_string(self).unwrap();
        json_fils.write(buf.as_bytes()).unwrap();
        Ok(())
    }
    pub fn read() -> Option<Self> {
        if let Ok(json_file) = File::open("./backup.json") {
            let backup: Info = serde_json::from_reader(json_file).unwrap();
            Some(backup)
        } else {
            None
        }
    }
}
