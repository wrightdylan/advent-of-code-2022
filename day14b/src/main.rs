const CAVE_WIDTH: usize = 1000;
const CAVE_HEIGHT: usize = 500;
const NUM_TILES: usize = CAVE_WIDTH * CAVE_HEIGHT;

/*
 * You have misread the scan. There isn't a bottomless abyss below, there is a
 * floor that extends in all directions for infinity, positioned at 2 + the hi-
 * ghest point on the map (with the height being inverted). Now the sand will
 * continue flowing until it comes to rest at 500, 0.
 * 
 * Coding strategy
 * ===============
 * 
 * The second part is almost the same. Just add a floor, and a stop condition
 * when there is a sand tile at 500, 0.
 */

#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Air,
    Rock,
    Sand
}

fn map_idx(x: usize, y: usize) -> usize {
    (y * CAVE_WIDTH) + x
}

// I have learned to love structs
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize
}

// You will learn to love structs
struct Map {
    tiles: Vec<TileType>
}

impl Map {
    fn new() -> Self {
        Self { tiles: vec![TileType::Air; NUM_TILES] }
    }

    fn can_enter(&self, point: Point) -> bool {
        if self.tiles[map_idx(point.x, point.y)] == TileType::Air {
            true
        } else {
            false
        }
    }

    fn set(&mut self, point: Point, tile: TileType) {
        self.tiles[map_idx(point.x, point.y)] = tile;
    }

    fn is_full(&self) -> bool {
        if self.tiles[map_idx(500, 0)] == TileType::Sand {
            true
        } else {
            false
        }
    }
}

fn parse_input(data: &str) -> (Map, usize) {
    let mut cave = Map::new();
    let mut depth = 0;
    for line in data.lines() {
        let mut rocks = line.split(" -> ").map(|p| {
            let (x,y) = p.split_once(',').unwrap();
            Point {
                x: x.parse::<usize>().unwrap(),
                y: y.parse::<usize>().unwrap()
            }
        });
        let mut current = rocks.next().unwrap();
        for new in rocks {
            depth = depth.max(new.y);
            cave.set(current, TileType::Rock);
            while current != new {
                if current.x < new.x {
                    current.x += 1;
                } else if current.x > new.x {
                    current.x -= 1;
                }
                if current.y < new.y {
                    current.y += 1;
                } else if current.y > new.y {
                    current.y -= 1;
                }
                cave.set(current, TileType::Rock);
            }
        }
    }
    (cave, depth)
}

fn sand_simulation(mut cave: Map, depth: usize) -> usize {
    let mut units: usize = 0;
    'outer: loop {
        let mut sand = Point { x: 500, y: 0 };
        loop {
            if sand.y >= depth {
                break 'outer;
            }

            if cave.is_full() {
                break 'outer;
            }

            if cave.can_enter(Point { x: sand.x, y: sand.y + 1}) {
                sand.y += 1;
                continue;
            } else if cave.can_enter(Point { x: sand.x -1, y: sand.y + 1}) {
                sand.x -= 1;
                sand.y += 1;
                continue;
            } else if cave.can_enter(Point { x: sand.x + 1, y: sand.y + 1}) {
                sand.x += 1;
                sand.y += 1;
                continue;
            }

            // When the sand unit has hit something and can't move
            cave.set(sand, TileType::Sand);
            break;
        }
        units += 1;
    }
    units
}

fn main() {
    let data = include_str!("../input.txt");
    let (mut cave, depth) = parse_input(data);
    let units = sand_simulation(cave, depth);
    println!("Part 1: {} units of sand come to rest.", units);


    let (mut cave2, depth) = parse_input(data);
    // Add a rock floor
    for x in 0..=CAVE_WIDTH {
        cave2.set(Point {x: x, y: depth + 2}, TileType::Rock);
    }
    let units2 = sand_simulation(cave2, depth + 3);
    println!("Part 2: {} units of sand come to rest.", units2);
}
