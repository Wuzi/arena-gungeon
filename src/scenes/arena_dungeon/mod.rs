use macroquad::prelude::{Rect, WHITE, clear_background, collections::storage, draw_rectangle};
use macroquad::color::SKYBLUE;

use crate::{
  resources::Resources,
  entities::player::Player,
};

use super::Scenes;

pub struct ArenaDungeonScreen {
  pub player: Player
}

impl ArenaDungeonScreen {
  pub async fn render(&self) -> Option<Scenes> {
    clear_background(SKYBLUE);

    let resources = storage::get_mut::<Resources>();
    let w = resources.tiled_map.raw_tiled_map.tilewidth * resources.tiled_map.raw_tiled_map.width;
    let h = resources.tiled_map.raw_tiled_map.tileheight * resources.tiled_map.raw_tiled_map.height;
    
    resources.tiled_map.draw_tiles("ground", Rect::new(0.0, 0.0, w as f32 * 2., h as f32 * 2.), None);
    resources.tiled_map.draw_tiles("wall", Rect::new(0.0, 0.0, w as f32 * 2., h as f32 * 2.), None);

    draw_rectangle(self.player.position.x, self.player.position.y, 40.0, 40.0, WHITE);

    None
  }
}
