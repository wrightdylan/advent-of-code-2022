use std::collections::HashSet;

/*
 * Your distress signal leads you to a network of underground tunnels. You have
 * deployable sensors that were built to locate lost Elves. The sensors attach
 * themselves to a hard surface and begin monitoring for the nearest signal so-
 * urce beacon. Sensors and beacons always exist at integer coordinates. Each
 * sensor knows its position and can determine the position of a single beacon
 * precisely, however they can only lock onto the one beacon closest to the se-
 * nsor as measure by the Manhattan distance (there is never a tie where two
 * beacons are the same distance to a sensor).
 * 
 * Since the sensors can only lock on to beacons closer to them, there may be
 * beacons that are not detected at all due to sensor saturation. None of the
 * detected beacons seem to be producing the distress signal, so you'll need to
 * work out where the distress beacon is by working out where it isn't. For now,
 * keep things simple by counting the positions where a beacon cannot possibly
 * be along just a single row.
 * 
 * Consult the report from the sensors you just deployed. In the row where
 * y=2_000_000, how many positions cannot contain a beacon?
 * 
 * Where there is a beacon, there is bacon.
 * 
 * Coding strategy
 * ===============
 * 
 * Sensors appear to detect in a diamond pattern around them (this is the Manh-
 * attan distance). As there will be some overlap between sensors, we can use a
 * HashSet for the radius fill between a sensor and the beacon it detects. Like
 * in the previous exercise, I could use a coverage map showing sensors and be-
 * acons. The problem is that we don't know the size the map needs to be befor-
 * ehand.
 * 
 * The Manhattan distance between the sensor and beacon is simply the horizontal
 * separation plus ths vertical separation. Since only one line is needed, it
 * probably isn't that necessary to map the full coverage of every sensor and
 * instead just map coverage leading down to and along line 2_000_000. However
 * the full map may be required for part 2...
 */

// Type alias vs struct
type Point = (isize, isize);

struct Sensor {
    position: Point,
    beacon:   Point,
    taxicab:  isize
}

impl Sensor {
    fn new(position @ (p_x, p_y): Point, beacon @ (b_x, b_y): Point) -> Self {
        let taxicab = p_x.abs_diff(b_x) + p_y.abs_diff(b_y);
        let taxicab = taxicab as isize;
        Self {
            position,
            beacon,
            taxicab,
        }
    }
}

// I don't know why I keep getting InvalidDigit with this
/*fn parse_input(data: &str) {
    data.trim()
        .lines()
        .map(|line| {
            let coords: Vec<isize> = line.split_ascii_whitespace()
                .map(|c| c.parse::<isize>().unwrap())
                .collect();
            let sensor = (coords[0], coords[1]);
            let beacon = (coords[2], coords[3]);
            (sensor, beacon)
        })
        .collect();
}*/

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| {
            let line = line.strip_prefix("Sensor at ").unwrap();
            let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();

            let sensor = sensor.split_once(", ").unwrap();
            let sensor: (isize, isize) = (
                sensor.0.strip_prefix("x=").unwrap().parse().unwrap(),
                sensor.1.strip_prefix("y=").unwrap().parse().unwrap(),
            );

            let beacon = beacon.split_once(", ").unwrap();
            let beacon: (isize, isize) = (
                beacon.0.strip_prefix("x=").unwrap().parse().unwrap(),
                beacon.1.strip_prefix("y=").unwrap().parse().unwrap(),
            );

            Sensor::new(sensor, beacon)
        })
        .collect()
}

fn part_a(data: Vec<Sensor>, scan_line: isize) -> usize {
    let mut coverage = HashSet::new();
    let mut beacons = HashSet::new();
    for line in data {
        if line.beacon.1 == scan_line {
            beacons.insert(line.beacon);
        }

        if (line.position.1 <= scan_line && line.position.1 + line.taxicab >= scan_line) ||
        (line.position.1 >= scan_line && line.position.1 - line.taxicab <= scan_line) {
            let width = line.taxicab - (line.position.1 - scan_line).abs();
            for x in line.position.0 - width..=line.position.0 + width {
                coverage.insert((x, scan_line));
            }
        }
    }
    coverage.difference(&beacons).count()
}

fn main() {
    let data = include_str!("../input.txt");
    let all_data = parse_input(data);
    let positions = part_a(all_data, 2_000_000);
    println!("Part A: Possible positions that cannot contain a beacon: {}", positions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let data = include_str!("../test.txt");
        let all_data = parse_input(data);
        let positions = part_a(all_data, 10);
        assert!(positions == 26);
    }
}