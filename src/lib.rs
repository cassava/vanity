/*!
The vanity create provides functionality to convert alphanumerical strings
into vanity numbers.
*/

use std::iter::FromIterator;

/// Returns true if the string can be sensibly converted into a vanity string.
pub fn is_vain(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_alphanumeric() {
            return false;
        }
    }
    true
}

/// Returns a String that consists only of numbers.
///
/// Note that any non-convertible characters will be stripped.
pub fn to_vanity(s: &str) -> String {
    let mut vanity = vec![];
    for c in s.chars() {
        for k in c.to_uppercase() {
            let d = match k {
                '0'...'9' => c,
                'A'...'C' => '2',
                'D'...'F' => '3',
                'G'...'I' => '4',
                'J'...'L' => '5',
                'M'...'O' => '6',
                'P'...'S' => '7',
                'T'...'V' => '8',
                'W'...'Z' => '9',
                _ => if k.is_alphanumeric() { '1' } else { '0' },
            };
            if d == '0' {
                continue;
            }
            vanity.push(d);
        }
    }
    String::from_iter(vanity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vain_strings() {
        let v = vec![
            ("vanity", true, "826489"),
            ("not vain", false, "6688246"),
            ("heiß", true, "43477"),
        ];
        for t in v {
            assert_eq!(is_vain(t.0), t.1);
            assert_eq!(to_vanity(t.0), t.2);
        }
    }
}
