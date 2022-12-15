use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct StartingItems {
    items: VecDeque<usize>,
}

impl FromStr for StartingItems {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.trim_start().split(":").collect::<Vec<&str>>();

        if tokens[0] != "Starting items" {
            return Err(format!("String `{}` does not have first part right", s));
        }

        let tokens = tokens[1].trim_start().split(',').collect::<Vec<&str>>();

        return Ok(Self {
            items: tokens
                .into_iter()
                .map(|item_worry_level| {
                    item_worry_level
                        .trim_start()
                        .parse::<usize>()
                        .expect(&format!("Expected number. Got `{}`", item_worry_level))
                })
                .collect(),
        });
    }
}

impl StartingItems {
    pub(crate) fn push_back(&mut self, value: usize) {
        self.items.push_back(value)
    }

    pub(crate) fn pop_front(&mut self) -> Option<usize> {
        self.items.pop_front()
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let starting_items = StartingItems::from_str("  Starting items: 98, 89, 52").unwrap();
        assert_eq!(starting_items.items, vec![98, 89, 52]);
    }
}
