use std::collections::HashSet;
use std::io::stdin;

pub fn start() -> (HashSet<(i32, i32)>, u64) {
    println!(
        "#############################################################################################"
    );
    println!(
        "#                                                                                           #"
    );
    println!(
        "#                               Welcome to Conway's Game of Life!                           #"
    );
    println!(
        "#                                                                                           #"
    );
    println!(
        "#############################################################################################"
    );
    println!();
    println!("Please enter the coordinates of the initial cells:");
    println!();

    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let mut point_count = 1;
    'out: loop {
        // x-coordinate
        println!(
            "Type in the x-value of point {} (must be an integer)",
            point_count
        );
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
        println!(
            "Type in the y-value of point {} (must be an integer)",
            point_count
        );
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
        println!();

        loop {
            println!("Would you like to initialize another cell? Y/n");
            println!("(3+ cells is recomended)");
            let mut response = String::new();
            stdin()
                .read_line(&mut response)
                .expect("Failed to read input.");
            if response.trim().to_lowercase() == "y" {
                point_count += 1;
                println!();
                break;
            } else if response.trim().to_lowercase() == "n" {
                break 'out;
            } else {
                println!();
                println!("Please enter either 'y' or 'n'.")
            }
        }
    }

    let execution_speed: u64;
    // speed
    loop {
        println!();
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
                println!("Type in a valid integer.");
                continue;
            }
        };

        execution_speed = speed_checked;
        break;
    }

    (points, execution_speed)
}
