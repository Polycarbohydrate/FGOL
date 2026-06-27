use std::collections::HashSet;

const LINE_MIN: i32 = 1;

pub fn display_frame(points: HashSet<(i32, i32)>, generation: u64, columns: u32, rows: u32) {
    // The following is example size of frame
    // ---------------------------------------------------
    // 35 x 101 grid

    // rows = y
    // colums = x

    // y limit is 35; x limit is 101
    // middle is 17 for y = 0; 50 is middle for x = 0

    // ---------------------------------------------------

    // rows and columns start from 1 not 0 where 1 is the leftmost column or bottom-most row
    // note in the code min is 0 and max is 100

    let y_rows = rows as i32;
    let x_columns = columns as i32;
    let y_middle = y_rows / 2;
    let x_middle = x_columns / 2;

    println!();
    println!("Generation: {}", generation);
    println!();

    let mut points_to_be_printed = vec![];

    // check if points are within grid area
    for (x, y) in points {
        if x + x_middle <= x_columns
            && x + x_middle >= LINE_MIN
            && y + y_middle <= y_rows
            && y + y_middle >= LINE_MIN
        {
            points_to_be_printed.push((x, y));
        }
    }

    let mut frame: String = "".to_string();
    // displaying stuff
    for row in (LINE_MIN..=y_rows).rev() {
        // 1 to 35 inclusive
        let mut row_print: String = "".to_string();
        'out: for column in LINE_MIN..=x_columns {
            // 1 to 101 inclusive
            for (x, y) in &points_to_be_printed {
                // finding proper location of point
                if y + y_middle == row && x + x_middle == column {
                    row_print.push('🦀');
                    continue 'out;
                }
            }
            row_print.push_str(". ");
        }
        row_print.push('\n');
        frame.push_str(&row_print);
    }
    println!("{frame}");
}
