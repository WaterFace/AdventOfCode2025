use std::collections::HashMap;

use super::{vec2, Vec2};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseGrid<T> {
    pub data: HashMap<Vec2, T>,
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}

impl<T> SparseGrid<T> {
    pub fn new() -> Self {
        let top_left = vec2(i32::MAX, i32::MAX);
        let bottom_right = vec2(i32::MIN, i32::MIN);
        let data = HashMap::new();
        Self {
            data,
            top_left,
            bottom_right,
        }
    }

    pub fn from_iter<'a, S: 'a, F>(
        iter: impl Iterator<Item = &'a S>,
        width: usize,
        mut f: F,
    ) -> Self
    where
        F: FnMut(&'a S) -> Option<T>,
    {
        let mut grid = SparseGrid::new();
        for (i, s) in iter.enumerate() {
            let x = (i % width) as i32;
            let y = (i / width) as i32;
            if let Some(t) = f(s) {
                grid.insert(vec2(x, y), t);
            }
        }

        grid
    }

    /// parses a string to a grid, determining width and height automatically based on newlines (`\n`)
    pub fn from_str<F>(s: &str, mut f: F) -> Option<Self>
    where
        F: FnMut(char) -> Option<T>,
    {
        let mut grid = SparseGrid::new();

        let mut width = None;
        let mut n = 0;
        let mut y = 0;
        for c in s.chars() {
            if c == '\n' {
                if let Some(w) = width {
                    if n == w {
                        n = 0;
                        continue;
                    } else {
                        return None;
                    }
                } else {
                    width = Some(n);
                }

                n = 0;
                y += 1;
                continue;
            }

            let x = n;
            if let Some(t) = f(c) {
                grid.insert(vec2(x, y), t);
            }
            n += 1;
        }

        Some(grid)
    }
    pub fn get(&self, i: &Vec2) -> Option<&T> {
        self.data.get(i)
    }

    pub fn set(&mut self, i: Vec2, v: T) -> Option<T> {
        self.insert(i, v)
    }

    pub fn insert(&mut self, i: Vec2, v: T) -> Option<T> {
        self.expand_bounds(i);
        self.data.insert(i, v)
    }

    fn expand_bounds(&mut self, i: Vec2) {
        self.top_left.x = i32::min(i.x, self.top_left.x);
        self.top_left.y = i32::min(i.y, self.top_left.y);

        self.bottom_right.x = i32::max(i.x, self.bottom_right.x);
        self.bottom_right.y = i32::max(i.y, self.bottom_right.y);
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vec2, &T)> {
        self.data.iter().map(|(k, v)| (*k, v))
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn pretty_print<F>(&self, mut f: F)
    where
        F: FnMut(Option<&T>) -> char,
    {
        for y in self.top_left.y..self.bottom_right.y + 1 {
            for x in self.top_left.x..self.bottom_right.x + 1 {
                print!("{}", f(self.get(&vec2(x, y))))
            }
            println!()
        }
    }
}

impl<T: Default + Clone> SparseGrid<T> {
    pub fn get_or_default(&self, i: &Vec2) -> T {
        self.get(i).cloned().unwrap_or_default()
    }
}

impl<T> IntoIterator for SparseGrid<T> {
    type Item = (Vec2, T);
    type IntoIter = <HashMap<Vec2, T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<T> Default for SparseGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}
