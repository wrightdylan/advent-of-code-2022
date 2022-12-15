const CAVE_WIDTH: usize = 1000;
const CAVE_HEIGHT: usize = 500;
const NUM_TILES: usize = CAVE_WIDTH * CAVE_HEIGHT;

/*
 * The distress signal leads you to a cave system behind a waterfall. You scan
 * a vertical slice of the cave above, and discover it it mostly air with some
 * structures of rock. Your scan traces the path of each rock structure, and 
 * reports the x, y coordinates that form the shape of the path. Each path app-
 * ears as a single line of text. Each point indicates the end of a straight
 * horizontal or vertical line to be drawn from the previous point.
 * 
 * Sand is produced one unit at a time, and the next unit is not produced until
 * the previous unit has come to rest. A unit of sand is large enough to fill
 * one tile in the scan. The unit of sand always falls down one step if possib-
 * le. If the tile below is blocked, the sand attempts to move one step down and
 * to the left. If that path is blocked then it will attempt to move down and to
 * the right. If all three paths are blocked, then the sand comes to rest. Sand
 * pours in from 500, 0.
 * 
 * Use your scan to simulate the falling sand. Find how many units of sand come
 * to rest before the rest falls into the abyss below.
 * 
 * Coding strategy
 * ===============
 * 
 * Might want to take some cues from from the dungeon crawler game I wrote when
 * learning rust. Use an enum for 'tile' types (air, rock, sand), buile the map
 * using a struct with a vector of tile types. One thing to note is that in the
 * input file there are multiple entries that are repeated many times; one case
 * is repeated 17 times. When parsing the input file it may be necessary to cr-
 * eate a HashSet of these rock paths? On the other hand it probably doesn't
 * matter that much if we 'redraw' the same line over again.
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
    println!("{} units of and come to rest.", units);
}
