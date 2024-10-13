fn alive_or_dead(arr_grid: &Vec<Vec<u8>>, row: usize, col: usize) -> (bool, bool) {
    //determines if currect cell should be set to alive or dead
    let mut alive_cell_num: u8 = 0;

    for r in row - 1..=row + 1 {
        for c in col - 1..=col + 1 {
            if r == row && c == col {
            } else if arr_grid[r][c] == 1 {
                alive_cell_num += 1;
            }
        }
    }

    let (state, birth): (bool, bool) = match alive_cell_num {
        0..2 => (false, false),
        2 => (true, false),
        3 => (true, true),
        _ => (false, false),
    };
    (state, birth)
}

fn next_frame(grid_arr: &Vec<Vec<u8>>, mut next_grid_arr: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut cell_state: bool;
    let mut birth: bool;

    for r in 1..grid_arr.len() - 1 {
        for c in 1..grid_arr.len() - 1 {
            (cell_state, birth) = alive_or_dead(&grid_arr, r, c);
            if grid_arr[r][c] == 1 {
                if cell_state {
                    next_grid_arr[r][c] = 1
                } else if !cell_state {
                    next_grid_arr[r][c] = 0
                }
            } else {
                if birth {
                    next_grid_arr[r][c] = 1
                }
            }
        }
    }
    next_grid_arr
}

fn game_of_life_sim(is_playing:bool){
    /*let mut is_paused: bool = false;

    //main array, 1 cell buffer on each side
    const N: usize = 20;
    let mut grid_arr: Vec<Vec<u8>> = vec![vec![0u8; N]; N];

    grid_arr[9][8] = 1;
    grid_arr[9][9] = 1;
    grid_arr[9][10] = 1;
    grid_arr[9][11] = 1;
    grid_arr[10][11] = 1;

    let mut next_grid_arr: Vec<Vec<u8>> = grid_arr.clone();

    println!("Frame 1:");
    for line in &grid_arr {
        println!("{:?}", line);
    }

    let mut app_end:bool = false;
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut population: u32 = 0;
        let mut generation: u32 = 0;
        //is_paused = true;
        //app_end = rx.try_recv().unwrap();
        while !app_end {
            while !is_paused {
                generation += 1;
                next_grid_arr = next_frame(&grid_arr, next_grid_arr);
                grid_arr = next_grid_arr.clone();

                println!("Frame:");
                for line in &grid_arr {
                    println!("{:?}", line);
                }


                thread::sleep(Duration::from_millis(500));
                app_end = rx.try_recv().unwrap_or(false);
                if app_end{
                    break;
                }
                //is_paused = app_end.clone();
            }

            
        }
    });
*/
}