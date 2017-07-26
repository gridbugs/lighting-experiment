use std::convert::TryFrom;
use cgmath::Vector2;
use static_grid::Coord;

impl From<Coord> for Vector2<u32> {
    fn from(coord: Coord) -> Self {
        Vector2::new(coord.x, coord.y)
    }
}

impl From<Vector2<u32>> for Coord {
    fn from(v: Vector2<u32>) -> Self {
        Coord::new(v.x, v.y)
    }
}

impl<'a> From<&'a Vector2<u32>> for Coord {
    fn from(v: &'a Vector2<u32>) -> Self {
        Coord::new(v.x, v.y)
    }
}

impl From<Coord> for Vector2<i32> {
    fn from(coord: Coord) -> Self {
        Vector2::new(coord.x as i32, coord.y as i32)
    }
}

impl TryFrom<Vector2<i32>> for Coord {
    type Error = ();
    fn try_from(v: Vector2<i32>) -> Result<Self, Self::Error> {
        if v.x >= 0 && v.y >= 0 {
            Ok(Coord::new(v.x as u32, v.y as u32))
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a Vector2<i32>> for Coord {
    type Error = ();
    fn try_from(v: &'a Vector2<i32>) -> Result<Self, Self::Error> {
        if v.x >= 0 && v.y >= 0 {
            Ok(Coord::new(v.x as u32, v.y as u32))
        } else {
            Err(())
        }
    }
}
