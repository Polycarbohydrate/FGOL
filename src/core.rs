use std::collections::{HashMap, HashSet};
use std::thread::sleep;
use std::time::Duration;

pub fn algorithm(points: HashSet<(i32, i32)>, speed: u64) -> HashSet<(i32, i32)> {
    let mut cells: HashSet<(i32, i32)> = points;

    // test for dead cells
    let mut holding_dead_cells: HashSet<(i32, i32)> = HashSet::new();
    for cell in cells.clone() {
        if is_cell_dead(cell.0, cell.1, cells.clone()) {
            holding_dead_cells.insert((cell.0, cell.1));
        }
    }

    // once cells are verified to be alive, the program can generate the next generation
    let mut possible_new_cells: HashMap<(i32, i32), i32> = HashMap::new();
    for cell in cells.clone() {
        let dead_neighbors = find_valid_new_neighbours(cell.0, cell.1, cells.clone());
        for cell in dead_neighbors {
            *possible_new_cells.entry(cell).or_insert(0) += 1;
        }
    }

    // remove tested dead cells from previous testing
    for cell in holding_dead_cells {
        cells.remove(&cell);
    }

    // add new cells
    for (k, v) in possible_new_cells {
        if v == 3 {
            cells.insert(k);
        }
    }

    sleep(Duration::from_millis(speed));

    cells
}

fn is_cell_dead(x: i32, y: i32, cells: HashSet<(i32, i32)>) -> bool {
    let number_of_neighbouring_cells = check_num_neighbours(x, y, cells);
    !(2..4).contains(&number_of_neighbouring_cells)
}

fn check_num_neighbours(x: i32, y: i32, cells: HashSet<(i32, i32)>) -> i32 {
    let mut count = 0;
    if cells.contains(&(x + 1, y)) {
        count += 1;
    } // right side

    if cells.contains(&(x, y + 1)) {
        count += 1;
    } // top side

    if cells.contains(&(x + 1, y + 1)) {
        count += 1;
    } // top-right side

    if cells.contains(&(x + 1, y - 1)) {
        count += 1;
    } // bottom-right side

    if cells.contains(&(x - 1, y)) {
        count += 1;
    } // left side

    if cells.contains(&(x, y - 1)) {
        count += 1;
    } // bottom side

    if cells.contains(&(x - 1, y - 1)) {
        count += 1;
    } // bottom-left side

    if cells.contains(&(x - 1, y + 1)) {
        count += 1;
    } // top-left side
    count
}

fn find_valid_new_neighbours(x: i32, y: i32, cells: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut new_points: HashSet<(i32, i32)> = HashSet::new();
    if !cells.contains(&(x + 1, y)) {
        new_points.insert((x + 1, y));
    } // right side

    if !cells.contains(&(x, y + 1)) {
        new_points.insert((x, y + 1));
    } // top side

    if !cells.contains(&(x + 1, y + 1)) {
        new_points.insert((x + 1, y + 1));
    } // top-right side

    if !cells.contains(&(x + 1, y - 1)) {
        new_points.insert((x + 1, y - 1));
    } // bottom-right side

    if !cells.contains(&(x - 1, y)) {
        new_points.insert((x - 1, y));
    } // left side

    if !cells.contains(&(x, y - 1)) {
        new_points.insert((x, y - 1));
    } // bottom side

    if !cells.contains(&(x - 1, y - 1)) {
        new_points.insert((x - 1, y - 1));
    } // bottom-left side

    if !cells.contains(&(x - 1, y + 1)) {
        new_points.insert((x - 1, y + 1));
    } // top-left side

    new_points
}
