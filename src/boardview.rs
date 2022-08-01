use crate::board::Board;
use crate::generator;
use crate::cellview::{CellView, CellMode};
use cursive::{
  direction::Direction,
  event::{Event, Key, EventResult, MouseButton, MouseEvent},
  theme::{BaseColor, Color, ColorStyle},
  view::CannotFocus,
  views::{Button, Dialog, LinearLayout, Panel, SelectView},
  Cursive, Printer, Vec2,
};

const CELL_SIZE: usize = 4;
const BORDER_SIZE: usize = 36; // 4 * 9
pub struct BoardView {
  // Actual board, unknown to the player.
  board: Board,

  // Visible board
  // overlay: Vec<Cell>,

  focused: Option<usize>,

  cellviews: Vec<CellView>
}

impl BoardView {
  pub fn new() -> Self {
    let mut board = Board::new();
    let mut cellviews = Vec::new();
    for cell in &mut board.cells {
      cell.fill_candidates();
      let cv = CellView::new(cell.clone(), CellMode::Draft, false);
      cellviews.push(cv);
    }
    BoardView {
        board,
        cellviews,
        focused: None
    }
  }

  pub fn set_folus_cell(&mut self, index: usize) {
    match self.focused {
        Some(i) => {
          let cv = CellView::new(self.board.cells[i as usize].clone(), CellMode::Draft, false);
          self.cellviews[i as usize] = cv;
          let cv2 = CellView::new(self.board.cells[index].clone(), CellMode::Draft, true);
          self.cellviews[index] = cv2;
        },
        _ => {
          let cv = CellView::new(self.board.cells[index].clone(), CellMode::Draft, true);
          self.cellviews[index] = cv;
        }
    }
    self.focused = Some(index)
  }
}

impl cursive::view::View for BoardView {
  fn draw(&self, printer: &Printer) {
    for index in 0..81 {
      let r = (index / 9) * CELL_SIZE + 1;
      let c = (index % 9) * CELL_SIZE + 1;

      let cv = &self.cellviews[index];
      cv.draw(&printer.offset(Vec2::new(c,r)));
    }
    // print h lines
    for i in 0..10 {
      if i % 3 == 0 {
        printer.print_hline((0, i * 4), BORDER_SIZE, "#");
      } else {
        printer.print_hline((0, i * 4), BORDER_SIZE, "-");
      }
    }
    // print v lines
    for i in 0..10 {
      if i % 3 == 0 {
        printer.print_vline((i * 4, 0), BORDER_SIZE, "#");
      } else {
        printer.print_vline((i * 4, 0), BORDER_SIZE, "⎮");
      }
    }
  }

  fn take_focus(
      &mut self,
      _: Direction,
  ) -> Result<EventResult, CannotFocus> {
      Ok(EventResult::Consumed(None))
  }

  fn on_event(&mut self, event: Event) -> EventResult {
      match event {
          Event::Key(Key::Up) => {
            match self.focused {
                Some(f) => {
                  if f >= 9 {
                    self.set_folus_cell(f - 9);
                  }
                },
                _ => {
                  self.set_folus_cell(0);
                }
            }
          }
          Event::Key(Key::Down) => {
            match self.focused {
                Some(f) => {
                  if f < 72 {
                    self.set_folus_cell(f + 9);
                  }
                },
                _ => {
                  self.set_folus_cell(0);
                }
            }
          }
          Event::Key(Key::Left) => {
            match self.focused {
                Some(f) => {
                  if f > 0 {
                    self.set_folus_cell(f - 1);
                  }
                },
                _ => {
                  self.set_folus_cell(0);
                }
            }
          }
          Event::Key(Key::Right) => {
            match self.focused {
                Some(f) => {
                  if f < 80 {
                    self.set_folus_cell(f + 1);
                  }
                },
                _ => {
                  self.set_folus_cell(0);
                }
            }
          }
          _ => (),
      }

      EventResult::Ignored
  }

  fn required_size(&mut self, _: Vec2) -> Vec2 {
      Vec2::new(BORDER_SIZE + 1, BORDER_SIZE + 1)
  }
}