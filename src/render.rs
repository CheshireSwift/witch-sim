extern crate rltk;
// mod render;
// mod types;
use crate::types::core::*;
use crate::types::render::*;
use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn run(
  &ctx: &Rltk,
  &positions: &ReadStorage<Pos>,
  &renderables: &ReadStorage<Renderable>,
  &additional_renderables: &ReadStorage<AdditionalRenderables>,
) {
  // Draw renderables
  for (pos, render, maybe_additionals) in
    (positions, renderables, additional_renderables.maybe()).join()
  {
    ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    if let Some(additionals) = maybe_additionals {
      for additional in &additionals.renderables {
        ctx.set(
          pos.x + additional.offset.horizontal,
          pos.y + additional.offset.vertical,
          additional.renderable.fg,
          additional.renderable.bg,
          additional.renderable.glyph,
        );
      }
    }
  }
}
