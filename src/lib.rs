extern crate prettytable;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use std::ops::{Index, IndexMut};
use self::Direction::{Down, Left, Right, Up};
use self::Error::*;

#[cfg(test)]
mod test;
// Possible directions for cycle
#[derive(Clone,Copy,PartialEq,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
// The structure for emulation a two-dimensional array
struct Matrix<T> {
    cols: usize,
    data: Vec<T>,
}

impl<T> Index<usize> for Matrix<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let i = index * self.cols;
        &self.data[i..i + self.cols]
    }
}

impl<T> IndexMut<usize> for Matrix<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = index * self.cols;
        &mut self.data[i..i + self.cols]
    }
}

impl<T> Matrix<T> {
    fn new(columns: usize) -> Matrix<T> {
        Matrix {
            cols: columns,
            data: Vec::new(),
        }
    }
    // Add a line to the matrix
    fn push(&mut self, mut vector: Vec<T>) {
        assert_eq!(self.cols, vector.len());
        self.data.append(&mut vector);
    }
    // The number of rows in the matrix
    fn rows(&self) -> usize {
        self.data.len() / self.cols
    }
    // The number of columns in the matrix
    fn cols(&self) -> usize {
        self.cols
    }
}

impl Matrix<Direction> {
    fn iter<'a>(&'a self, i: usize, j: usize) -> MatrixIter<'a> {
        MatrixIter {
            state: self,
            start_i: i,
            start_j: j,
            i: i,
            j: j,
            f: false,
        }
    }
}

struct MatrixIter<'a> {
    state: &'a Matrix<Direction>,
    start_i: usize,
    start_j: usize,
    i: usize,
    j: usize,
    f: bool,
}

impl<'a> MatrixIter<'a> {
    fn move_by(&mut self, d: Direction) {
        match d {
            Up => self.i += 1,
            Down => self.i -= 1,
            Left => self.j += 1,
            Right => self.j -= 1,
            Direction::None => unreachable!(),
        }
    }
}

impl<'a> Iterator for MatrixIter<'a> {
    type Item = (usize,usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.start_i && self.j == self.start_j {
            if self.f {
                return None;
            }
            self.f = true;
        }

        let buf = (self.i, self.j);
        let d = self.state[self.i][self.j];
        while self.state[self.i][self.j] == d || self.state[self.i][self.j] == Direction::None {
            self.move_by(d);
        }
        Some(buf)
    }
}

// Initialization errors
#[derive(Debug)]
pub enum Error {
    NumOfSupOrCust,
    NumOfRows,
    NumOfCols,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use std::error::Error;
        f.write_str(self.description())
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            NumOfSupOrCust => "invalid number of suppliers or customers",
            NumOfRows => "invalid number of rows in the matrix of costs",
            NumOfCols => "invalid number of columns in the matrix of costs",
        }
    }
}

// The main structure
pub struct Transportation {
    supply: Vec<u64>,
    demand: Vec<u64>,
    cost: Matrix<u64>,
    trans: Matrix<Option<u64>>,
}

impl Transportation {
    fn least_cost_method(&mut self) {
        let mut a = self.supply.to_vec();
        let mut b = self.demand.to_vec();
        let mut min;

        loop {
            min = None;

            for i in 0..self.cost.rows() {
                if a[i] == 0 {
                    continue;
                }
                for j in 0..self.cost.cols() {
                    if b[j] == 0 {
                        continue;
                    }
                    if let Some((_, _, v)) = min {
                        if v > self.cost[i][j] {
                            min = Some((i, j, self.cost[i][j]));
                        }
                    } else {
                        min = Some((i, j, self.cost[i][j]));
                    }
                }
            }

            if let Some((i, j, _)) = min {
                if a[i] >= b[j] {
                    self.trans[i][j] = Some(b[j]);
                    a[i] -= b[j];
                    b[j] = 0;
                } else {
                    self.trans[i][j] = Some(a[i]);
                    b[j] -= a[i];
                    a[i] = 0;
                }
            } else {
                return;
            }
        }
    }
    // Detect cycle via dfs and build a matrix of directions
    // If the direction of the trans_state[i][j] isn't None, the cycle is found
    fn cycle_detection(&self, i: usize, j: usize) -> Matrix<Direction> {
        let mut trans_state = Matrix::new(self.trans.cols());
        for _ in 0..self.trans.rows() {
            trans_state.push(vec![Direction::None;self.trans.cols()]);
        }

        fn _cycle_detection(x: &Matrix<Option<u64>>, x_state: &mut Matrix<Direction>, i: usize, j: usize) {
            if x_state[i][j] != Down {
                for k in (0..i).rev() {
                    if x[k][j] != None {
                        if x_state[k][j] == Direction::None {
                            x_state[k][j] = Up;
                            _cycle_detection(x, x_state, k, j);
                        } else {
                            return;
                        }
                        break;
                    }
                }
            }
            if x_state[i][j] != Up {
                for k in i + 1..x.rows() {
                    if x[k][j] != None {
                        if x_state[k][j] == Direction::None {
                            x_state[k][j] = Down;
                            _cycle_detection(x, x_state, k, j);
                        } else {
                            return;
                        }
                        break;
                    }
                }
            }
            if x_state[i][j] != Right {
                for k in (0..j).rev() {
                    if x[i][k] != None {
                        if x_state[i][k] == Direction::None {
                            x_state[i][k] = Left;
                            _cycle_detection(x, x_state, i, k);
                        } else {
                            return;
                        }
                        break;
                    }
                }
            }
            if x_state[i][j] != Left {
                for k in j + 1..x.cols() {
                    if x[i][k] != None {
                        if x_state[i][k] == Direction::None {
                            x_state[i][k] = Right;
                            _cycle_detection(x, x_state, i, k);
                        } else {
                            return;
                        }
                        break;
                    }
                }
            }
        }

        _cycle_detection(&self.trans, &mut trans_state, i, j);

        trans_state
    }
    // If the number of basic cells in the transportation plan is less then
    // m + n - 1, it must be replenished
    fn replenish(&mut self) {
        let number = self.trans.data.iter().filter(|&&a| a != None).count();

        if number < self.trans.rows() + self.trans.cols() - 1 {
            for i in 0..self.trans.rows() {
                for j in 0..self.trans.cols() {
                    if self.trans[i][j] == None {
                        self.trans[i][j] = Some(0);
                        if self.cycle_detection(i, j)[i][j] != Direction::None {
                            self.trans[i][j] = None;
                        } else {
                            return;
                        }
                    }
                }
            }
        }
    }
    // Recursive calculation of potentials
    fn calculation_of_potentials(&self) -> (Vec<i64>, Vec<i64>) {
        let mut u = vec![0;self.trans.rows()];
        let mut v = vec![0;self.trans.cols()];

        fn calculation_of_potentials_h(c: &Matrix<u64>, x: &Matrix<Option<u64>>, u: &mut [i64], v: &mut [i64], i: usize, j: usize) {
            for k in 0..x.cols() {
                if x[i][k] != None {
                    v[k] = c[i][k] as i64 - u[i];
                    if k != j {
                        calculation_of_potentials_v(c, x, u, v, i, k);
                    }
                }
            }
        }

        fn calculation_of_potentials_v(c: &Matrix<u64>, x: &Matrix<Option<u64>>, u: &mut [i64], v: &mut [i64], i: usize, j: usize) {
            for k in 0..x.rows() {
                if x[k][j] != None {
                    u[k] = c[k][j] as i64 - v[j];
                    if k != i {
                        calculation_of_potentials_h(c, x, u, v, k, j);
                    }
                }
            }
        }

        calculation_of_potentials_h(&self.cost, &self.trans, &mut u, &mut v, 0, 0);
        calculation_of_potentials_v(&self.cost, &self.trans, &mut u, &mut v, 0, 0);

        (u, v)
    }
    // Check for optimality
    // If the transportation plan is not optimal, then return the minimum of difference
    fn check(&self) -> Option<(usize, usize, i64)> {
        let mut min = None;
        let (u, v) = self.calculation_of_potentials();

        for i in 0..self.cost.rows() {
            for j in 0..self.cost.cols() {
                let d = self.cost[i][j] as i64 - u[i] - v[j];
                if d < 0 {
                    if let Some((_, _, m)) = min {
                        if m > d {
                            min = Some((i, j, d));
                        }
                    } else {
                        min = Some((i, j, d));
                    }
                }
            }
        }
        min
    }
    // Calculate the total cost
    fn total_cost(&self) -> u64 {
        let mut z = 0;
        for (t, c) in self.trans.data.iter().zip(self.cost.data.iter()) {
            if !t.is_none() {
                z += t.unwrap() * c;
            }
        }
        z
    }

    pub fn potential_method(&mut self) {
        self.least_cost_method();
        self.replenish();

        while let Some((i, j, _)) = self.check() {
            self.trans[i][j] = Some(0);
            let trans_state = self.cycle_detection(i, j);

            let mut min = std::u64::MAX;
            let mut f = true;
            for x in trans_state.iter(i, j) {
                if f {
                    f = false;
                } else {
                    if min > self.trans[x.0][x.1].unwrap() {
                        min = self.trans[x.0][x.1].unwrap();
                    }
                    f = true;
                }
            }

            f = true;
            for x in trans_state.iter(i, j) {
                if f {
                    self.trans[x.0][x.1] = Some(self.trans[x.0][x.1].unwrap() + min);
                    f = false;
                } else {
                    self.trans[x.0][x.1] = Some(self.trans[x.0][x.1].unwrap() - min);
                    f = true;
                }
            }

            let mut max = (0, 0, 0);
            for x in trans_state.iter(i, j) {
                if self.trans[x.0][x.1].unwrap() == 0 && max.2 <= self.cost[x.0][x.1] {
                    max = (x.0, x.1, self.cost[x.0][x.1]);
                }
            }
            self.trans[max.0][max.1] = None;
        }
    }

    pub fn printstd(&self) {
        let mut table = Table::new();
        table.add_row(Row::new(vec![Cell::new("")]));
        for i in 0..self.demand.len() {
            table[0].add_cell(Cell::new(&format!("B{}", i + 1)));
        }
        table[0].add_cell(Cell::new("Запасы"));
        for (i, s) in self.supply.iter().enumerate() {
            table.add_row(Row::new(vec![Cell::new(&format!("A{}", i + 1))]));
            for m in self.trans[i].iter() {
                if m.is_none() {
                    table[i + 1].add_cell(Cell::new("0"));
                } else {
                    table[i + 1].add_cell(Cell::new(&format!("{}", m.unwrap())));
                }
            }
            table[i + 1].add_cell(Cell::new(&format!("{}", s)));
        }
        table.add_row(Row::new(vec![Cell::new("Потребности")]));
        for d in self.demand.iter() {
            table[self.supply.len() + 1].add_cell(Cell::new(&format!("{}", d)));
        }
        table.add_row(Row::new(vec![Cell::new("Общая стоимость"), Cell::new(&format!("{}", self.total_cost()))]));
        table.printstd();
    }

    pub fn new(mut a: Vec<u64>, mut b: Vec<u64>, mut c: Vec<Vec<u64>>) -> Result<Transportation, Error> {
        if a.is_empty() || b.is_empty() {
            return Err(NumOfSupOrCust);
        }
        if a.len() != c.len() {
            return Err(NumOfRows);
        }
        for i in &c {
            if b.len() != i.len() {
                return Err(NumOfCols);
            }
        }

        let sum_s = a.iter().fold(0, std::ops::Add::add); //a.iter().sum();
        let sum_d = b.iter().fold(0, std::ops::Add::add); //b.iter().sum();
        if sum_s > sum_d {
            b.push(sum_s - sum_d);
            for i in &mut c {
                i.push(0);
            }
        }
        if sum_s < sum_d {
            a.push(sum_d - sum_s);
            c.push(vec![0;b.len()]);
        }
        let mut cost = Matrix::new(c[0].len());
        for i in c {
            cost.push(i);
        }

        Ok(Transportation {
            trans: Matrix {
                cols: b.len(),
                data: vec![None;a.len()*b.len()],
            },
            supply: a,
            demand: b,
            cost: cost,
        })
    }
}
