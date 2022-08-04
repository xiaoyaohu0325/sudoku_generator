use crate::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use cursive::{
  direction::Direction,
  event::EventResult,
  theme::{BaseColor, Color, ColorStyle},
  view::CannotFocus,
  Rect,
  Printer, Vec2
};

#[derive(Debug, Copy, Clone)]
pub enum CellMode {
  Draft, Edit
}

pub struct CellView {
  cell: Rc<RefCell<Cell>>,
  mode: CellMode,
  active: bool,
  highlight: bool
}

impl CellView {
  pub fn new(cell: Rc<RefCell<Cell>>) -> Self {
    CellView {
      cell, mode: CellMode::Edit, active: false, highlight: false
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = active;
  }

  pub fn set_highlight(&mut self, highlight: bool) {
    self.highlight = highlight;
  }

  pub fn set_mode(&mut self, mode: CellMode) {
    self.mode = mode;
  }

  pub fn get_mode(&self) -> CellMode {
    self.mode
  }
}

impl cursive::view::View for CellView {
  fn draw(&self, printer: &Printer) {
    let cellref = self.cell.borrow();

    // three status: normal, active (currently selected cell), highlight (has the same value as the selected cell)
    // two types of cell: readonly, editable
    let style = if cellref.is_readonly() {
      if self.active {
        ColorStyle::title_primary()
      } else if self.highlight {
        ColorStyle::highlight_inactive()
      } else {
        ColorStyle::secondary()
      }
    } else {
      if self.active {
        ColorStyle::highlight()
      } else if self.highlight {
        ColorStyle::highlight_inactive()
      } else {
        ColorStyle::primary()
      }
    };

    if cellref.is_fixed() {
      printer.with_color(
        style,
        |printer| printer.print((2, 1), format!("{}", cellref.get_value()).as_str()),
      );
    } else {
      for v in 0..9 {
        let r = v / 3;
        let c = (v % 3) * 2;

        if cellref.has_candidate(v+1 as u8).0 && matches!(self.mode, CellMode::Draft) {
          printer.with_color(
            style,
            |printer| printer.print((c as usize, r as usize), format!("{}", v+1).as_str()),
          );
        } else {
          printer.with_color(
            style,
            |printer| printer.print((c as usize, r as usize), "."),
          );
        }
      }
    }
  }

  fn take_focus(
    &mut self,
    _: Direction,
  ) -> Result<EventResult, CannotFocus> {
      Ok(EventResult::Consumed(None))
  }

  fn required_size(&mut self, _: Vec2) -> Vec2 {
    Vec2::new(6, 3)
  }

  fn important_area(&self, view_size: Vec2) -> Rect {
    Rect::from_size((0, 0), view_size)
  }
}