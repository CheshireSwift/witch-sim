extern crate rltk;
// mod render;
// mod types;
use crate::types::core::*;
use crate::types::render::*;
use itertools::sorted;
use rltk::RGB;
use rltk::{Console, Rltk};
use specs::prelude::*;
use std::cmp::Ordering;

macro_rules! implement_unord {
  ($t:ty) => {
    impl Ord for $t {
      fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
      }
    }
    impl PartialOrd for $t {
      fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
      }
    }
  };
}

implement_unord!(AdditionalRenderables);

pub fn run(ctx: &mut Rltk, ecs: &World, background_color: RGB) {
  let positions = ecs.read_storage::<Pos>();
  let renderables = ecs.read_storage::<Renderable>();
  let additional_renderables = ecs.read_storage::<AdditionalRenderables>();
  let render_infos = (&renderables, &positions, additional_renderables.maybe()).join();
  // Draw renderables
  for (render, pos, maybe_additionals) in sorted(render_infos) {
    ctx.set(
      pos.x,
      pos.y,
      render.fg,
      render.bg.unwrap_or(background_color),
      render.glyph,
    );

    if let Some(additionals) = maybe_additionals {
      for additional in &additionals.renderables {
        ctx.set(
          pos.x + additional.offset.horizontal,
          pos.y + additional.offset.vertical,
          additional.renderable.fg,
          additional.renderable.bg.unwrap_or(background_color),
          additional.renderable.glyph,
        );
      }
    }
  }
}
