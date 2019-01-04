
use models::actors::{player, bandid, mountain, water};
use models::world::World;
use models::entity::{EntityType};
use models::coordinate::Coordinate;

pub fn print(world: World) -> String {
    let mut lines: Vec<String> = vec![];
    for y in 0..world.size_y {
        let mut line: Vec<String> = vec![String::new(); world.size_x];
        for x in 0..world.size_x {
            line[x] = coord_to_str(&world, x, y);
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
            EntityType::Hole(c) => c.to_string()
        }
        None => " ".to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use models::World;

    #[test]
    fn empty_world() {
        let world = World::new();
        let result = print(world);

        assert_eq!(result, ["        ",
                            "        ",
                            "        ",
                            "        "].join("\n"));
    }

    #[test]
    fn world_with_actors() {
        let mut world = World::new();
        world.register(player::create_at(Coordinate::new(1, 1)));
        world.register(mountain::create_at(Coordinate::new(4, 1)));
        world.register(mountain::create_at(Coordinate::new(5, 1)));
        world.register(water::create_at(Coordinate::new(4, 2)));
        world.register(water::create_at(Coordinate::new(5, 2)));
        world.register(bandid::create_at(Coordinate::new(5, 3)));

        let result = print(world);

        assert_eq!(result, ["        ",
                            " 1  ##  ",
                            "    ~~  ",
                            "     B  "].join("\n"));
    }
}
