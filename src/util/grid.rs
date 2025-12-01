#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Grid<T> {
    pub fn from_scalar(data: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![data; width * height],
            width,
            height,
        }
    }

    pub fn from_vec(data: Vec<T>, width: usize, height: usize) -> Option<Self>
    where
        T: Clone,
    {
        if data.len() == width * height {
            Some(Self {
                data,
                height,
                width,
            })
        } else {
            None
        }
    }

    pub fn from_slice(data: &[T], width: usize, height: usize) -> Option<Self>
    where
        T: Clone,
    {
        if data.len() == width * height {
            Some(Self {
                data: Vec::from(data),
                height,
                width,
            })
        } else {
            None
        }
    }

    pub fn from_iter<'a, S: 'a, F>(
        iter: impl Iterator<Item = &'a S>,
        width: usize,
        mut f: F,
    ) -> Option<Self>
    where
        F: FnMut(&'a S) -> T,
    {
        let mut data = vec![];
        for s in iter {
            data.push(f(s));
        }

        if data.len() % width == 0 {
            let height = data.len() / width;
            Some(Self {
                data,
                height,
                width,
            })
        } else {
            None
        }
    }

    /// parses a string to a grid, determining width and height automatically based on newlines (`\n`)
    pub fn from_str<F>(s: &str, mut f: F) -> Option<Self>
    where
        F: FnMut(char) -> T,
    {
        let mut data = vec![];
        let mut width = None;
        let mut n = 0;
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
                continue;
            }

            data.push(f(c));
            n += 1;
        }

        let height = data.len() / width?;

        Some(Self {
            data,
            width: width?,
            height,
        })
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        if (0..self.width).contains(&(x as usize)) && (0..self.height).contains(&(y as usize)) {
            let i = x as usize + y as usize * self.width;
            self.data.get(i)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if (0..self.width).contains(&(x as usize)) && (0..self.height).contains(&(y as usize)) {
            let i = x as usize + y as usize * self.width;
            self.data.get_mut(i)
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = ((i32, i32), &T)> {
        GridIterator::new(self)
    }

    pub fn neighbors(
        &self,
        x: i32,
        y: i32,
        include_self: bool,
    ) -> impl Iterator<Item = ((i32, i32), &T)> {
        NeighborsIterator::new(self, x, y, include_self)
    }

    pub fn orthogonal_neighbors(
        &self,
        x: i32,
        y: i32,
        include_self: bool,
    ) -> impl Iterator<Item = ((i32, i32), &T)> {
        OrthogonalNeighborsIterator::new(self, x, y, include_self)
    }

    pub fn pretty_print<F>(&self, mut f: F)
    where
        F: FnMut(&T) -> &str,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                let Some(t) = self.get(x as i32, y as i32) else {
                    unreachable!()
                };
                print!("{}", f(t))
            }
            println!()
        }
    }
}

impl<T> std::ops::Index<(i32, i32)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (i32, i32)) -> &Self::Output {
        match self.get(x, y) {
            None => panic!(
                "index out of bounds: the grid size is ({}, {}), but the index is ({}, {})",
                self.width, self.height, x, y
            ),
            Some(t) => t,
        }
    }
}

impl<T> std::ops::IndexMut<(i32, i32)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (i32, i32)) -> &mut Self::Output {
        self.get_mut(x, y)
            .unwrap_or_else(|| panic!("index out of bounds: index is ({x}, {y})"))
    }
}

impl<T> std::ops::Index<super::Vec2> for Grid<T> {
    type Output = T;
    fn index(&self, super::Vec2 { x, y }: super::Vec2) -> &Self::Output {
        match self.get(x, y) {
            None => panic!(
                "index out of bounds: the grid size is ({}, {}), but the index is ({}, {})",
                self.width, self.height, x, y
            ),
            Some(t) => t,
        }
    }
}

impl<T> std::ops::IndexMut<super::Vec2> for Grid<T> {
    fn index_mut(&mut self, super::Vec2 { x, y }: super::Vec2) -> &mut Self::Output {
        self.get_mut(x, y)
            .unwrap_or_else(|| panic!("index out of bounds: index is ({x}, {y})"))
    }
}

pub struct GridIterator<'a, T> {
    grid: &'a Grid<T>,
    i: usize,
}

impl<'a, T> GridIterator<'a, T> {
    fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, i: 0 }
    }
}

impl<'a, T> Iterator for GridIterator<'a, T> {
    type Item = ((i32, i32), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        let x = (self.i % self.grid.width) as i32;
        let y = (self.i / self.grid.width) as i32;

        let res = self.grid.data.get(self.i).map(|t| ((x, y), t));
        self.i += 1;
        res
    }
}

pub struct NeighborsIterator<'a, T> {
    grid: &'a Grid<T>,
    x: i32,
    y: i32,
    include_self: bool,
    i: usize,
}

impl<'a, T> NeighborsIterator<'a, T> {
    fn new(grid: &'a Grid<T>, x: i32, y: i32, include_self: bool) -> Self {
        Self {
            grid,
            x,
            y,
            i: 0,
            include_self,
        }
    }
}

impl<'a, T> Iterator for NeighborsIterator<'a, T> {
    type Item = ((i32, i32), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        const DIR: &[(i32, i32)] = &[
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let mut found = None;
        while found.is_none() {
            if self.i >= DIR.len() {
                break;
            }
            let (dx, dy) = DIR[self.i];
            if let Some(t) = self.grid.get(self.x + dx, self.y + dy) {
                if !self.include_self && dx == 0 && dy == 0 {
                    // do nothing
                } else {
                    found = Some(((self.x + dx, self.y + dy), t));
                }
            }

            self.i += 1;
        }

        found
    }
}

pub struct OrthogonalNeighborsIterator<'a, T> {
    grid: &'a Grid<T>,
    x: i32,
    y: i32,
    include_self: bool,
    i: usize,
}

impl<'a, T> OrthogonalNeighborsIterator<'a, T> {
    fn new(grid: &'a Grid<T>, x: i32, y: i32, include_self: bool) -> Self {
        Self {
            grid,
            x,
            y,
            i: 0,
            include_self,
        }
    }
}

impl<'a, T> Iterator for OrthogonalNeighborsIterator<'a, T> {
    type Item = ((i32, i32), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        const DIR: &[(i32, i32)] = &[(0, -1), (-1, 0), (0, 0), (1, 0), (0, 1)];
        let mut found = None;
        while found.is_none() {
            if self.i >= DIR.len() {
                break;
            }
            let (dx, dy) = DIR[self.i];
            if let Some(t) = self.grid.get(self.x + dx, self.y + dy) {
                if !self.include_self && dx == 0 && dy == 0 {
                    // do nothing
                } else {
                    found = Some(((self.x + dx, self.y + dy), t));
                }
            }

            self.i += 1;
        }

        found
    }
}
