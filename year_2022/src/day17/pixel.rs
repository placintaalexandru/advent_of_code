#[derive(Clone, Eq, PartialEq)]
pub(crate) enum Pixel {
    Empty,
    Rock,
}

impl From<&Pixel> for char {
    fn from(pixel: &Pixel) -> Self {
        match *pixel {
            Pixel::Empty => '.',
            Pixel::Rock => '#',
        }
    }
}
