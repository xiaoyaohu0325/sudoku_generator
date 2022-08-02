use crate::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use cursive::{
  direction::Direction,
  event::{Event, EventResult, MouseButton, MouseEvent},
  theme::{BaseColor, Color, ColorStyle},
  view::CannotFocus,
  views::{Button, Dialog, LinearLayout, Panel, SelectView},
  Cursive, Printer, Vec2,
};

#[derive(Debug, Copy, Clone)]
pub enum CellMode {
  Draft, Edit
}

pub struct CellView {
  cell: Rc<RefCell<Cell>>,
  mode: CellMode,
  active: bool
}

impl CellView {
  pub fn new(cell: Rc<RefCell<Cell>>, mode: CellMode, active: bool) -> Self {
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

  pub fn get_mode(&self) -> CellMode {
    self.mode
  }

  fn print_value(&self, printer: &Printer, value: u8, pos: Vec2, with_style: bool) {
    let mut v_str = ".";
    let formated_value = format!("{}", value);
    if value > 0 {
      v_str = formated_value.as_str();
    }
    if with_style {
      printer.with_color(
        ColorStyle::new(Color::Dark(BaseColor::Black), Color::Light(BaseColor::White)),
        |printer| printer.print(pos, v_str),
      );
    } else {
      printer.print(pos, v_str);
    }
  }
}

impl cursive::view::View for CellView {
  fn draw(&self, printer: &Printer) {
    let cellref = self.cell.borrow();
    if cellref.is_fixed() {
      if cellref.is_readonly() {
        printer.with_color(
          ColorStyle::new(Color::Dark(BaseColor::Black), Color::RgbLowRes(5, 0, 3)),
          |printer| printer.print(Vec2::new(2, 1), format!("{}", cellref.get_value()).as_str()),
        );
      } else {
        self.print_value(printer, cellref.get_value(), Vec2::new(2, 1), self.active);
      }
    } else {
      for v in 0..9 {
        let r = v / 3;
        let c = (v % 3) * 2;

        if cellref.has_candidate(v+1 as u8).0 && matches!(self.mode, CellMode::Draft) {
          self.print_value(printer, v+1, Vec2::new(c as usize, r as usize), self.active);
        } else {
          self.print_value(printer, 0, Vec2::new(c as usize, r as usize), self.active);
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
}