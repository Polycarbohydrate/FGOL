use std::collections::HashSet;

const X_MIDDLE: i32 = 51;
const Y_MIDDLE: i32 = 18;
const LINE_MIN: i32 = 1;
const X_MAX: i32 = 101;
const Y_MAX: i32 = 35;

pub fn display_frame(points: HashSet<(i32, i32)>, generation: u64) {
    // 35 x 101 grid

    // rows = y
    // colums = x

    // y limit is 35; x limit is 101
    // middle is 18 for y = 0; 51 is middle for x = 0

    // rows and columns start from 1 not 0 where 1 is the leftmost column or bottom-most row
    // note in the code min is 0 and max is 100

    println!();
    println!("Generation: {}", generation);
    println!();

    let mut points_to_be_printed = vec![];

    // check if points are within grid area 35 x 101
    for (x, y) in points {
        if x + X_MIDDLE <= X_MAX
            && x + X_MIDDLE >= LINE_MIN
            && y + Y_MIDDLE <= Y_MAX
            && y + Y_MIDDLE >= LINE_MIN
        {
            // negation is becuase something with my logic is inverting the y-values... this is the lazy fix lol
            points_to_be_printed.push((x, -y));
        }
    }

    // displaying stuff
    for row in LINE_MIN..=Y_MAX {
        // 1 to 35 inclusive
        let mut row_print: String = "".to_string();
        'out: for column in LINE_MIN..=X_MAX {
            // 1 to 101 inclusive
            for (x, y) in &points_to_be_printed {
                // finding proper location of point
                if y + Y_MIDDLE == row && x + X_MIDDLE == column {
                    row_print.push_str("# ");
                    continue 'out;
                }
            }
            row_print.push_str(". ");
        }
        println!("{row_print}");
    }
}
