use std::collections::HashSet;

/* 
 * The Elves discover a patch of trees planted in a grid, and plan to build a
 * tree house. The trees are mapped according to height where 0 is the shortest
 * and 9 is the tallest. A tree is only visible on the grid from shorter trees.
 * Thankfully, trees are scanned in a horizontal and vertical direction only,
 * so no ray calculations will be necessary. Adjacent trees of the same height
 * are not visible from each other, because logic. The tree house is to be bui-
 * lt on a tree that is not visible from the outside.
 * 
 * For part 1, calculate the sum total of trees that are visible from outside
 * the grid. Overhead can be reduced as all trees at the edge are already visi-
 * ble by default, so only the interior trees need to be analysed.
 * 
 * Coding strategy
 * ===============
 * 
 * The input data can be loaded directly into a vector of vectors. Scanning can 
 * be done via indexing. The outer edges will not be scanned as they are alrea-
 * dy known to be visible, however scanning will be done from the edges. Some
 * trees will inevitably be counted twice as a result, so a HashSet can be used
 * to eliminate duplicates.
 */

#[derive(Debug)]
struct Grid{
    pos: Vec<Vec<u8>>,
    visible: HashSet<(usize, usize)>
}

fn load_data(data: &str) -> Grid {
    let pos = data
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c as u8)
                .filter(|b| b >= &48 && b <= &57)
                .map(|b| b - ('0' as u8))
                .collect()
        })
        .collect();
    Grid {
        pos,
        visible: HashSet::new(),
    }
}

// Location is loc(row, column)
fn visible_from_outside(mut grid: Grid) -> Grid {
    // Any size map of (height, width)
    let map = (grid.pos.len(), grid.pos[0].len());
    // Scan horizontal
    for e in 1..map.1 - 1 {
        peek_south(&mut grid, (0, e), map.0);
        peek_north(&mut grid, (map.0 - 1, e));
    }
    // Scan vertical
    for s in 1..map.1 - 1 {
        peek_east(&mut grid, (s, 0), map.1);
        peek_west(&mut grid, (s, map.1 - 1))
    }
    // Include perimeter
    for i in 0..map.0 {
        grid.visible.insert((i, 0));
        grid.visible.insert((i, map.1 - 1));
    }
    for i in 0..map.1 {
        grid.visible.insert((0, i));
        grid.visible.insert((map.0 - 1, i));
    }
    grid
}

fn peek_north(grid: &mut Grid, loc: (usize, usize)) {
    let mut height = grid.pos[loc.0][loc.1];
    for row in (1..loc.0-1).rev() {
        if height < grid.pos[row][loc.1] {
            height = grid.pos[row][loc.1];
            grid.visible.insert((row, loc.1));
        }
    }
}

fn peek_south(grid: &mut Grid, loc: (usize, usize), map_h: usize) {
    let mut height = grid.pos[loc.0][loc.1];
    for row in loc.0 + 1..map_h-1 {
        if height < grid.pos[row][loc.1] {
            height = grid.pos[row][loc.1];
            grid.visible.insert((row, loc.1));
        }
    }
}

fn peek_east(grid: &mut Grid, loc: (usize, usize), map_w: usize) {
    let mut height = grid.pos[loc.0][loc.1];
    for col in loc.1 + 1..map_w-1 {
        if height < grid.pos[loc.0][col] {
            height = grid.pos[loc.0][col];
            grid.visible.insert((loc.0, col));
        }
    }
}

fn peek_west(grid: &mut Grid, loc: (usize, usize)) {
    let mut height = grid.pos[loc.0][loc.1];
    for col in (1..loc.1-1).rev() {
        if height < grid.pos[loc.0][col] {
            height = grid.pos[loc.0][col];
            grid.visible.insert((loc.0, col));
        }
    }
    
}

fn main() {
    let data = include_str!("../input.txt");
    let grid = load_data(data);

    let grid = visible_from_outside(grid);
    println!("Trees visible from outside = {}", grid.visible.len());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_ok() {
        let data = include_str!("../input.txt");
        let grid = load_data(data);
        assert!(grid.pos.len() == 99);
        assert!(grid.pos[0].len() == 99);
    }

    #[test]
    fn find_edge() {
        let data = "
        30373
        25512
        65332
        33549
        35390";
        let grid = load_data(data.into());
        let grid = visible_from_outside(grid);
        println!("{}", grid.visible.len());
        assert!(grid.visible.len() == 21);
    }
}
