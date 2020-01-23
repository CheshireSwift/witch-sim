extern crate rltk;
mod render;
mod types;
use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use std::cmp::{max, min};
use types::core::*;
use types::render::*;

// State gets a new World entry for Specs, an RNG, and a timer
struct State {
  ecs: World,
  time: f32,
  // rng: rltk::RandomNumberGenerator,
}

static MAX_X: i32 = 127;
static MAX_Y: i32 = 71;

impl GameState for State {
  fn tick(&mut self, ctx: &mut Rltk) {
    ctx.cls();

    render::run(ctx, &self.ecs, RGB::named(rltk::DARKGREEN));

    // Readable data stores
    let mut positions = self.ecs.write_storage::<Pos>();
    let mut players = self.ecs.write_storage::<Player>();

    // Player movement
    for (_player, pos) in (&mut players, &mut positions).join() {
      match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
          VirtualKeyCode::Numpad7 => {
            pos.x = max(pos.x - 1, 0);
            pos.y = max(pos.y - 1, 0);
          }
          VirtualKeyCode::Numpad9 => {
            pos.x = min(pos.x + 1, MAX_X);
            pos.y = max(pos.y - 1, 0);
          }
          VirtualKeyCode::Numpad1 => {
            pos.x = max(pos.x - 1, 0);
            pos.y = min(pos.y + 1, MAX_Y);
          }
          VirtualKeyCode::Numpad3 => {
            pos.x = min(pos.x + 1, MAX_X);
            pos.y = min(pos.y + 1, MAX_Y);
          }
          VirtualKeyCode::Left | VirtualKeyCode::A | VirtualKeyCode::Numpad4 => {
            pos.x = max(pos.x - 1, 0);
          }
          VirtualKeyCode::Right | VirtualKeyCode::D | VirtualKeyCode::Numpad6 => {
            pos.x = min(pos.x + 1, MAX_X);
          }
          VirtualKeyCode::Up | VirtualKeyCode::W | VirtualKeyCode::Numpad8 => {
            pos.y = max(pos.y - 1, 0);
          }
          VirtualKeyCode::Down | VirtualKeyCode::S | VirtualKeyCode::Numpad2 => {
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

    #[cfg(debug_assertions)]
    ctx.print_color(
      1,
      1,
      RGB::named(rltk::YELLOW),
      RGB::named(rltk::BLACK),
      &format!("FPS: {}", ctx.fps),
    );
    #[cfg(debug_assertions)]
    ctx.print_color(
      1,
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
    .with(Pos {
      x: MAX_X / 2,
      y: MAX_Y / 2,
    })
    .with(Renderable {
      glyph: rltk::to_cp437('☺'),
      fg: RGB::named(rltk::ANTIQUEWHITE),
      layer: layer::FORE,
      ..Default::default()
    })
    .with(AdditionalRenderables {
      renderables: vec![AdditionalRenderable::new(
        Renderable {
          glyph: rltk::to_cp437('▲'),
          fg: RGB::named(rltk::DARKMAGENTA),
          ..Default::default()
        },
        0,
        -1,
      )],
    })
    .with(Player {})
    .build();

  for x in 0..=MAX_X {
    for y in 0..=MAX_Y {
      let seed = ((((x * x + y) as f32).sin() + 1.0) * 1000.0) as i32;
      gs.ecs
        .create_entity()
        .with(Pos { x: x, y: y })
        .with(Renderable {
          glyph: rltk::to_cp437(match seed % 4 {
            0 => '.',
            1 => ',',
            2 => '\'',
            3 => '`',
            _ => '?',
          }),
          fg: if seed % 2 > 0 {
            RGB::named(rltk::GREEN)
          } else {
            RGB::named(rltk::GREENYELLOW)
          },
          layer: layer::BACK,
          ..Default::default()
        })
        .build();
    }
  }

  let context = Rltk::init_simple8x8(MAX_X + 1, MAX_Y + 1, "Witch Sim", "resources");
  rltk::main_loop(context, gs);
}
