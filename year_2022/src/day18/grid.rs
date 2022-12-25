use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Coordinate(pub(crate) i8, pub(crate) i8, pub(crate) i8);

pub(crate) struct Grid {
    pub(crate) cubes: HashSet<Coordinate>,
    pub(crate) bot_left: Coordinate,
    pub(crate) top_right: Coordinate,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cubes: Default::default(),

            bot_left: Coordinate(i8::MAX, i8::MAX, i8::MAX),
            top_right: Coordinate(i8::MIN, i8::MIN, i8::MIN),
        }
    }
}

impl Grid {
    pub(crate) fn add(&mut self, point: Coordinate) {
        self.top_right.0 = self.top_right.0.max(point.0);
        self.top_right.1 = self.top_right.1.max(point.1);
        self.top_right.2 = self.top_right.2.max(point.2);

        self.bot_left.0 = self.bot_left.0.min(point.0);
        self.bot_left.1 = self.bot_left.1.min(point.1);
        self.bot_left.2 = self.bot_left.2.min(point.2);

        self.cubes.insert(point);
    }

    pub(crate) fn surface(&self) -> usize {
        self.cubes
            .iter()
            .map(|coordinate| {
                [
                    !self
                        .cubes
                        .contains(&Coordinate(coordinate.0 - 1, coordinate.1, coordinate.2))
                        as usize,
                    !self
                        .cubes
                        .contains(&Coordinate(coordinate.0 + 1, coordinate.1, coordinate.2))
                        as usize,
                    !self
                        .cubes
                        .contains(&Coordinate(coordinate.0, coordinate.1 - 1, coordinate.2))
                        as usize,
                    !self
                        .cubes
                        .contains(&Coordinate(coordinate.0, coordinate.1 + 1, coordinate.2))
                        as usize,
                    !self
                        .cubes
                        .contains(&Coordinate(coordinate.0, coordinate.1, coordinate.2 - 1))
                        as usize,
                    !self
                        .cubes
                        .contains(&Coordinate(coordinate.0, coordinate.1, coordinate.2 + 1))
                        as usize,
                ]
                .iter()
                .sum::<usize>()
            })
            .sum()
    }

    fn inside_limit(&self, position: &Coordinate) -> bool {
        self.bot_left.0 <= position.0
            && position.0 <= self.top_right.0
            && self.bot_left.1 <= position.1
            && position.1 <= self.top_right.1
            && self.bot_left.2 <= position.2
            && position.2 <= self.top_right.2
    }

    pub(crate) fn flood(&self, position: Coordinate, water_regions: &mut HashSet<Coordinate>) {
        if water_regions.contains(&position)
            || self.cubes.contains(&position)
            || !self.inside_limit(&position)
        {
            return;
        }

        water_regions.insert(position.clone());

        self.flood(
            Coordinate(position.0 + 1, position.1, position.2),
            water_regions,
        );
        self.flood(
            Coordinate(position.0 - 1, position.1, position.2),
            water_regions,
        );

        self.flood(
            Coordinate(position.0, position.1 - 1, position.2),
            water_regions,
        );
        self.flood(
            Coordinate(position.0, position.1 + 1, position.2),
            water_regions,
        );

        self.flood(
            Coordinate(position.0, position.1, position.2 - 1),
            water_regions,
        );
        self.flood(
            Coordinate(position.0, position.1, position.2 + 1),
            water_regions,
        );
    }
}
