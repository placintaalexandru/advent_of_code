use std::str::FromStr;

pub(crate) enum Play {
    Add(usize),
    Sub(usize),
    Mul(usize),
    Pow(u32),
}

impl FromStr for Play {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.trim_start().split(':').collect::<Vec<&str>>();

        if tokens[0] != "Operation" {
            return Err(format!("String `{}` does not have first part right", s));
        }

        let tokens = tokens[1].trim_start().split(' ').collect::<Vec<&str>>();

        match tokens[3] {
            "+" => Ok(Self::Add(tokens[4].parse::<usize>().unwrap())),
            "-" => Ok(Self::Sub(tokens[4].parse::<usize>().unwrap())),
            "*" => match tokens[4] {
                "old" => Ok(Self::Pow(2)),
                _ => Ok(Self::Mul(tokens[4].parse::<usize>().unwrap())),
            },
            _ => Err(format!("Cannot parse any command from {}", s)),
        }
    }
}

impl Play {
    pub(crate) fn apply(&self, val: usize) -> usize {
        match *self {
            Self::Mul(m) => val * m as usize,
            Self::Pow(p) => usize::pow(val, p as u32),
            Self::Add(a) => val + a as usize,
            Self::Sub(s) => val - s as usize,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_mul() {
        let operation = Play::from_str("  Operation: new = old * 2").unwrap();
        assert_eq!(operation.apply(4), 8);
    }

    #[test]
    fn parse_pow() {
        let operation = Play::from_str("  Operation: new = old * old").unwrap();
        assert_eq!(operation.apply(3), 9);
    }

    #[test]
    fn parse_add() {
        let operation = Play::from_str("  Operation: new = old + 2").unwrap();
        assert_eq!(operation.apply(3), 5);
    }

    #[test]
    fn parse_sub() {
        let operation = Play::from_str("  Operation: new = old - 2").unwrap();
        assert_eq!(operation.apply(3), 1);
    }
}
