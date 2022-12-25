/// Tuple that keeps index now, value, and original index.
///
/// We need both index now and original index to know the same
/// order of processing items
#[derive(Debug, Clone)]
pub(crate) struct IndexValuePair(pub(crate) usize, pub(crate) i64);

#[derive(Debug, Clone)]
pub(crate) struct Mixer {
    pub(crate) positions: Vec<IndexValuePair>,
}

impl Mixer {
    pub(crate) fn new(values: Vec<i64>) -> Self {
        Self {
            positions: values
                .into_iter()
                .enumerate()
                .map(|(idx, value)| IndexValuePair(idx, value))
                .collect(),
        }
    }

    fn shift(&mut self, idx: usize) {
        let length = self.positions.len();
        let element_to_update = &self.positions[idx];
        let old_position = element_to_update.0;
        let mut new_position = (element_to_update.0 as i64 + element_to_update.1)
            .rem_euclid(length as i64 - 1) as usize;

        if new_position == 0 {
            new_position = self.positions.len() - 1;
        }

        // The new position does not change as the offset is a multiple of list length
        if new_position == old_position {
            return;
        }

        if new_position < old_position {
            self.positions
                .iter_mut()
                .rev()
                .filter(|tuple| tuple.0 >= new_position && tuple.0 < old_position)
                .for_each(|tuple| tuple.0 += 1);
        } else {
            self.positions
                .iter_mut()
                .filter(|tuple| tuple.0 <= new_position && tuple.0 > old_position)
                .for_each(|tuple| tuple.0 -= 1);
        }

        self.positions.get_mut(idx).unwrap().0 = new_position;
    }

    pub(crate) fn shift_rounds(&mut self, rounds: usize) {
        (0..rounds).for_each(|_| {
            (0..self.positions.len()).for_each(|i| {
                self.shift(i);
            });
        })
    }
}

impl From<&Mixer> for Vec<i64> {
    fn from(shifter: &Mixer) -> Self {
        let mut result = Vec::with_capacity(shifter.positions.len());

        unsafe {
            shifter.positions.iter().for_each(|position| {
                (result.as_mut_ptr() as *mut i64)
                    .offset(position.0 as isize)
                    .write(position.1);
            });

            result.set_len(result.capacity());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_left_simple() {
        let mut shifter = Mixer::new(vec![1, 2, -3, 3, -2, 0, 4]);

        assert_eq!(Vec::<i64>::from(&shifter), vec![1, 2, -3, 3, -2, 0, 4]);
        shifter.shift(4);
        assert_eq!(Vec::<i64>::from(&shifter), vec![1, 2, -2, -3, 3, 0, 4]);
    }

    #[test]
    fn shift_left_circular1() {
        let mut shifter = Mixer::new(vec![-1, 2, -3, 3, -2, 0, 4]);

        assert_eq!(Vec::<i64>::from(&shifter), vec![-1, 2, -3, 3, -2, 0, 4]);
        shifter.shift(0);
        assert_eq!(Vec::<i64>::from(&shifter), vec![2, -3, 3, -2, 0, -1, 4,]);
    }

    #[test]
    fn shift_left_circular2() {
        let mut shifter = Mixer::new(vec![4, -2, 5, 6, 7, 8, 9]);

        assert_eq!(Vec::<i64>::from(&shifter), vec![4, -2, 5, 6, 7, 8, 9]);
        shifter.shift(1);
        assert_eq!(Vec::<i64>::from(&shifter), vec![4, 5, 6, 7, 8, -2, 9]);
    }

    #[test]
    fn shift_right_simple1() {
        let mut shifter = Mixer::new(vec![1, 2, -3, 3, -2, 0, 4]);

        assert_eq!(Vec::<i64>::from(&shifter), vec![1, 2, -3, 3, -2, 0, 4]);
        shifter.shift(0);
        assert_eq!(Vec::<i64>::from(&shifter), vec![2, 1, -3, 3, -2, 0, 4]);
    }

    #[test]
    fn shift_right_circular1() {
        let mut shifter = Mixer::new(vec![1, 2, -3, 3, -2, 0, 4]);

        assert_eq!(Vec::<i64>::from(&shifter), vec![1, 2, -3, 3, -2, 0, 4]);
        shifter.shift(3);
        assert_eq!(Vec::<i64>::from(&shifter), vec![3, 1, 2, -3, -2, 0, 4]);
    }

    #[test]
    fn shift_right_circular2() {
        let mut shifter = Mixer::new(vec![1, 2, -3, 3, -2, 0, 4]);

        assert_eq!(Vec::<i64>::from(&shifter), vec![1, 2, -3, 3, -2, 0, 4]);
        shifter.shift(6);
        assert_eq!(Vec::<i64>::from(&shifter), vec![1, 2, -3, 3, 4, -2, 0]);
    }
}
