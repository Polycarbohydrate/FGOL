use std::collections::HashSet;
use std::io::{Write, stdin, stdout};

pub fn start() -> (HashSet<(i32, i32)>, u64, u32, u32) {
    println!(
        "🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀"
    );
    println!(
        "🦀                                                                                          🦀"
    );
    println!(
        "🦀                               Welcome to Ferris's Game of Life!                          🦀"
    );
    println!(
        "🦀                                                                                          🦀"
    );
    println!(
        "🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀"
    );
    println!();
    println!("Input cells by using X and Y coordinates in the Cartesian coordinate system.");
    println!("The point (0,0) will be the center of the display.");
    println!("The dispay grid is limited so any coordinate too big will not be rendered.");
    println!();

    let mut points = input_coordinates();

    loop {
        println!("Would you like to initialize another cell? Y/n");
        println!("(3+ cells is recomended)");
        let mut response = String::new();
        stdin()
            .read_line(&mut response)
            .expect("Failed to read input.");
        if response.trim().to_lowercase() == "y" {
            let more = input_coordinates();
            points.extend(more);
        } else if response.trim().to_lowercase() == "n" {
            break;
        } else {
            println!();
            println!("Please enter either 'y' or 'n'.")
        }
    }

    // speed section
    let execution_speed: u64;
    println!();
    loop {
        println!(
            "Please enter the speed of the simulation (time between each generation) in milliseconds (ms)."
        );
        println!("Put 0 for the maximum speed (limited by cpu speed).");
        let mut speed = String::new();
        stdin()
            .read_line(&mut speed)
            .expect("Failed to read input.");
        let speed_checked = match speed.trim().parse::<u64>() {
            Ok(num) => num,
            Err(_) => {
                println!();
                println!("Type in a valid positive integer.");
                continue;
            }
        };

        execution_speed = speed_checked;
        break;
    }

    // grid size
    println!();
    let mut number_of_columns: u32;
    let number_of_rows: u32;
    'out: loop {
        println!(
            "Please type the number (1, 2, 3, or 4) of the appropriate grid size you would like the simulation to be displayed in: "
        );
        println!("1. Small (20 x 20)");
        println!("2. Medium (50 x 50)");
        println!("3. Large (100 x 100)");
        println!(
            "4. Custom (You may enter the grid size. Note that large grids may be unable to fit the display properly.)"
        );
        print!("Choice: ");
        let _ = stdout().flush();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let number = match input.trim().parse::<u32>() {
            Ok(num) => {
                if num == 0 {
                    println!();
                    println!("Please enter a positive integer!");
                    continue;
                }
                num
            }
            Err(_) => {
                println!();
                println!("Please enter a positive integer!");
                continue;
            }
        };

        if number == 1 {
            number_of_rows = 20;
            number_of_columns = 20;
            break;
        } else if number == 2 {
            number_of_rows = 50;
            number_of_columns = 50;
            break;
        } else if number == 3 {
            number_of_rows = 100;
            number_of_columns = 100;
            break;
        } else if number == 4 {
            println!();
            loop {
                print!("Enter the number of columns (x-axis): ");
                let _ = stdout().flush();
                let mut x = String::new();
                stdin().read_line(&mut x).expect("Failed to read input.");
                let number_x = match x.trim().parse::<u32>() {
                    Ok(num) => {
                        if num == 0 {
                            println!("Please enter a positive integer!");
                            continue;
                        } else {
                            num
                        }
                    }
                    Err(_) => {
                        println!("Please enter a positive integer!");
                        continue;
                    }
                };

                number_of_columns = number_x;

                print!("Enter the number of rows (y-axis): ");
                let _ = stdout().flush();
                let mut y = String::new();
                stdin().read_line(&mut y).expect("Failed to read input.");
                let number_y = match y.trim().parse::<u32>() {
                    Ok(num) => {
                        if num == 0 {
                            println!("Please enter a positive integer!");
                            continue;
                        }
                        num
                    }
                    Err(_) => {
                        println!("Please enter a positive integer!");
                        continue;
                    }
                };

                number_of_rows = number_y;

                break 'out;
            }
        } else {
            println!();
            println!("Please enter a valid response. Type the number 1, 2, 3, or 4.");
            continue;
        }
    }

    (points, execution_speed, number_of_columns, number_of_rows)
}

fn input_coordinates() -> HashSet<(i32, i32)> {
    let mut points: HashSet<(i32, i32)> = HashSet::new();

    let n: i32;
    loop {
        println!(
            "How many points would you like to add? You will have the opportunity to add more later: "
        );
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        println!();
        n = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer value (3, 87, 12, etc...)");
                continue;
            }
        };
        break;
    }

    let mut counter = 1; // counter used to ensure the point count is accurate even if someone fails to input an integer.

    loop {
        // x-coordinate
        print!(
            "Type in the x-value of point {} (must be an integer): ",
            counter
        );
        let _ = stdout().flush();
        let mut x = String::new();
        stdin().read_line(&mut x).expect("Failed to read input.");
        let x_value: i32 = match x.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Type in a valid integer. Please try again.");
                continue;
            }
        };

        // y-coordinate
        print!(
            "Type in the y-value of point {} (must be an integer): ",
            counter
        );
        let _ = stdout().flush();
        let mut y = String::new();
        stdin().read_line(&mut y).expect("Failed to read input.");
        let y_value: i32 = match y.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Type in a valid integer. Please try again.");
                continue;
            }
        };

        points.insert((x_value, y_value));
        if counter == n {
            break;
        }
        counter += 1;
        println!();
    }
    println!();
    points
}
