# Game of Life implementation
use math for rnd;
use process for sleep;

let WIDTH = 30;
let HEIGHT = 15;

# Create initial random grid
let grid = [];

let y = 0;
while y < HEIGHT {
    let row = [];

    let x = 0;
    while x < WIDTH {
        row.push(rnd(0, 2));  # Random 0 or 1
        x += 1;
    }

    grid.push(row);
    y += 1;
}

fn clear() {
    # ANSI escape sequence to clear screen
    print("\x{1B}[H\x{1B}[J");
}

fn draw() {
    # Render grid to terminal
    let y = 0;
    while y < HEIGHT {

        let row = grid.get(y);

        let x = 0;
        while x < WIDTH {

            if row.get(x) == 1 {
                print("█");  # Alive cell
            } else {
                print(" ");  # Dead cell
            }

            x += 1;
        }

        println("");  # New line after each row
        y += 1;
    }
}

fn count_neighbors(y, x) {
    # Count alive neighbors around cell (y, x)
    let count = 0;

    let dy = -1;
    while dy <= 1 {

        let dx = -1;
        while dx <= 1 {

            # Skip the cell itself
            if !(dy == 0 && dx == 0) {

                let ny = y + dy;
                let nx = x + dx;

                # Check bounds
                if ny >= 0 && ny < HEIGHT && nx >= 0 && nx < WIDTH {
                    let row = grid.get(ny);
                    count += row.get(nx);
                }
            }

            dx += 1;
        }

        dy += 1;
    }

    return count;
}

fn step() {

    # Create new empty grid
    let new_grid = [];

    let y = 0;
    while y < HEIGHT {
        let new_row = [];

        let x = 0;
        while x < WIDTH {
            new_row.push(0);  # Initialize as dead
            x += 1;
        }

        new_grid.push(new_row);
        y += 1;
    }

    # Apply Game of Life rules
    y = 0;
    while y < HEIGHT {

        let row = grid.get(y);
        let new_row = new_grid.get(y);

        let x = 0;
        while x < WIDTH {

            let neighbors = count_neighbors(y, x);

            if row.get(x) == 1 {
                # Alive cell survives with 2 or 3 neighbors
                if neighbors == 2 || neighbors == 3 {
                    new_row.set(x, 1);
                }
            } else {
                # Dead cell becomes alive with exactly 3 neighbors
                if neighbors == 3 {
                    new_row.set(x, 1);
                }
            }

            x += 1;
        }

        y += 1;
    }

    # Replace old grid with new one
    grid = new_grid;
}

# Main loop
while true {
    clear();
    draw();
    step();
    sleep(500);  # Delay in milliseconds
}
