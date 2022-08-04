use cursive::Cursive;
use cursive::views::{Dialog, LinearLayout, Panel};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

mod cell;
mod board;
mod generator;
mod cellview;
mod boardview;


fn main() {
    let mut cells: Arc<Vec<Rc<RefCell<cell::Cell>>>> = Arc::new(Vec::new());

    for _ in 0..81 {
      let cell = Rc::new(RefCell::new(cell::Cell::new()));
      Arc::get_mut(&mut cells).unwrap().push(Rc::clone(&cell));
    }

    let new_cells = Arc::clone(&cells);
    let reset_cells = Arc::clone(&cells);
    let check_cells = Arc::clone(&cells);

    let mut siv = cursive::default();
    let bv = boardview::BoardView::new(Arc::clone(&cells));
    siv.add_layer(
      Dialog::new()
          .title("Sudoku")
          .content(
              LinearLayout::horizontal()
                  .child(Panel::new(bv)),
          )
          .button("New", move |_| {
            new_game(&new_cells);
          })
          .button("Reset", move |_| {
            reset_game(&reset_cells);
          })
          .button("Check", move |s| {
            check_game(s, &check_cells);
          })
          .button("Quit", |s| {
              s.quit();
          })
    );
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn new_game(cells: &Arc<Vec<Rc<RefCell<cell::Cell>>>>) {
  let game = generator::generate_game();
  let puzzle = generator::dig_holes(&game);
  let game_vec = board::game_str_to_vec(&puzzle).unwrap();
  for index in 0..81 {
    let c = &cells[index];
    let mut item = c.try_borrow_mut().unwrap();
    item.reset();
    if game_vec[index] > 0 {
      item.set_value(game_vec[index]);
      item.set_readonly(true);
    } else {
      item.fill_candidates();
    }
  }
}

fn reset_game(cells: &Arc<Vec<Rc<RefCell<cell::Cell>>>>) {
  for index in 0..81 {
    let c = &cells[index];
    let mut item = c.try_borrow_mut().unwrap();

    if !item.is_readonly() {
      item.reset();
      item.fill_candidates();
    }
  }
}

fn check_game(s: &mut Cursive, cells: &Arc<Vec<Rc<RefCell<cell::Cell>>>>) {
  let mut game = Vec::new();

  for index in 0..81 {
    let c = &cells[index];
    let item = c.try_borrow().unwrap();

    if item.is_fixed() {
      game.push(item.get_value());
    } else {
      game.push(0);
    }
  }
  let mut board = board::Board::new();
  board.load_game(&game);

  s.add_layer(Dialog::text( match board.is_solved() {
    true => "Game solved.",
    _ => "Game not solved."
  })
      .title("Result")
      .button("Ok", |s| {
        s.pop_layer();
      }));
}
