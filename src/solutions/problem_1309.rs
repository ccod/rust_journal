pub fn freq_alphabets(s: String) -> String {
    let as_chars: Vec<char> = s.chars().collect();
    let mut idx: usize = 0;
    let mut result = String::new();

    while idx < as_chars.len() {
        if idx + 2 < as_chars.len() && as_chars[idx + 2] == '#' {
            let num: String = as_chars[idx..idx + 2].iter().collect();
            match &num[..] {
                "10" => result.push('j'),
                "11" => result.push('k'),
                "12" => result.push('l'),
                "13" => result.push('m'),
                "14" => result.push('n'),
                "15" => result.push('o'),
                "16" => result.push('p'),
                "17" => result.push('q'),
                "18" => result.push('r'),
                "19" => result.push('s'),
                "20" => result.push('t'),
                "21" => result.push('u'),
                "22" => result.push('v'),
                "23" => result.push('w'),
                "24" => result.push('x'),
                "25" => result.push('y'),
                "26" => result.push('z'),
                _ => (),
            }
            idx += 3
        } else {
            match as_chars[idx] {
                '1' => result.push('a'),
                '2' => result.push('b'),
                '3' => result.push('c'),
                '4' => result.push('d'),
                '5' => result.push('e'),
                '6' => result.push('f'),
                '7' => result.push('g'),
                '8' => result.push('h'),
                '9' => result.push('i'),
                _ => (),
            }
            idx += 1
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_freq_alphabets() {
        assert_eq!(freq_alphabets("10#11#12".to_string()), "jkab".to_string());
        assert_eq!(freq_alphabets("".to_string()), "".to_string());
        assert_eq!(freq_alphabets("1326#".to_string()), "acz".to_string());
        assert_eq!(freq_alphabets("25#".to_string()), "y");
        assert_eq!(freq_alphabets("12345678910#11#12#13#14#15#16#17#18#19#20#21#22#23#24#25#26#".to_string()), "abcdefghijklmnopqrstuvwxyz".to_string());
    }
}
