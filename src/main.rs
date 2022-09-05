struct Grid {
    state: Vec<Vec<CellState>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            state: vec![vec![CellState::Dead; width]; height],
            width: width,
            height: height,
        }
    }

    fn neighbors(&self, point: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = (point.0 as isize, point.1 as isize);
        let mut points: Vec<(isize, isize)> = Vec::new();

        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }

                let neighbor = (x + x_offset, y + y_offset);

                if !self.in_bounds(neighbor) {
                    continue;
                }

                points.push(neighbor);
            }
        }

        let points: Vec<(usize, usize)> = points.iter()
            .map(|p| (p.0 as usize, p.1 as usize))
            .collect();

        points
    }

    fn in_bounds(&self, point: (isize, isize)) -> bool {
        let (x, y) = point;
        let (width, height) = (self.width as isize, self.height as isize);

        (0isize..width).contains(&x) && (0isize..height).contains(&y)
    }

    fn at(&self, point: (usize, usize)) -> CellState {
        let (x, y) = point;
        self.state[y][x]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum CellState {
    Dead,
    Alive,
}

struct Game {
    grid: Grid,
    generation: u32,
}

impl Game {
    fn new(width: usize, height: usize) -> Game {
        Game {
            grid: Grid::new(width, height),
            generation: 0,
        }
    }

    fn compute_next(&mut self) {
        let mut next_grid_state = self.grid.state.clone();

        for i in 0..(self.grid.height) {
            for j in 0..(self.grid.width) {
                let cell_state = self.grid.at((j, i));
                let alive_neighbor_count = self.grid.neighbors((j, i)).iter()
                    .filter(|p| self.grid.at(**p) == CellState::Alive)
                    .count();

                match cell_state {
                    CellState::Alive if alive_neighbor_count < 2 || alive_neighbor_count > 3 => {
                        next_grid_state[i][j] = CellState::Dead;
                    },
                    CellState::Dead if alive_neighbor_count == 3 => {
                        next_grid_state[i][j] = CellState::Alive;
                    },
                    _ => (),
                }
            }
        }

        self.grid.state = next_grid_state;
        self.generation += 1;
    }
}

fn symbol_for(cell: CellState) -> String {
    match cell {
        CellState::Alive => String::from("\u{25cf}"),
        CellState::Dead => String::from(" "),
    }
}

fn print(grid: &Grid) {
    for i in 0..(grid.height) {
        for j in 0..(grid.width) {
            match grid.state[i][j] {
                CellState::Alive =>
                    print!("{}", symbol_for(CellState::Alive)),
                CellState::Dead =>
                    print!("{}", symbol_for(CellState::Dead)),
            }
        }

        println!();
    }
}

fn clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn sleep(ms: u64) {
    use std::time::Duration;
    use std::thread;

    let duration = Duration::from_millis(ms);
    thread::sleep(duration);
}

fn main() {
    let width = 130;
    let height = 30;
    let mut game = Game::new(width, height);

    for i in 0..height {
        for j in 0..width {
            if (i > 12 || i < 3) && j > 40 {
                game.grid.state[i][j] = CellState::Alive;
            }
        }
    }

    loop {
        clear();
        print(&game.grid);
        println!("Generation #{}", game.generation);
        sleep(10);
        game.compute_next();
    }
}
