use crate::{board::Board, board::game_str_to_vec};
use rand::{self, thread_rng, prelude::SliceRandom};

const MIN_CELLS: u8 = 17;

pub fn generate_game() -> String {
  let mut board = Board::new();
  board.fill_candidates();

  // set 1-9 randomly to nine of the cells
  for i in 0..9 {
    let index = select_non_fixed(&board);
    board.assign_cell(index, i + 1);
  }

  let board_state = board.backup();
  try_init_game(&mut board);

  loop {
    let (solved, _) = try_solve_game(&mut board); // board.solve_concurrent(); //
    if !solved {
      println!("can not solve, try another init");
      board.restore(&board_state);
      try_init_game(&mut board);
    } else {
      break;
    }
  }

  board.serialize()
}

fn dig_holes(game: &str) -> String {
  let remain_cells = MIN_CELLS + random_index(6);
  let mut can_dig_cells = [true; 81];
  let mut game_vec = game_str_to_vec(game).unwrap();
  let mut total = 81;

  while total > remain_cells {
    let start = next_diggable_index(&can_dig_cells);
    if start > 80 {
      break;
    }

    if is_game_has_unique_solution(&mut game_vec, start as usize) {
      game_vec[start] = 0;
      total -= 1;
    }
    can_dig_cells[start] = false;
  }
  game_vec.into_iter().map(|item| item.to_string()).collect()
}

fn next_diggable_index(diggable_cells: &[bool;81]) -> usize {
  let mut candidates = Vec::new();

  for index in 0..81 {
    if diggable_cells[index] {
      candidates.push(index);
    }
  }
  if candidates.len() > 0 {
    let mut rng = thread_rng();
    candidates.shuffle(&mut rng);
    return candidates[0];
  }

  return 81;
}

fn random_index(total: u8) -> u8 {
  rand::random::<u8>() % total
}

/**
 * In one trail of digging a hole, suppose we try to dig a cell filled with the digit 6, then
 * Step 1: substitute the digit 6 into another new one from 1 through 9 one by one
 * excluding 6 while meeting the game rules;
 * Step 2: call the solver to solve the puzzle with the givens including the new digit.
 * Step 3: once the solver reports a solution, terminate the solver and claim that the
 * puzzle generated by digging out the digit 6 into empty cell has two solutions at least
 * because originally a solution exists when the cell is filled with the digit 6.
 * Step 4: only if all rest 8 digits excluding 6 are used to do such a trial in step 1 and 2
 * and the solver reports none solution, it is safe to claim that the puzzle generated by
 * digging out the digit 6 into empty cell has a unique solution, which means that the
 * operation of digging out the digit 6 is feasible and legal.
 */
fn is_game_has_unique_solution(game_vec: &mut Vec<u8>, index: usize) -> bool {
  let cell_value = game_vec[index];

  for value in 1..10 {
    if value == cell_value {
      continue;
    }
    game_vec[index] = value;
    if Board::is_valid_game(game_vec) {
      let mut board = Board::new();
      if let Ok(_) = board.init(game_vec) {
        let (solved, _) = try_solve_game(&mut board);
        if solved {
          game_vec[index] = cell_value; // restore
          return false;
        }
      }
    }
  }
  true
}

fn select_non_fixed(board: &Board) -> u8 {
  loop {
    let next_id = random_index(81);
    let cell = &board.cells[next_id as usize];
    if cell.is_fixed() {
      continue;
    }
    return next_id;
  }
}

/**
 * Reference
 * Finally, we find that randomly creating a puzzle with 11 givens can help to minimize
 * the computational time and meanwhile enhance the diversity of the generated
 * puzzles.
 */
fn try_init_game(board: &mut Board) {
  let mut count = 0;
  loop {
    let index:u8 = select_non_fixed(&board);
    let cell = &board.cells[index as usize];
    let mut candidates = cell.collect_candidates();
    let mut rng = thread_rng();
    candidates.shuffle(&mut rng);

    let board_state = board.backup();

    for c in &candidates {
      let assign_result = board.assign_cell(index, *c);
      if assign_result {
        count += 1;
        break;
      } else {
        board.restore(&board_state);
      }
    }
    if count >= 2 {
      break;
    }
  }
}

fn try_solve_game(board: &mut Board) -> (bool, usize) {
  if board.is_solved() {
    return (true, 0);
  }

  let (index, mut candidates) = board.next_candidate_cell();
  let mut rng = thread_rng();
  candidates.shuffle(&mut rng);
  let board_state = board.backup();
  let mut steps: usize = 0;

  for c in candidates {
    steps += 1;
    if steps > 100 {
      // too many steps, try another solution
      return (false, steps);
    }

    let assign_result = board.assign_cell(index, c);
    if assign_result {
      // assign succeed, continue to solve the game
      let (solve_result, solve_steps) = try_solve_game(board);
      steps += solve_steps;

      if solve_result {
        return (true, steps);
      }
    }
    // current candidate failed
    // restore the board and test next candidate
    board.restore(&board_state);
  }

  // all candidates failed
  (false, steps)
}

#[cfg(test)]
mod tests {
    use super::{random_index, generate_game, dig_holes};
    use rand::{self, thread_rng, prelude::SliceRandom};
    use crate::board::{Board, game_str_to_vec};

    #[test]
    fn test_random_index() {
      println!("{}", random_index(81));
      println!("{}", random_index(81));
      println!("{}", random_index(81));
    }

    #[test]
    fn test_shuffle() {
      let mut v = vec![1,2,3,4,5,6,7,8,9];
      let mut rng = thread_rng();
      v.shuffle(&mut rng);
      println!("Shuffled:   {:?}", v);
    }

    #[test]
    fn test_genera_new_game() {
      let g = generate_game();
      println!("{}", g);
      let p = dig_holes(&g);
      println!("{}", p);

      let mut board = Board::new();
      let game_vec = game_str_to_vec(&p).unwrap();
      board.init(&game_vec).unwrap();
      let success = board.solve();
      assert_eq!(success, true);
      println!("{}", board.serialize());
    }
}