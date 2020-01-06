extern crate rltk;
mod render;
mod types;
use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};
use types::core::*;
use types::render::*;

// State gets a new World entry for Specs, an RNG, and a score counter
struct State {
  ecs: World,
  time: f32,
  // rng: rltk::RandomNumberGenerator,
}

static MAX_X: i32 = 79;
static MAX_Y: i32 = 49;

impl GameState for State {
  fn tick(&mut self, ctx: &mut Rltk) {
    ctx.cls();

    // Readable data stores
    let mut positions = self.ecs.write_storage::<Pos>();
    let renderables = self.ecs.read_storage::<Renderable>();
    let additional_renderables = self.ecs.read_storage::<AdditionalRenderables>();
    let mut players = self.ecs.write_storage::<Player>();

    // Player movement
    for (_player, pos) in (&mut players, &mut positions).join() {
      match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
          VirtualKeyCode::Left | VirtualKeyCode::A => {
            pos.x = max(pos.x - 1, 0);
          }
          VirtualKeyCode::Right | VirtualKeyCode::D => {
            pos.x = min(pos.x + 1, MAX_X);
          }
          VirtualKeyCode::Up | VirtualKeyCode::W => {
            pos.y = max(pos.y - 1, 0);
          }
          VirtualKeyCode::Down | VirtualKeyCode::S => {
            pos.y = min(pos.y + 1, MAX_Y);
          }
          VirtualKeyCode::Space | VirtualKeyCode::Return | VirtualKeyCode::NumpadEnter => {}
          _ => {}
        },
      }
    }

    self.time += ctx.frame_time_ms;
    // if self.time > 200.0 {
    //   self.time = 0.0;

    //   // Find the player
    //   // let mut player_x = 0;
    //   // for (_player, player_pos) in (&mut players, &mut positions).join() {
    //   //     player_x = player_pos.x;
    //   // }

    //   // move/handle NPCs
    //   // for (_baby, pos) in (&mut babies, &mut positions).join() {
    //   //     pos.y += 1;
    //   //     if pos.y > 48 {
    //   //         pos.y = 0;
    //   //         if player_x == pos.x {
    //   //             // We saved them
    //   //             self.saved += 1;
    //   //         } else {
    //   //             // Squish!
    //   //             self.squished += 1;
    //   //         }
    //   //         pos.x = self.rng.roll_dice(1, 79);
    //   //     }
    //   // }
    // }

    render::run(&ctx, &positions, &renderables, &additional_renderables);

    #[cfg(debug_assertions)]
    ctx.print_color(
      40,
      1,
      RGB::named(rltk::YELLOW),
      RGB::named(rltk::BLACK),
      &format!("FPS: {}", ctx.fps),
    );
    #[cfg(debug_assertions)]
    ctx.print_color(
      40,
      2,
      RGB::named(rltk::CYAN),
      RGB::named(rltk::BLACK),
      &format!("Frame Time: {} ms", ctx.frame_time_ms),
    );
  }
}

fn main() {
  let mut gs = State {
    ecs: World::new(),
    time: 0.0,
    // rng: rltk::RandomNumberGenerator::new(),
  };
  gs.ecs.register::<Pos>();
  gs.ecs.register::<Renderable>();
  gs.ecs.register::<AdditionalRenderables>();
  gs.ecs.register::<Solid>();
  gs.ecs.register::<Player>();

  gs.ecs
    .create_entity()
    .with(Pos { x: 40, y: 49 })
    .with(Renderable {
      glyph: rltk::to_cp437('☺'),
      fg: RGB::named(rltk::ANTIQUEWHITE),
      bg: RGB::named(rltk::BLACK),
    })
    .with(AdditionalRenderables {
      renderables: vec![AdditionalRenderable::new(
        Renderable {
          glyph: rltk::to_cp437('▲'),
          fg: RGB::named(rltk::ANTIQUEWHITE),
          bg: RGB::named(rltk::BLACK),
        },
        0,
        -1,
      )],
    })
    .with(Player {})
    .build();

  let context = Rltk::init_simple8x8(80, 50, "Witch Sim", "resources");
  rltk::main_loop(context, gs);
}
