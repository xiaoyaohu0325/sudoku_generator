use crate::cell::Cell;
use crate::cellview::{CellMode, CellView};
use cursive::{
    direction::Direction,
    event::{Event, EventResult, Key},
    view::CannotFocus,
    Printer, Vec2,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

const CELL_WIDTH: usize = 7;
const CELL_HEIGHT: usize = 4;
const BORDER_WIDTH: usize = 63; // 7 * 9
const BORDER_HEIGHT: usize = 36; // 4 * 9

pub struct BoardView {
    cells: Arc<Vec<Rc<RefCell<Cell>>>>,

    focused: Option<usize>,

    cellviews: Vec<CellView>,
}

impl BoardView {
    pub fn new(cells: Arc<Vec<Rc<RefCell<Cell>>>>) -> Self {
        let mut cellviews = Vec::new();
        for i in 0..81 {
            let cell = &cells[i];
            let cv = CellView::new(Rc::clone(cell));
            cellviews.push(cv);
        }
        BoardView {
            cells,
            cellviews,
            focused: None,
        }
    }

    pub fn set_folus_cell(&mut self, index: usize) {
        match self.focused {
            Some(i) => {
                let cv = &mut self.cellviews[i as usize];
                cv.set_active(false);
                let cv2 = &mut self.cellviews[index as usize];
                cv2.set_active(true);
            }
            _ => {
                let cv = &mut self.cellviews[index as usize];
                cv.set_active(true);
            }
        }
        self.focused = Some(index);

        // clear all highlighted
        for cv in &mut self.cellviews {
            cv.set_highlight(false);
        }

        // set highlight cells that has the same value as current focused one
        let item = &self.cells[index];
        let cell = item.borrow();
        let is_fixed = cell.is_fixed();
        let v = cell.get_value();
        if is_fixed {
            for i in 0..81 {
                let item = &self.cells[i];
                let c = item.borrow();
                if c.get_value() == v {
                    let cv = &mut self.cellviews[i as usize];
                    cv.set_highlight(true);
                }
            }
        }
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        for index in 0..81 {
            let r = (index / 9) * CELL_HEIGHT + 1;
            let c = (index % 9) * CELL_WIDTH + 1;

            let cv = &self.cellviews[index];
            cv.draw(&printer.offset(Vec2::new(c, r)));
        }
        // print h lines
        for i in 0..10 {
            if i % 3 == 0 {
                printer.print_hline((0, i * CELL_HEIGHT), BORDER_WIDTH, "#");
            } else {
                printer.print_hline((0, i * CELL_HEIGHT), BORDER_WIDTH, "-");
            }
        }
        // print v lines
        for i in 0..10 {
            if i % 3 == 0 {
                printer.print_vline((i * CELL_WIDTH, 0), BORDER_HEIGHT, "#");
            } else {
                printer.print_vline((i * CELL_WIDTH, 0), BORDER_HEIGHT, "⎮");
            }
        }
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Key(Key::Up) => match self.focused {
                Some(f) => {
                    if f >= 9 {
                        self.set_folus_cell(f - 9);
                    }
                }
                _ => {
                    self.set_folus_cell(0);
                }
            },
            Event::Key(Key::Down) => match self.focused {
                Some(f) => {
                    if f < 72 {
                        self.set_folus_cell(f + 9);
                    }
                }
                _ => {
                    self.set_folus_cell(0);
                }
            },
            Event::Key(Key::Left) => match self.focused {
                Some(f) => {
                    if f > 0 {
                        self.set_folus_cell(f - 1);
                    }
                }
                _ => {
                    self.set_folus_cell(0);
                }
            },
            Event::Key(Key::Right) => match self.focused {
                Some(f) => {
                    if f < 80 {
                        self.set_folus_cell(f + 1);
                    }
                }
                _ => {
                    self.set_folus_cell(0);
                }
            },
            Event::Char(c) => {
                if let Some(d) = c.to_digit(10) {
                    if d > 0 {
                        if let Some(index) = self.focused {
                            let cv = &self.cellviews[index];
                            let item = Rc::clone(&self.cells[index]);
                            let mut cell = item.try_borrow_mut().unwrap();
                            if matches!(cv.get_mode(), CellMode::Edit) {
                                if !cell.is_fixed() {
                                    cell.set_value(d as u8);
                                    // set highlight cells that has the same value as current focused one
                                    for i in 0..81 {
                                        if i == index {
                                            continue;
                                        }
                                        let item = &self.cells[i];
                                        let c = item.borrow();
                                        if c.get_value() == d as u8 {
                                            let cv = &mut self.cellviews[i];
                                            cv.set_highlight(true);
                                        }
                                    }
                                }
                            } else {
                                cell.toggle_candidate(d as u8);
                            }
                        }
                    }
                }
                if c == 'd' {
                    if let Some(index) = self.focused {
                        let cv = &mut self.cellviews[index];
                        cv.set_mode(CellMode::Draft);
                        let item = Rc::clone(&self.cells[index]);
                        let mut cell = item.try_borrow_mut().unwrap();
                        if cell.num_candidates() == 0 {
                            cell.fill_candidates();
                        }
                    }
                }
                if c == 'e' {
                    if let Some(index) = self.focused {
                        let cv = &mut self.cellviews[index];
                        cv.set_mode(CellMode::Edit);
                    }
                }
                if c == 'c' {
                    if let Some(index) = self.focused {
                        let item = Rc::clone(&self.cells[index]);
                        let mut cell = item.try_borrow_mut().unwrap();
                        if !cell.is_readonly() {
                            cell.clear_value();
                            cell.clear_candidates();
                        }
                    }
                }
            }
            _ => (),
        }

        EventResult::Ignored
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(BORDER_WIDTH + 1, BORDER_HEIGHT + 1)
    }
}
