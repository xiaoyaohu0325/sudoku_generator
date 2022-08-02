use cursive::{Cursive, Vec2};
use cursive::views::{Dialog, LinearLayout, Panel};
use cursive::event::{Event, Key};
use std::cell::RefCell;
use std::rc::Rc;

mod cell;
mod board;
mod generator;
mod cellview;
mod boardview;


fn main() {
    let mut cells: Vec<Rc<RefCell<cell::Cell>>> = Vec::new();
    let mut cells_cp: Vec<Rc<RefCell<cell::Cell>>> = Vec::new();

    for _ in 0..81 {
      let cell = Rc::new(RefCell::new(cell::Cell::new()));
      cells.push(Rc::clone(&cell));
      cells_cp.push(Rc::clone(&cell));
    }

    let mut siv = cursive::default();
    let bv = boardview::BoardView::new(cells_cp);
    siv.add_layer(
      Dialog::new()
          .title("Sudoku")
          .content(
              LinearLayout::horizontal()
                  .child(Panel::new(bv)),
          )
          .button("New game", move |s| {
            let game = generator::generate_game();
            let puzzle = generator::dig_holes(&game);
            let game_vec = board::game_str_to_vec(&puzzle).unwrap();
            for index in 0..81 {
              let c = &cells[index];
              let mut item = c.borrow_mut();
              item.reset();
              if game_vec[index] > 0 {
                item.set_value(game_vec[index]);
                item.set_readonly(true);
              } else {
                item.fill_candidates();
              }
            }
          })
          .button("Quit game", |s| {
              s.pop_layer();
          })
    );
    siv.add_global_callback('q', |s| s.quit());

    // siv.add_global_callback(Event::Key(Key::Enter), |s| {
    //   s.add_layer(
    //     Dialog::new()
    //         .content(
    //             LinearLayout::horizontal()
    //                 .child(Panel::new(cellview::CellView::new(Rc::clone(&cells[0]), cellview::CellMode::Draft, true))),
    //         )
    //   );
    // });

    siv.add_global_callback(Event::Key(Key::Esc), |s| {
      s.pop_layer();
    });

    siv.run();
}