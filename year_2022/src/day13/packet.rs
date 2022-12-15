use serde_json::Value;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Packet(Value);

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content = serde_json::from_str::<Value>(s);

        match content {
            Ok(v) => Ok(Self(v)),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet(Value::Number(n1)), Packet(Value::Number(n2))) => {
                let v1 = n1.as_i64().unwrap();
                let v2 = n2.as_i64().unwrap();

                v1.partial_cmp(&v2)
            }
            (Packet(Value::Number(v1)), Packet(Value::Array(_))) => {
                Packet(Value::Array(vec![Value::Number(v1.clone())])).partial_cmp(other)
            }
            (Packet(Value::Array(_)), Packet(Value::Number(_))) => {
                other.partial_cmp(self).map(Ordering::reverse)
            }
            (Packet(Value::Array(a1)), Packet(Value::Array(a2))) => {
                for (v1, v2) in a1.iter().zip(a2.iter()) {
                    if let Some(order) = Packet(v1.clone()).partial_cmp(&Packet(v2.clone())) {
                        if matches!(order, Ordering::Less | Ordering::Greater) {
                            return Some(order);
                        }
                    }
                }

                if a2.len() < a1.len() {
                    return Some(Ordering::Greater);
                }

                if a1.len() < a2.len() {
                    return Some(Ordering::Less);
                }

                return Some(Ordering::Equal);
            }
            _ => panic!("Branch {:?} {:?} not covered", self, other),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        Packet::from_str(
            r#"
            [[6,4,[[10,5,7],3,2],[0,[10,1,7],[9,1,8,8,2]],10],[[],3,3,[7,7,[1]],2],
            [10,[0,[0,10,0,6,8],[],[9,8,2],[]],4,6],[3,[[4,9,5],10,[],0],5],[7,6,[5],
            [[3,5],0],[[9,8,4,7,4],[5,1,5,10]]]]
            "#,
        )
        .unwrap();
    }

    #[test]
    fn cmp_number_number() {
        let p1 = Packet::from_str("-12").unwrap();
        let p2 = Packet::from_str("2").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn cmp_simple_list_equal() {
        let p1 = Packet::from_str("[1,2,3]").unwrap();
        let p2 = Packet::from_str("[1,2,3]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Equal));
    }

    #[test]
    fn cmp_simple_list_less() {
        let p1 = Packet::from_str("[1,1,3,1,1]").unwrap();
        let p2 = Packet::from_str("[1,1,5,1,1]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn cmp_simple_list_greater() {
        let p1 = Packet::from_str("[1,1,5,1,1]").unwrap();
        let p2 = Packet::from_str("[1,1,3,1,1]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_simple_list_left_is_drained() {
        let p1 = Packet::from_str("[1,1]").unwrap();
        let p2 = Packet::from_str("[1,1,1]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn cmp_simple_list_right_is_drained() {
        let p1 = Packet::from_str("[1,1,1]").unwrap();
        let p2 = Packet::from_str("[1,1]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_list_number_less() {
        let p1 = Packet::from_str("[2,3,4]").unwrap();
        let p2 = Packet::from_str("4").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn cmp_list_number_greater() {
        let p1 = Packet::from_str("[5,3,4]").unwrap();
        let p2 = Packet::from_str("4").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_number_list_less() {
        let p1 = Packet::from_str("4").unwrap();
        let p2 = Packet::from_str("[5,3,4]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn cmp_number_list_greater() {
        let p1 = Packet::from_str("6").unwrap();
        let p2 = Packet::from_str("[5,3,4]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_empty_list_number_greater() {
        let p1 = Packet::from_str("6").unwrap();
        let p2 = Packet::from_str("[]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
    }

    #[test]
    fn cmp_empty_list_number_less() {
        let p1 = Packet::from_str("[]").unwrap();
        let p2 = Packet::from_str("6").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn test_nested() {
        let p1 = Packet::from_str("[[1],[2,3,4]]").unwrap();
        let p2 = Packet::from_str("[[1],4]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Less));
    }

    #[test]
    fn complex_example1() {
        let p1 = Packet::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        let p2 = Packet::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
    }

    #[test]
    fn complex_example2() {
        let p1 = Packet::from_str("[[[]]]").unwrap();
        let p2 = Packet::from_str("[[]]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
    }

    #[test]
    fn complex_example3() {
        let p1 = Packet::from_str("[[10,[[10,2],2],3,2],[5,7,[2,[9,4],0,[4,7],0]],[[6,6,10,6],[[7,5],[1,2,9,9,8],4,[9,7]],6],[8,[],8,8,[[4,6,7,7]]],[5,[7],[3,8],[0]]]").unwrap();
        let p2 = Packet::from_str("[[[6,1,[]]],[8,[[6,10,0,4],2,10,[0,3,10,4],9],7,7]]").unwrap();
        assert_eq!(p1.partial_cmp(&p2), Some(Ordering::Greater));
    }
}
