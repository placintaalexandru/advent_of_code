use crate::day24::blizzard::Blizzard;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Pixel {
    Land,
    Rock,
    Blizzard(Blizzard),
    Blizzards(Vec<Blizzard>),
}

impl From<&Pixel> for char {
    fn from(pixel: &Pixel) -> Self {
        match pixel {
            Pixel::Land => '.',
            Pixel::Rock => '#',
            Pixel::Blizzard(blizzard) => (&blizzard.direction).into(),

            // Can't have more than 4 blizzards on a single position
            Pixel::Blizzards(blizzards) => ('0' as u8 + blizzards.len() as u8) as char,
        }
    }
}
