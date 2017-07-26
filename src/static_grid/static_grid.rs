use std::slice;
use cgmath::Vector2;
use limits::LimitsRect;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticGrid<T> {
    items: Vec<T>,
    width: u32,
    height: u32,
    size: u32,
}

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

    fn wrap(&self, coord: Vector2<u32>) -> u32 {
        coord.y * self.width + coord.x
    }

    pub fn contains(&self, coord: Vector2<u32>) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn contains_signed(&self, coord: Vector2<i32>) -> bool {
        coord.x >= 0 && coord.y >= 0 && (coord.x as u32) < self.width && (coord.y as u32) < self.height
    }

    pub fn get(&self, coord: Vector2<u32>) -> Option<&T> {
        if coord.x < self.width {
            self.items.get(self.wrap(coord) as usize)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, coord: Vector2<u32>) -> Option<&mut T> {
        if coord.x < self.width {
            let idx = self.wrap(coord);
            self.items.get_mut(idx as usize)
        } else {
            None
        }
    }

    pub fn get_signed(&self, coord: Vector2<i32>) -> Option<&T> {
        if coord.x >= 0 && coord.y >= 0 {
            self.get(coord.cast())
        } else {
            None
        }
    }

    pub fn get_signed_mut(&mut self, coord: Vector2<i32>) -> Option<&mut T> {
        if coord.x >= 0 && coord.y >= 0 {
            self.get_mut(coord.cast())
        } else {
            None
        }
    }

    pub fn get_valid(&self, coord: Vector2<u32>) -> Option<&T> {
        self.items.get(self.wrap(coord) as usize)
    }

    pub fn get_valid_mut(&mut self, coord: Vector2<u32>) -> Option<&mut T> {
        let idx = self.wrap(coord);
        self.items.get_mut(idx as usize)
    }

    pub unsafe fn get_unchecked(&self, coord: Vector2<u32>) -> &T {
        self.items.get_unchecked(self.wrap(coord) as usize)
    }

    pub unsafe fn get_unchecked_mut(&mut self, coord: Vector2<u32>) -> &mut T {
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
        (&self, base: Vector2<u32>, into_iter: IntoIter) -> NeighbourCoordIter<IntoOffset, Iter>
    where IntoOffset: Into<Vector2<i32>>,
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

    pub fn neighbour_coord_signed_iter<IntoOffset, Iter, IntoIter>
        (&self, base: Vector2<i32>, into_iter: IntoIter) -> NeighbourCoordIter<IntoOffset, Iter>
    where IntoOffset: Into<Vector2<i32>>,
          Iter: Iterator<Item=IntoOffset>,
          IntoIter: IntoIterator<Item=IntoOffset, IntoIter=Iter>,
    {
        NeighbourCoordIter {
            width: self.width,
            height: self.height,
            base: base,
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
    type Item = Vector2<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.height {
            return None
        }

        let ret = Some(Vector2::new(self.x, self.y));

        self.x += 1;
        if self.x == self.width {
            self.x = 0;
            self.y += 1;
        }

        ret
    }
}

pub struct NeighbourCoordIter<IntoOffset: Into<Vector2<i32>>, Iter: Iterator<Item=IntoOffset>> {
    width: u32,
    height: u32,
    base: Vector2<i32>,
    iter: Iter,
}

impl<IntoOffset: Into<Vector2<i32>>, Iter: Iterator<Item=IntoOffset>> Iterator for NeighbourCoordIter<IntoOffset, Iter> {
    type Item = Vector2<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(offset) = self.iter.next() {
            let coord = self.base + offset.into();
            if coord.x >= 0 && coord.y >= 0 {
                let coord = Vector2::new(coord.x as u32, coord.y as u32);
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
