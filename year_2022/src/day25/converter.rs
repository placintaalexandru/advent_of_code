pub(crate) struct Converter;

impl Converter {
    fn snafu_char_to_i32(c: char) -> i64 {
        match c {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        }
    }

    pub(crate) fn snafu_to_base10(snafu: &str, base: i64) -> i64 {
        snafu
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| base.pow(i as u32) * Self::snafu_char_to_i32(c))
            .sum()
    }

    pub(crate) fn base10_to_snafu(n: i64, base: i64) -> String {
        let mut buffer = vec![];
        let mut n = n;

        loop {
            let q = n / base;
            let r = n - base * q;

            buffer.push(r);

            n = q;

            if q == 0 {
                break;
            }
        }

        let mut result = String::default();
        let mut carry = 0;

        buffer.iter().for_each(|c| {
            let mut r = (*c as u8 + carry);
            carry = (r > 2) as u8;

            r %= 5;

            if r == 3 {
                result.push('=');
            } else if r == 4 {
                result.push('-');
            } else {
                result.push((r + '0' as u8) as char);
            }
        });

        if carry != 0 {
            result.push((carry + '0' as u8) as char);
        }

        result.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert1() {
        assert_eq!(Converter::snafu_to_base10("1=-0-2", 5), 1747);
    }

    #[test]
    fn convert2() {
        assert_eq!(Converter::snafu_to_base10("20012", 5), 1257);
    }
}
