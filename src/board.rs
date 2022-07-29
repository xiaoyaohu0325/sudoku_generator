use std::{fmt, thread};
use std::sync::{Mutex, Arc};
extern crate num_cpus;
use crate::cell::{Cell, one_hot};

type CellState = (u8, u16);

type BoardState = Vec<CellState>;

struct Strategy {
  state: BoardState,
  index: u8,
  value: u8
}

type Unit = [u8; 9];

const ROWS: [Unit; 9] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8],
    [9, 10, 11, 12, 13, 14, 15, 16, 17],
    [18, 19, 20, 21, 22, 23, 24, 25, 26],
    [27, 28, 29, 30, 31, 32, 33, 34, 35],
    [36, 37, 38, 39, 40, 41, 42, 43, 44],
    [45, 46, 47, 48, 49, 50, 51, 52, 53],
    [54, 55, 56, 57, 58, 59, 60, 61, 62],
    [63, 64, 65, 66, 67, 68, 69, 70, 71],
    [72, 73, 74, 75, 76, 77, 78, 79, 80],
];

const COLS: [Unit; 9] = [
    [0, 9, 18, 27, 36, 45, 54, 63, 72],
    [1, 10, 19, 28, 37, 46, 55, 64, 73],
    [2, 11, 20, 29, 38, 47, 56, 65, 74],
    [3, 12, 21, 30, 39, 48, 57, 66, 75],
    [4, 13, 22, 31, 40, 49, 58, 67, 76],
    [5, 14, 23, 32, 41, 50, 59, 68, 77],
    [6, 15, 24, 33, 42, 51, 60, 69, 78],
    [7, 16, 25, 34, 43, 52, 61, 70, 79],
    [8, 17, 26, 35, 44, 53, 62, 71, 80],
];

const BLOCKS: [Unit; 9] = [
    [0, 1, 2, 9, 10, 11, 18, 19, 20],
    [3, 4, 5, 12, 13, 14, 21, 22, 23],
    [6, 7, 8, 15, 16, 17, 24, 25, 26],
    [27, 28, 29, 36, 37, 38, 45, 46, 47],
    [30, 31, 32, 39, 40, 41, 48, 49, 50],
    [33, 34, 35, 42, 43, 44, 51, 52, 53],
    [54, 55, 56, 63, 64, 65, 72, 73, 74],
    [57, 58, 59, 66, 67, 68, 75, 76, 77],
    [60, 61, 62, 69, 70, 71, 78, 79, 80],
];

fn get_row_unit(row: u8) -> &'static Unit {
  &ROWS[row as usize]
}

fn get_col_unit(col: u8) -> &'static Unit {
  &COLS[col as usize]
}

fn get_block_unit_by_index(index: u8) -> &'static Unit {
  &BLOCKS[index as usize]
}

fn get_block_unit_by_pos(row: u8, col: u8) -> &'static Unit {
  let block_row = row / 3;
  let block_col = col / 3;
  let block_id = block_row * 3 + block_col;

  get_block_unit_by_index(block_id)
}

pub struct Board {
    cells: [Cell; 81],
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [Cell::new(); 81],
        }
    }

    // reset all cells value
    pub fn reset(&mut self) {
      for cell in &mut self.cells {
        cell.reset();
      }
    }

    /**
   * A game is a string of length 81, each character maps to a cell. An example:
   * 4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......
   */
    pub fn load_game(&mut self, game: &str) {
      if game.len() != 81 {
        return
      }
      self.reset();
      for i in 0..81 {
        if let Some(value) = game.chars().nth(i).unwrap().to_digit(10) {
          let cell = &mut self.cells[i];
          cell.set_value(value as u8);
        }
      }
    }

    // fill candidates, then load a new game and update candidates
    pub fn init(&mut self, game: &str) -> Result<bool, bool> {
      if game.len() != 81 {
        return Err(false);
      }
      self.reset();

      for cell in &mut self.cells {
        cell.fill_candidates();
      }

      for i in 0..81 {
        if let Some(value) = game.chars().nth(i).unwrap().to_digit(10) {
          if !self.assign_cell(i as u8, value as u8) {
            return Err(false);
          }
        }
      }
      Ok(true)
    }

    /**
     * Backup a board to an array of values.
     * Each cell has two ints, first is value and second is candidates
     */
    fn backup(&self) -> Vec<(u8, u16)> {
      let mut result: Vec<(u8, u16)> = Vec::with_capacity(81);
      for cell in &self.cells {
        result.push(cell.backup());
      }
      result
    }

    fn restore(&mut self, data: &Vec<(u8, u16)>) {
      for i in 0..81 {
        let cell = &mut self.cells[i];
        let value = data[i];
        cell.restore(value);
      }
    }

    /**
     * cells in a unit should have unique values
     */
    fn is_unit_compatible(&self, unit: &Unit) -> bool {
      let mut result: u16 = 0;
      for index in unit {
        let cell = &self.cells[*index as usize];
        if cell.is_fixed() {
          let value = one_hot(cell.get_value());
          if (value & result) > 0 {
            return false
          } else {
            result += value;
          }
        }
      }
      true
    }

    fn is_solved(&self) -> bool {
      // 1. all cells are fixed
      for cell in &self.cells {
        if !cell.is_fixed() {
          return false;
        }
      }
      // 2. all units are compatible
      for i in 0..9 {
        let row = get_row_unit(i);
        if !self.is_unit_compatible(row) {
          return false;
        }
        let col = get_col_unit(i);
        if !self.is_unit_compatible(col) {
          return false;
        }
        let block = get_block_unit_by_index(i);
        if !self.is_unit_compatible(block) {
          return false;
        }
      }
      return true;
    }

    /**
     * try to assign a new value to a cell, check validity during the process.
     */
    fn assign_cell(&mut self, index: u8, value: u8) -> bool {
      let cell = &mut self.cells[index as usize];
      let row = index / 9;
      let col = index % 9;

      cell.set_value(value);

      let row_unit = get_row_unit(row);
      if !self.eliminate_unit_candidate(row_unit, value) {
        return false;
      }

      let col_unit = get_col_unit(col);
      if !self.eliminate_unit_candidate(col_unit, value) {
        return false;
      }

      let block_unit = get_block_unit_by_pos(row, col);
      if !self.eliminate_unit_candidate(block_unit, value) {
        return false;
      }
      true
    }

    // remove the candidate from specified units
    fn eliminate_unit_candidate(&mut self, unit: &Unit, candidate: u8) -> bool {
      for index in unit {
        let cell = &mut self.cells[*index as usize];

        if cell.is_fixed() {
          continue;
        }

        if let Err(_) = cell.eliminate_candidate(candidate) {
          return false;
        }
        // only has one candidate after removing
        let fixed = cell.is_fixed();
        if fixed {
          let value = cell.get_value();
          if !self.assign_cell(*index, value) {
            return false;
          }
        }
        if !self.check_unit_content(unit) {
          return false;
        }
        if !self.is_unit_compatible(unit) {
          return false;
        }
      }
      true
    }

    /**
     * If a candidate is only contained in one cell of a unit, then it must be the value of that cell.
     */
    fn check_unit_content(&mut self, unit: &Unit) -> bool {
      for candidate in 1..=9 {
        let mut fixed = false;
        let mut cell_has_it: Option<u8> = None;
        let mut num_cells = 0;

        for index in unit {
          let cell = &self.cells[*index as usize];
          if cell.is_fixed() && cell.get_value() == candidate {
            fixed = true;
            break;
          } else if cell.has_candidate(candidate).0 {
            num_cells += 1;
            cell_has_it = Some(*index);
          }
        }

        if !fixed && num_cells == 1 {
          if !self.assign_cell(cell_has_it.unwrap(), candidate) {
            return false;
          }
        }
      }
      true
    }

    // find the cell that has minimum candidates
    fn next_candidate_cell(&self) -> (u8, Vec<u8>) {
      let mut index:u8 = 0;
      let mut min_candidates = 10;

      for idx in 0..81 {
        let cell = &self.cells[idx];
        if !cell.is_fixed() {
          let count = cell.num_candidates();
          if count > 0 && count < min_candidates {
            index = idx as u8;
            min_candidates = count;
          }
        }
      }

      let cell = &self.cells[index as usize];
      let candidates = cell.collect_candidates();

      return (index, candidates)
    }

    /**
     * try to solve current game
     */
    fn solve(&mut self) -> bool {
      if self.is_solved() {
        return true;
      }

      let (index, candidates) = self.next_candidate_cell();
      let board_state = self.backup();

      // let row = index / 9 + 1;
      // let col = index % 9 + 1;
      // let label = cell.candidates_str();
      // println!("Cell of row {} col {} has candidates: {}", row, col, label);

      for candidate in candidates {
        // println!("{}", self);
        // println!("assign {} to row {} col {}", candidate, row, col);
        if self.assign_cell(index, candidate) {
          // assign succeed, continue to solve the game
          if self.solve() {
            return true;
          }
        }
        // current candidate failed
        // println!("row {} col {} can not be {}, try another one\n", row, col, candidate);
        // restore the board and test next candidate
        self.restore(&board_state);
      }

      false
    }

    // tasks: [(board_state, index, value)]
    // thread extract task to execute until solved
    fn solve_concurrent(&mut self) -> bool {
      // create send/receiver vars
      // to move data through channel
      // thread-safe and lockable
      let cores = num_cpus::get();

      let (index, candidates) = self.next_candidate_cell();
      let board_state = self.backup();
      let mut init_strategy: Vec<Strategy> = Vec::new();
      for candidate in candidates {
        init_strategy.push(Strategy {
          index,
          state: board_state.clone(),
          value: candidate
        });
      }

      let solved = Arc::new(Mutex::new(false));
      let strategies = Arc::new(Mutex::new(init_strategy));

      // more than one handle
      // store them in a vec for convenience
      let mut handles = vec![];
      // println!("num of cores: {}", cores);

      for i in 0..cores {
          // clone the transmitter
          let solved = Arc::clone(&solved);
          let strategies = Arc::clone(&strategies);
          let thread_id = i;

          // create the thread
          let handle = thread::spawn(move || {
            loop {
              // lock the value
              {
                let finished = solved.lock().unwrap();

                if *finished {
                  break;
                }
              }

              let mut next_strategy: Option<Strategy> = None;
              {
                let mut s_pool = strategies.lock().unwrap();
                next_strategy = s_pool.pop();
              }

              if let Some(s) = next_strategy {
                let mut next_board = Board::new();
                next_board.restore(&s.state);

                // println!("thread {} assign cell {} with value {}", thread_id, s.index, s.value);
                if next_board.assign_cell(s.index, s.value) {
                  if next_board.is_solved() {
                    let mut s_pool = strategies.lock().unwrap();
                    let mut finished = solved.lock().unwrap();
                    // println!("solved in thread {}", thread_id);
                    *finished = true;
                    s_pool.clear();
                    s_pool.push(Strategy {
                      index: 0,
                      state: next_board.backup(),
                      value: 0
                    });
                  } else {
                    let finished = solved.lock().unwrap();
                    if !*finished {
                      let mut s_pool = strategies.lock().unwrap();
                      let (index, candidates) = next_board.next_candidate_cell();
                      let board_state = next_board.backup();
                      // println!("thread {} add cell {} with candidate {:?}", thread_id, index, candidates);
                      for candidate in candidates {
                        s_pool.push(Strategy {
                          index,
                          state: board_state.clone(),
                          value: candidate
                        });
                      }
                    }
                  }
                }
              }
            }
          });

          // push the handle into the handles
          // vector so we can join them later
          handles.push(handle);
      }

      // join the handles in the vector
      for i in handles {
        i.join().unwrap();
      }

      // all thread finished
      let mut s_pool = strategies.lock().unwrap();
      if let Some(s) = s_pool.pop() {
        self.reset();
        self.restore(&s.state);
      }
      let finished = solved.lock().unwrap();
      *finished
    }

    pub fn serialize(&self) -> String {
      let mut result = String::new();
      for cell in &self.cells {
        result.push(cell.serialize());
      }
      result
    }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let groups = [[1,2,3], [4,5,6], [7,8,9]];
    writeln!(f, "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++")?;

    for i in 0..9 {
      let row = &ROWS[i];

      for j in 0..3 {
        let group = &groups[j];

        for k in 0..9 {
          let idx = row[k] as usize;
          let cell: &Cell = &self.cells[idx];
          if k % 3 == 0 {
            write!(f, "┼ ")?;
          } else {
            write!(f, "⏐ ")?;
          }

          for n in 0..3 {
            if cell.is_fixed() && group[n] == 5 {
              let v = cell.get_value();
              write!(f, "{v} ")?;
            } else if cell.has_candidate(group[n]).0 {
              let v = group[n];
              write!(f, "{v} ")?;
            } else {
              write!(f, "  ")?;
            }
          }
        }
        writeln!(f, "┼")?;
      }
      if i % 3 == 2 {
        writeln!(f, "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++")?;
      } else {
        writeln!(f, "-------------------------------------------------------------------------")?;
      }
    }

    return Ok(());
  }
}

#[cfg(test)]
mod tests {
    use super::{Board, get_row_unit, get_col_unit, get_block_unit_by_pos};
    const GAME: &str = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";

    #[test]
    fn is_load_work() {
      let mut b = Board::new();
      b.load_game(GAME);
      assert_eq!(b.cells[0].get_value(), 4);
      assert_eq!(b.cells[6].get_value(), 8);
      assert_eq!(b.cells[74].get_value(), 4);
      println!("{}", b);
    }

    #[test]
    fn is_init_works() {
        let mut b = Board::new();
        assert_eq!(b.init(GAME), Ok(true));
        println!("{}", b);
    }

    #[test]
    fn is_backup_restore_works() {
        let mut b = Board::new();
        assert_eq!(b.init(GAME), Ok(true));
        let s1 = b.serialize();

        let backup = b.backup();
        b.reset();
        let s2 = b.serialize();
        assert_ne!(s1, s2);

        b.restore(&backup);
        let s3 = b.serialize();
        assert_eq!(s1, s3);
    }

    #[test]
    fn is_serialize_works() {
        let mut b = Board::new();
        b.load_game(GAME);
        assert_eq!(b.serialize(), GAME);
    }

    #[test]
    fn is_get_units_works() {
        let row = get_row_unit(3);
        let col = get_col_unit(3);
        let block = get_block_unit_by_pos(3, 3);

        for i in 0..9 {
          // row index from 27-35
          assert_eq!(row[i], (27 + i) as u8);
          // row index from 3-75, step 9
          assert_eq!(col[i], (3 + i * 9) as u8);
          // block index [30, 31, 32, 39, 40, 41, 48, 49, 50]
          let m = i % 3;
          let d = i / 3;
          assert_eq!(block[i], (30 + m + d * 9) as u8)
        }
    }

    #[test]
    fn is_solve_works() {
      let mut b = Board::new();
      assert_eq!(b.init(GAME), Ok(true));
      println!("{}", b);
      let success = b.solve();
      assert_eq!(success, true);
      println!("{}", b);
    }

    #[test]
    fn is_solve_concurrent_works() {
      let mut b = Board::new();
      assert_eq!(b.init(GAME), Ok(true));
      println!("{}", b);
      let success = b.solve_concurrent();
      assert_eq!(success, true);
      println!("{}", b);
    }
  }