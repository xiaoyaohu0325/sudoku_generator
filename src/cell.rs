#[allow(dead_code)]
use std::fmt;
pub const SOLVED_VALUE: u16 = 511;

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    index: u8,
    value: u8,
    candidates: u16,
}

pub fn one_hot(value: u8) -> u16 {
    1 << (value - 1)
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            index: 0,
            value: 0,
            candidates: 0,
        }
    }

    pub fn set_index(&mut self, idx: u8) {
      self.index = idx;
    }

    pub fn get_index(&self) -> u8 {
        self.index
    }

    pub fn is_fixed(&self) -> bool {
        self.value > 0
    }

    pub fn add_candidate(&mut self, c: u8) {
        let (has, candidate) = self.has_candidate(c);
        if !has {
            self.candidates += candidate;
        }
    }

    pub fn remove_candidate(&mut self, c: u8) {
        let (has, candidate) = self.has_candidate(c);
        if has {
            self.candidates -= candidate;
        }
    }

    pub fn eliminate_candidate(&mut self, c: u8) -> Result<u8, u8> {
        let (has, _) = self.has_candidate(c);
        if has {
            let count = self.num_candidates();
            if count == 1 { // can not eliminate the last candidate
                return Err(c);
            }
            if count > 1 {
                self.remove_candidate(c);
                if count == 2 {
                    // remains one candidate after eliminating
                    self.apply_candidate();
                    return Ok(c);
                }
            }
        }
        Ok(0)
    }

    pub fn fill_candidates(&mut self) {
        if !self.is_fixed() {
            self.candidates = SOLVED_VALUE;
        }
    }

    pub fn clear_candidates(&mut self) {
        self.candidates = 0;
    }

    pub fn apply_candidate(&mut self) -> Option<u8> {
        match self.num_candidates() {
            1 => {
                self.value = (self.candidates.trailing_zeros() as u8) + 1;
                Some(self.value)
            }
            _ => None,
        }
    }

    // return two value, (has_candidate, one_hot)
    pub fn has_candidate(&self, c: u8) -> (bool, u16) {
        let candidate = one_hot(c);
        ((self.candidates & candidate) != 0, candidate)
    }

    pub fn num_candidates(&self) -> u8 {
        self.candidates.count_ones() as u8
    }

    // get all candidates as an array
    pub fn collect_candidates(&self) -> Vec<u8> {
        let mut result = Vec::new();
        for i in 1..10 {
            if self.has_candidate(i).0 {
                result.push(i);
            }
        }
        result
    }

    pub fn candidates_str(&self) -> String {
        return format!("{:b}", self.candidates);
    }

    pub fn get_value(&self) -> u8 {
      self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
        self.candidates = 0;
    }

    pub fn backup(&self) -> (u8, u16) {
        (self.value, self.candidates)
    }

    pub fn restore(&mut self, bak: (u8, u16)) {
        self.value = bak.0;
        self.candidates = bak.1;
    }

    pub fn reset(&mut self) {
        self.value = 0;
        self.candidates = 0;
    }

    pub fn serialize(&self) -> char {
      if self.is_fixed() {
        return char::from_digit(self.value as u32, 10).unwrap();
      }
      '.'
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_fixed() {
            let v = self.value;
            write!(f, "{v}")?;
        } else {
            write!(f, ".")?;
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::one_hot;
    use super::Cell;

    #[test]
    fn is_fixed_works() {
        let c = Cell::new();
        println!("{}", c);
        assert_eq!(c.is_fixed(), false);
    }

    #[test]
    fn is_one_hot_works() {
        for i in 1..10 {
            let result = one_hot(i);
            let trailing_zeros = result.trailing_zeros() as u8;
            assert_eq!(trailing_zeros, i - 1);
        }
    }

    #[test]
    fn is_add_candidate_works() {
        let mut c = Cell::new();
        c.add_candidate(1u8);
        assert_eq!(c.candidates, 1u16);

        c.add_candidate(5u8);
        assert_eq!(c.candidates, 0b10001u16);
        // add duplicate candidate
        c.add_candidate(5u8);
        assert_eq!(c.candidates, 0b10001u16);

        c.add_candidate(4u8);
        assert_eq!(c.candidates, 0b11001u16);
    }

    #[test]
    fn is_candidates_str_works() {
        let mut c = Cell::new();
        c.add_candidate(1u8);
        assert_eq!(c.candidates_str(), "1");

        c.add_candidate(5u8);
        assert_eq!(c.candidates_str(), "10001");

        c.add_candidate(4u8);
        assert_eq!(c.candidates_str(), "11001");
    }

    #[test]
    fn is_remove_candidate_works() {
        let mut c = Cell::new();
        for i in 1..10 {
            assert_eq!(c.has_candidate(i).0, false);
            c.add_candidate(i);
            assert_eq!(c.has_candidate(i).0, true);
            c.remove_candidate(i);
            assert_eq!(c.has_candidate(i).0, false);
        }
    }

    #[test]
    fn is_has_candidate_works() {
        let mut c = Cell::new();
        assert_eq!(c.has_candidate(1u8).0, false);
        c.add_candidate(1u8);
        assert_eq!(c.has_candidate(1u8).0, true);

        assert_eq!(c.has_candidate(5u8).0, false);
        c.add_candidate(5u8);
        assert_eq!(c.has_candidate(5u8).0, true);

        assert_eq!(c.has_candidate(4u8).0, false);
        c.add_candidate(4u8);
        assert_eq!(c.has_candidate(4u8).0, true);

        assert_eq!(c.has_candidate(2u8).0, false);
        assert_eq!(c.has_candidate(3u8).0, false);
        assert_eq!(c.has_candidate(6u8).0, false);
        assert_eq!(c.has_candidate(7u8).0, false);
        assert_eq!(c.has_candidate(9u8).0, false);
    }

    #[test]
    fn is_num_candidate_works() {
        let mut c = Cell::new();

        assert_eq!(c.num_candidates(), 0);

        for i in 1..10 {
            c.add_candidate(i);
            assert_eq!(c.num_candidates(), i);
        }
    }

    #[test]
    fn is_collect_candidate_works() {
        let mut c = Cell::new();
        let r = c.collect_candidates();
        assert_eq!(r.len(), 0);

        c.add_candidate(1u8);
        let r = c.collect_candidates();
        assert_eq!(r.len(), 1);
        assert_eq!(r[0], 1);

        c.add_candidate(5u8);
        let r = c.collect_candidates();
        assert_eq!(r.len(), 2);
        assert_eq!(r[0], 1);
        assert_eq!(r[1], 5);

        c.add_candidate(4u8);
        let r = c.collect_candidates();
        assert_eq!(r.len(), 3);
        assert_eq!(r[0], 1);
        assert_eq!(r[1], 4);
        assert_eq!(r[2], 5);
    }

    #[test]
    fn is_apply_candidate_works() {
        let mut c = Cell::new();
        assert_eq!(c.apply_candidate(), None);

        c.add_candidate(4u8);
        assert_eq!(c.apply_candidate(), Some(4));

        c.add_candidate(5u8);
        assert_eq!(c.apply_candidate(), None);
    }
}
