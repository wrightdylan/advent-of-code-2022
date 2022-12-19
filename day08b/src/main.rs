use std::collections::HashSet;

/* 
 * The Elves wantto build a tree house, and need to know the best spot to build
 * it where they can see a lot of trees. Viewing distance is,as before,determi-
 * ned by cardinal directions. They don't need to be on the tallest tree, and
 * they won't be able to see the tops of the tallest trees due to the large ea-
 * ves on their house. From the tree house, the Elves will be able to see trees
 * that are the same height or smaller. Their view is blocked by taller trees,
 * trees of equal height, and edges. The tree house can be built at the edge of
 * the grid,but one of its viewing distances will be zero.
 * 
 * A tree's scenic score is calculated by its view distances along each direct-
 * ion multiplied by each other. Viewing distances of 0 need to be avoided.
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

#[derive(Debug, Clone)]
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

fn best_scene(grid: &mut Grid) -> usize {
    let mut best_score = 0;
    for row in 0..grid.pos.len() {
        for col in 0..grid.pos[1].len() {
            let rating = get_rating(grid, row, col);
            if rating > best_score {
                best_score = rating;
            }
        }
    }

    best_score
}

// I was hoping to reuse the above functions somehow...
fn get_rating(grid: &mut Grid, row: usize, col: usize) -> usize {
    let dirs = [see_down, see_up, see_right, see_left];
    dirs.iter().map(|f| f(grid, row, col)).product()
}

fn see_up(grid: &mut Grid, row: usize, col: usize) -> usize {
    let mut score = 0;
    let val = grid.pos[row][col];
    for i in (0..row).rev() {
        score += 1;
        if val <= grid.pos[i][col] {
            break;
        }
    }
    score
}

fn see_down(grid: &mut Grid, row: usize, col: usize) -> usize {
    let map_height = grid.pos.len();
    let mut score = 0;
    let val = grid.pos[row][col];
    for i in row + 1..map_height {
        score += 1;
        if val <= grid.pos[i][col] {
            break;
        }
    }
    score
}

fn see_right(grid: &mut Grid, row: usize, col: usize) -> usize {
    let map_width = grid.pos[0].len();
    let mut score = 0;
    let val = grid.pos[row][col];
    for j in col + 1..map_width {
        score += 1;
        if val <= grid.pos[row][j] {
            break;
        }
    }
    score
}

fn see_left(grid: &mut Grid, row: usize, col: usize) -> usize {
    let mut score = 0;
    let val = grid.pos[row][col];
    for j in (0..col).rev() {
        score += 1;
        if val <= grid.pos[row][j] {
            break;
        }
    }
    score
}

fn main() {
    let data = include_str!("../input.txt");
    let grid = load_data(data);

    let mut grid = visible_from_outside(grid);
    println!("Trees visible from outside = {}", grid.visible.len());

    let scenic_score = best_scene(&mut grid);
    println!("The best scenic score is {}", scenic_score);
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
