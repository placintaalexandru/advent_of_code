use std::ops::Deref;

use std::str::FromStr;

pub(crate) struct Test {
    test: Box<dyn Fn(usize) -> bool>,
}

impl FromStr for Test {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.trim_start().split(':').collect::<Vec<&str>>();

        if tokens.is_empty() {
            return Err(format!("`{}` yielded empty tokens", s));
        }

        if tokens[0] != "Test" {
            return Err(format!("`{}` is not a valid test1 command", s));
        }

        let tokens = tokens[1].trim_start().split(' ').collect::<Vec<&str>>();

        match tokens[0] {
            "divisible" => {
                if tokens[1] != "by" {
                    return Err(format!("Invalid `divisible by` test1 from `{}`", s));
                }

                let d = tokens[2].parse::<usize>().unwrap();
                return Ok(Self {
                    test: Box::new(move |val| val % d == 0),
                });
            }
            _ => Err(format!("Invalid test1 `{}`", s)),
        }
    }
}

impl Deref for Test {
    type Target = Box<dyn Fn(usize) -> bool>;

    fn deref(&self) -> &Self::Target {
        &self.test
    }
}

pub(crate) struct Action(pub(crate) Test, pub(crate) usize, pub(crate) usize);

impl Deref for Action {
    type Target = Test;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let test = Test::from_str("  Test: divisible by 13").unwrap();
        assert!(test(13));
    }

    #[test]
    fn parse_action() {
        let action = Action(Test::from_str("  Test: divisible by 13").unwrap(), 0, 2);
        assert!(action(13));
    }
}
