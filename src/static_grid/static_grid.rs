use std::slice;
use std::convert::TryInto;
use cgmath::Vector2;
use limits::LimitsRect;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticGrid<T> {
    items: Vec<T>,
    width: u32,
    height: u32,
    size: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn new(x: u32, y: u32) -> Self {
        Coord {
            x: x,
            y: y,
        }
    }
}

type Offset = Vector2<i32>;

impl<T> StaticGrid<T> {
    fn new_with_capacity(width: u32, height: u32) -> Self {
        let size = width * height;
        StaticGrid {
            items: Vec::with_capacity(size as usize),
            width: width,
            height: height,
            size: size,
        }
    }

    pub fn new_call<F>(width: u32, height: u32, mut f: F) -> Self
        where F: FnMut(isize, isize) -> T
    {
        let mut grid = StaticGrid::new_with_capacity(width, height);

        for y in 0..height as isize {
            for x in 0..width as isize {
                grid.items.push(f(x, y));
            }
        }

        grid
    }
}

impl<T: Default> StaticGrid<T> {
    pub fn new_default(width: u32, height: u32) -> Self {
        let mut grid = Self::new_with_capacity(width, height);
        for _ in 0..grid.size {
            grid.items.push(Default::default());
        }
        grid
    }
}

impl<T: Copy> StaticGrid<T> {
    pub fn new_copy(width: u32, height: u32, item: T) -> Self {
        let mut grid = Self::new_with_capacity(width, height);
        for _ in 0..grid.size {
            grid.items.push(item);
        }
        grid
    }
}

impl<T> StaticGrid<T> {
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    fn wrap(&self, coord: Coord) -> u32 {
        coord.y * self.width + coord.x
    }

    pub fn contains(&self, coord: Coord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        if coord.x < self.width {
            self.items.get(self.wrap(coord) as usize)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        if coord.x < self.width {
            let idx = self.wrap(coord);
            self.items.get_mut(idx as usize)
        } else {
            None
        }
    }

    pub fn get_signed(&self, coord: Vector2<i32>) -> Option<&T> {
        if let Ok(coord) = coord.try_into() {
            self.get(coord)
        } else {
            None
        }
    }

    pub fn get_signed_mut(&mut self, coord: Vector2<i32>) -> Option<&mut T> {
        if let Ok(coord) = coord.try_into() {
            self.get_mut(coord)
        } else {
            None
        }
    }

    pub fn get_valid(&self, coord: Coord) -> Option<&T> {
        self.items.get(self.wrap(coord) as usize)
    }

    pub fn get_valid_mut(&mut self, coord: Coord) -> Option<&mut T> {
        let idx = self.wrap(coord);
        self.items.get_mut(idx as usize)
    }

    pub unsafe fn get_unchecked(&self, coord: Coord) -> &T {
        self.items.get_unchecked(self.wrap(coord) as usize)
    }

    pub unsafe fn get_unchecked_mut(&mut self, coord: Coord) -> &mut T {
        let idx = self.wrap(coord);
        self.items.get_unchecked_mut(idx as usize)
    }

    pub fn rows(&self) -> Rows<T> {
        self.items.chunks(self.width as usize)
    }

    pub fn iter(&self) -> Iter<T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.items.iter_mut()
    }

    pub fn coord_iter(&self) -> CoordIter {
        CoordIter::new(self.width, self.height)
    }

    pub fn neighbour_coord_iter<IntoOffset, Iter, IntoIter>
        (&self, base: Coord, into_iter: IntoIter) -> NeighbourCoordIter<IntoOffset, Iter>
    where IntoOffset: Into<Offset>,
          Iter: Iterator<Item=IntoOffset>,
          IntoIter: IntoIterator<Item=IntoOffset, IntoIter=Iter>,
    {
        NeighbourCoordIter {
            width: self.width,
            height: self.height,
            base: Vector2::new(base.x as i32, base.y as i32),
            iter: into_iter.into_iter(),
        }
    }
}

pub type Iter<'a, T> = slice::Iter<'a, T>;
pub type IterMut<'a, T> = slice::IterMut<'a, T>;

pub type Rows<'a, T> = slice::Chunks<'a, T>;

pub struct CoordIter {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}

impl CoordIter {
    pub fn new(width: u32, height: u32) -> Self {
        CoordIter {
            width: width,
            height: height,
            x: 0,
            y: 0,
        }
    }
}

impl Iterator for CoordIter {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.height {
            return None
        }

        let ret = Some(Coord::new(self.x, self.y));

        self.x += 1;
        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }

        ret
    }
}

pub struct NeighbourCoordIter<IntoOffset: Into<Offset>, Iter: Iterator<Item=IntoOffset>> {
    width: u32,
    height: u32,
    base: Offset,
    iter: Iter,
}

impl<IntoOffset: Into<Offset>, Iter: Iterator<Item=IntoOffset>> Iterator for NeighbourCoordIter<IntoOffset, Iter> {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(offset) = self.iter.next() {
            let coord = self.base + offset.into();
            if coord.x >= 0 && coord.y >= 0 {
                let coord = Coord::new(coord.x as u32, coord.y as u32);
                if coord.x < self.width && coord.y < self.height {
                    return Some(coord);
                }
            }
        }
        None
    }
}

impl<T> LimitsRect for StaticGrid<T> {
    fn x_min(&self) -> i32 { 0 }
    fn x_max(&self) -> i32 { self.width as i32 - 1 }
    fn y_min(&self) -> i32 { 0 }
    fn y_max(&self) -> i32 { self.height as i32 - 1 }
}
