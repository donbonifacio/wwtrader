extern crate ggez;

use ggez::conf;
use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
use ggez::graphics::{DrawMode, Point2};
use ggez::{Context, GameResult};
use std::env;
use std::path;

use engine::controllers::{PlayerController, PlayerInput};
use engine::models::direction::{DOWN, LEFT, RIGHT, UP};
use engine::models::{Entity, EntityType};

// First we make a structure to contain the game's state
struct MainState {
    text: graphics::Text,
    world: engine::models::World,
    first_player_id: i32,
    second_player_id: i32,
}

impl MainState {
    fn new(ctx: &mut Context) -> MainState {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48).unwrap();
        let text = graphics::Text::new(ctx, "Wild Wild Trader", &font).unwrap();

        let world_data = [
            "               ",
            " 1           2 ",
            "      B        ",
            "      ~~~~##   ",
            "       ~~~~~#  ",
            "   B        B  ",
            "   #           ",
            "  ###          ",
            "   #     B     ",
            "   B           ",
        ]
        .join("\n");

        MainState {
            text,
            world: engine::serializers::basic::load(&world_data),
            first_player_id: 1,
            second_player_id: 2,
        }
    }
}

const START_X: f32 = 10.0;
const START_Y: f32 = 90.0;
const ENTITY_SIZE: f32 = 50.0;

impl MainState {
    fn draw_entity(&self, ctx: &mut Context, entity: &Entity) -> GameResult<()> {
        let color = match entity.entity_type {
            EntityType::Player(1) => graphics::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            EntityType::Player(2) => graphics::Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            EntityType::Player(_) => graphics::Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
            EntityType::Enemy(_) => graphics::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            EntityType::Obstacle(_) => graphics::Color {
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0,
            },
            EntityType::Hole(_) => graphics::Color {
                r: 0.2,
                g: 0.2,
                b: 0.8,
                a: 1.0,
            },
        };

        let x = START_X + entity.coord.x * ENTITY_SIZE + ENTITY_SIZE / 2.0;
        let y = START_Y + entity.coord.y * ENTITY_SIZE + ENTITY_SIZE / 2.0;

        let mesh = graphics::MeshBuilder::new()
            //.rectangle(graphics::DrawMode::Fill, graphics::Point2::new(100.0, 100.0), 100.0, 100.0, graphics::WHITE)
            .circle(DrawMode::Fill, Point2::new(x, y), ENTITY_SIZE / 2.0, 1.0)
            .build(ctx)?;

        graphics::set_color(ctx, color)?;
        graphics::draw(ctx, &mesh, graphics::Point2::new(0.0, 0.0), 0.0)?;

        Ok(())
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        match engine::game::runner::run(&mut self.world) {
            Ok(_) => Ok(()),
            Err(engine_error) => {
                eprintln!("Turn processing failed: {}", engine_error);
                Ok(())
            }
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_color(ctx, graphics::WHITE)?;

        // Drawables are drawn from their top-left corner.
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;

        for entity in self.world.entities.values() {
            self.draw_entity(ctx, &entity)?;
        }

        graphics::set_color(ctx, graphics::WHITE)?;

        // Because the border is one coordinate ahead
        let right_edge = self.world.right_edge.translate(1.0, 1.0);
        let board = graphics::MeshBuilder::new()
            .line(
                &[
                    Point2::new(START_X, START_Y),
                    Point2::new(START_X + right_edge.x * ENTITY_SIZE, START_Y),
                    Point2::new(
                        START_X + right_edge.x * ENTITY_SIZE,
                        START_Y + right_edge.y * ENTITY_SIZE,
                    ),
                    Point2::new(START_X, START_Y + right_edge.y * ENTITY_SIZE),
                    Point2::new(START_X, START_Y),
                ],
                4.0,
            )
            .build(ctx)?;

        graphics::draw(ctx, &board, graphics::Point2::new(0.0, 0.0), 0.0)?;

        graphics::present(ctx);

        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        let controller1 = PlayerController::new(self.first_player_id);
        let controller2 = PlayerController::new(self.second_player_id);
        match keycode {
            Keycode::Up => controller1.run(&mut self.world, PlayerInput::new(UP)),
            Keycode::Left => controller1.run(&mut self.world, PlayerInput::new(LEFT)),
            Keycode::Right => controller1.run(&mut self.world, PlayerInput::new(RIGHT)),
            Keycode::Down => controller1.run(&mut self.world, PlayerInput::new(DOWN)),
            Keycode::W => controller2.run(&mut self.world, PlayerInput::new(UP)),
            Keycode::A => controller2.run(&mut self.world, PlayerInput::new(LEFT)),
            Keycode::D => controller2.run(&mut self.world, PlayerInput::new(RIGHT)),
            Keycode::S => controller2.run(&mut self.world, PlayerInput::new(DOWN)),
            _ => (),
        }
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx);
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
