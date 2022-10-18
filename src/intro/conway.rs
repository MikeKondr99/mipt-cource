#![forbid(unsafe_code)]

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    grid: Vec<T>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        Grid {
            grid: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    pub fn from_slice(grid: &[T], rows: usize, cols: usize) -> Self {
        if grid.len() != rows * cols {
            panic!("Неверный размер");
        }
        Grid {
            grid: grid.to_vec(),
            rows,
            cols,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cols, self.rows)
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.grid[col + row * self.cols]
    }

    pub fn set(&mut self, value: T, row: usize, col: usize) {
        self.grid[col + row * self.cols] = value;
    }

    pub fn neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (row, col) = (row as isize, col as isize);
        let res = vec![
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ];
        res.into_iter()
            .filter(|p| {
                p.0 >= 0 && p.1 >= 0 && p.0 < self.rows as isize && p.1 < self.cols as isize
            })
            .map(|x| (x.0 as usize, x.1 as usize))
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

impl Default for Cell {
    fn default() -> Self {
        Self::Dead
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Eq)]
pub struct GameOfLife {
    grid: Grid<Cell>,
}

impl GameOfLife {
    pub fn from_grid(grid: Grid<Cell>) -> Self {
        GameOfLife { grid }
    }

    pub fn get_grid(&self) -> &Grid<Cell> {
        &self.grid
    }

    pub fn step(&mut self) {
        let mut next = Grid::new(self.grid.rows, self.grid.cols);
        for x in 0..self.grid.rows {
            for y in 0..self.grid.cols {
                let count = self
                    .grid
                    .neighbours(x, y)
                    .map(|n| self.grid.get(n.0, n.1))
                    .filter(|n| **n == Cell::Alive)
                    .count();
                let cell = self.grid.get(x, y);
                next.set(
                    match (cell, count) {
                        (c, 2) => *c,
                        (_, 3) => Cell::Alive,
                        _ => Cell::Dead,
                    },
                    x,
                    y,
                )
            }
        }
        self.grid = next;
    }
}

#[cfg(test)]
mod tests {
    use super::{Cell, GameOfLife, Grid};

    fn get_grid(grid: Vec<Vec<u8>>) -> Grid<Cell> {
        let rows = grid.len();
        let cols = grid[0].len();
        let grid: Vec<Cell> = grid
            .into_iter()
            .flatten()
            .map(|value| if value == 0 { Cell::Dead } else { Cell::Alive })
            .collect();
        assert_eq!(grid.len(), rows * cols);
        Grid::from_slice(grid.as_slice(), rows, cols)
    }

    #[test]
    fn grid_neighbours() {
        assert_eq!(
            Grid::<i32>::new(3, 3)
                .neighbours(2, 2)
                .into_iter()
                .collect::<Vec<_>>(),
            vec![(1, 1), (1, 2), (2, 1)]
        );
        assert_eq!(
            Grid::<i32>::new(1, 1)
                .neighbours(0, 0)
                .into_iter()
                .collect::<Vec<_>>(),
            vec![]
        );
        assert_eq!(
            Grid::<i32>::new(3, 4)
                .neighbours(1, 1)
                .into_iter()
                .collect::<Vec<_>>(),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );
    }

    #[test]
    fn first_rule() {
        #[rustfmt::skip]
    let grid = get_grid(vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0]
    ]);
        let final_grid = get_grid(vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]);
        let mut game = GameOfLife::from_grid(grid);
        game.step();
        assert!(game.get_grid() == &final_grid);
    }

    #[test]
    fn second_rule() {
        #[rustfmt::skip]
    let grid = get_grid(vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1]
    ]);
        #[rustfmt::skip]
    let final_grid = get_grid(vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 0]
    ]);
        let mut game = GameOfLife::from_grid(grid);
        game.step();
        assert!(game.get_grid() == &final_grid);
    }

    #[test]
    fn third_rule() {
        #[rustfmt::skip]
    let grid = get_grid(vec![
        vec![0, 1, 0],
        vec![1, 1, 1],
        vec![0, 1, 0]
    ]);
        let final_grid = get_grid(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]);
        let mut game = GameOfLife::from_grid(grid);
        game.step();
        assert!(game.get_grid() == &final_grid);
    }

    #[test]
    fn fourth_rule() {
        #[rustfmt::skip]
    let grid = get_grid(vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![1, 0, 1]
    ]);
        #[rustfmt::skip]
    let final_grid = get_grid(vec![
        vec![0, 0, 0],
        vec![0, 1, 0],
        vec![0, 1, 0]
    ]);
        let mut game = GameOfLife::from_grid(grid);
        game.step();
        assert!(game.get_grid() == &final_grid);
    }

    #[test]
    fn glider() {
        let grid1 = get_grid(vec![
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 1, 1],
        ]);
        let grid2 = get_grid(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 0, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 1, 1],
        ]);
        let grid3 = get_grid(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![1, 0, 1, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 1, 1],
        ]);
        let grid4 = get_grid(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0],
            vec![0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 1, 1, 1],
            vec![0, 0, 0, 0, 1, 1],
        ]);
        let grid5 = get_grid(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 1],
            vec![0, 0, 0, 1, 0, 1],
        ]);
        let grid6 = get_grid(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 1, 0],
        ]);
        let grid7 = get_grid(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ]);

        let mut game = GameOfLife::from_grid(grid1.clone());
        assert!(game.get_grid() == &grid1);
        game.step();
        assert!(game.get_grid() == &grid2);
        game.step();
        assert!(game.get_grid() == &grid3);
        game.step();
        assert!(game.get_grid() == &grid4);
        game.step();
        assert!(game.get_grid() == &grid5);
        game.step();
        assert!(game.get_grid() == &grid6);
        game.step();
        assert!(game.get_grid() == &grid7);
        game.step();
        assert!(game.get_grid() == &grid7);
    }
}
