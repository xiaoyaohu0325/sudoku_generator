use cursive::{Cursive, Vec2};
use cursive::views::{Dialog, LinearLayout, Panel};

mod cell;
mod board;
mod generator;
mod cellview;
mod boardview;

fn main() {
    let mut siv = cursive::default();
    // let mut cell = cell::Cell::new();
    // cell.fill_candidates();
    // siv.add_layer(cellview::CellView::new(cell, cellview::CellMode::Draft, false));

    siv.add_layer(
      Dialog::new()
          .title("Sudoku")
          .content(
              LinearLayout::horizontal()
                  .child(Panel::new(boardview::BoardView::new())),
          )
          .button("Quit game", |s| {
              s.pop_layer();
          }),
    );
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}