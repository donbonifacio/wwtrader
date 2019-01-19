use models::actors::{bandid, mountain, player, water};
use models::coordinate::Coordinate;
use models::entity::{Entity, EntityType};
use models::world::World;

pub fn print(world: &World) -> String {
    let mut lines: Vec<String> = vec![];
    for y in 0..world.right_edge.y as i32 {
        let mut line: Vec<String> = vec![];
        for x in 0..world.right_edge.x as i32 {
            line.push(coord_to_str(&world, x as f32, y as f32));
        }
        lines.push(line.join(""));
    }

    lines.join("\n")
}

fn coord_to_str(world: &World, x: f32, y: f32) -> String {
    match world.on_coord(Coordinate::new(x, y)) {
        Some(entity) => match entity.entity_type {
            EntityType::Player(n) => n.to_string(),
            EntityType::Enemy(c) => c.to_string(),
            EntityType::Obstacle(c) => c.to_string(),
            EntityType::Hole(c) => c.to_string(),
        },
        None => " ".to_string(),
    }
}

pub fn load(raw: &str) -> World {
    let lines: Vec<&str> = raw.split('\n').collect();

    let edge = Coordinate::new(lines[0].len() as f32, lines.len() as f32);
    let mut world = World::create(edge.translate(-1.0, -1.0));

    lines
        .iter()
        .enumerate()
        .for_each(|(y, line)| load_line(&mut world, y as f32, &line));

    world
}

fn load_line(world: &mut World, y: f32, raw: &str) {
    raw.chars().enumerate().for_each(|(x, c)| {
        let coord = Coordinate::new(x as f32, y);
        let entity: Option<Entity> = match c {
            '1' => Some(player::create_at(1, coord)),
            '2' => Some(player::create_at(2, coord)),
            'B' => Some(bandid::create_at(coord)),
            '#' => Some(mountain::create_at(coord)),
            '~' => Some(water::create_at(coord)),
            ' ' => None,
            _ => panic!("Don't know how to handle `{}`", c),
        };

        if let Some(entity) = entity {
            world.register(entity);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::World;

    #[test]
    fn print_empty_world() {
        let world = World::new();
        let result = print(&world);

        assert_eq!(
            result,
            ["        ", "        ", "        ", "        "].join("\n")
        );
    }

    #[test]
    fn print_world_with_actors() {
        let mut world = World::new();
        world.register(player::create_at(1, Coordinate::new(1.0, 1.0)));
        world.register(player::create_at(2, Coordinate::new(2.0, 1.0)));
        world.register(mountain::create_at(Coordinate::new(4.0, 1.0)));
        world.register(mountain::create_at(Coordinate::new(5.0, 1.0)));
        world.register(water::create_at(Coordinate::new(4.0, 2.0)));
        world.register(water::create_at(Coordinate::new(5.0, 2.0)));
        world.register(bandid::create_at(Coordinate::new(5.0, 3.0)));

        let result = print(&world);

        assert_eq!(
            result,
            ["        ", " 12 ##  ", "    ~~  ", "     B  "].join("\n")
        );
    }

    #[test]
    fn load_empty_world() {
        let world = load(&["        ", "        "].join("\n"));

        assert!(world.right_edge.is_at_x(7.0));
        assert!(world.right_edge.is_at_y(1.0));
        assert_eq!(world.has_actions(), false);
    }

    #[test]
    fn load_world_with_actors() {
        let world = load(&["12      ", "     B#~"].join("\n"));

        assert!(world.right_edge.is_at_x(7.0));
        assert!(world.right_edge.is_at_y(1.0));
        assert_eq!(world.has_actions(), false);

        let player: Option<&Entity> = world.on_coord(Coordinate::new(0.0, 0.0));
        assert!(player.is_some());
        if let Some(entity) = player {
            assert_eq!(entity.entity_type, EntityType::Player(1));
        }

        let player2: Option<&Entity> = world.on_coord(Coordinate::new(1.0, 0.0));
        assert!(player2.is_some());
        if let Some(entity) = player2 {
            assert_eq!(entity.entity_type, EntityType::Player(2));
        }

        let bandid: Option<&Entity> = world.on_coord(Coordinate::new(5.0, 1.0));
        assert!(bandid.is_some());
        if let Some(entity) = bandid {
            assert_eq!(entity.entity_type, EntityType::Enemy('B'));
        }

        let mountain: Option<&Entity> = world.on_coord(Coordinate::new(6.0, 1.0));
        assert!(mountain.is_some());
        if let Some(entity) = mountain {
            assert_eq!(entity.entity_type, EntityType::Obstacle('#'));
        }

        let water: Option<&Entity> = world.on_coord(Coordinate::new(7.0, 1.0));
        assert!(water.is_some());
        if let Some(entity) = water {
            assert_eq!(entity.entity_type, EntityType::Hole('~'));
        }
    }
}
