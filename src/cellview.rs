use crate::cell::Cell;
use cursive::{
  direction::Direction,
  event::{Event, EventResult, MouseButton, MouseEvent},
  theme::{BaseColor, Color, ColorStyle},
  view::CannotFocus,
  views::{Button, Dialog, LinearLayout, Panel, SelectView},
  Cursive, Printer, Vec2,
};

pub enum CellMode {
  Draft, Edit
}

pub struct CellView {
  cell: Cell,
  mode: CellMode,
  active: bool
}

impl CellView {
  pub fn new(cell: Cell, mode: CellMode, active: bool) -> Self {
    CellView {
      cell, mode, active
    }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = active;
  }

  pub fn set_mode(&mut self, mode: CellMode) {
    self.mode = mode;
  }

  fn print_value(&self, printer: &Printer, value: u8, pos: Vec2, with_style: bool) {
    if with_style {
      printer.with_color(
        ColorStyle::new(Color::Dark(BaseColor::Black), Color::Light(BaseColor::White)),
        |printer| printer.print(pos, format!("{}", value).as_str()),
      );
    } else {
      printer.print(pos, format!("{}", value).as_str());
    }
  }
}

impl cursive::view::View for CellView {
  fn draw(&self, printer: &Printer) {
    if matches!(self.mode, CellMode::Edit) && self.cell.is_fixed() {
      self.print_value(printer, self.cell.get_value(), Vec2::new(1, 1), self.active);
    } else if matches!(self.mode, CellMode::Draft) {
      for v in 0..9 {
        let r = v / 3;
        let c = v % 3;

        if self.cell.has_candidate(v+1 as u8).0 {
          self.print_value(printer, v+1, Vec2::new(c as usize, r as usize), self.active);
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
    Vec2::new(3, 3)
  }
}