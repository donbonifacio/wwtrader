use models::actors::{bandid, mountain, player, water};
use models::coordinate::Coordinate;
use models::entity::{Entity, EntityType};
use models::world::World;

pub fn print(world: &World) -> String {
    let mut lines: Vec<String> = vec![];
    for y in 0..world.size_y {
        let mut line: Vec<String> = vec![];
        for x in 0..world.size_x {
            line.push(coord_to_str(&world, x, y));
        }
        lines.push(line.join(""));
    }

    lines.join("\n")
}

fn coord_to_str(world: &World, x: usize, y: usize) -> String {
    match world.on_coord(Coordinate::new(x as i8, y as i8)) {
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

    let mut world = World::create(lines[0].len(), lines.len());

    lines
        .iter()
        .enumerate()
        .for_each(|(y, line)| load_line(&mut world, y, &line));

    world
}

fn load_line(world: &mut World, y: usize, raw: &str) {
    raw.chars().enumerate().for_each(|(x, c)| {
        let coord = Coordinate::new(x as i8, y as i8);
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
        world.register(player::create_at(1, Coordinate::new(1, 1)));
        world.register(player::create_at(2, Coordinate::new(2, 1)));
        world.register(mountain::create_at(Coordinate::new(4, 1)));
        world.register(mountain::create_at(Coordinate::new(5, 1)));
        world.register(water::create_at(Coordinate::new(4, 2)));
        world.register(water::create_at(Coordinate::new(5, 2)));
        world.register(bandid::create_at(Coordinate::new(5, 3)));

        let result = print(&world);

        assert_eq!(
            result,
            ["        ", " 12 ##  ", "    ~~  ", "     B  "].join("\n")
        );
    }

    #[test]
    fn load_empty_world() {
        let world = load(&["        ", "        "].join("\n"));

        assert_eq!(world.size_x, 8);
        assert_eq!(world.size_y, 2);
        assert_eq!(world.has_actions(), false);
    }

    #[test]
    fn load_world_with_actors() {
        let world = load(&["12      ", "     B#~"].join("\n"));

        assert_eq!(world.size_x, 8);
        assert_eq!(world.size_y, 2);
        assert_eq!(world.has_actions(), false);

        let player: Option<&Entity> = world.on_coord(Coordinate::new(0, 0));
        assert!(player.is_some());
        if let Some(entity) = player {
            assert_eq!(entity.entity_type, EntityType::Player(1));
        }

        let player2: Option<&Entity> = world.on_coord(Coordinate::new(1, 0));
        assert!(player2.is_some());
        if let Some(entity) = player2 {
            assert_eq!(entity.entity_type, EntityType::Player(2));
        }

        let bandid: Option<&Entity> = world.on_coord(Coordinate::new(5, 1));
        assert!(bandid.is_some());
        if let Some(entity) = bandid {
            assert_eq!(entity.entity_type, EntityType::Enemy('B'));
        }

        let mountain: Option<&Entity> = world.on_coord(Coordinate::new(6, 1));
        assert!(mountain.is_some());
        if let Some(entity) = mountain {
            assert_eq!(entity.entity_type, EntityType::Obstacle('#'));
        }

        let water: Option<&Entity> = world.on_coord(Coordinate::new(7, 1));
        assert!(water.is_some());
        if let Some(entity) = water {
            assert_eq!(entity.entity_type, EntityType::Hole('~'));
        }
    }
}
