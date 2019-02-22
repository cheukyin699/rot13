use std::io::{self, Read};

fn rot13_c(c: char) -> char {
    match c {
        'A'...'M' | 'a'...'m' => ((c as u8) + 13) as char,
        'N'...'Z' | 'n'...'z' => ((c as u8) - 13) as char,
        _ => c,
    }
}

fn rot13(line: String) -> String {
    line.chars().map(rot13_c).collect()
}

fn main() {
    let mut all = String::new();
    match io::stdin().read_to_string(&mut all) {
        Ok(_) => println!("{}", rot13(all)),
        Err(e) => panic!("Error reading stdin: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::rot13;

    #[test]
    fn test_ascii() {
        assert_eq!(rot13("abcde".to_string()), "nopqr");
        assert_eq!(rot13("###!!".to_string()), "###!!");
        assert_eq!(rot13("# nOp".to_string()), "# aBc");
    }

    #[test]
    fn test_utf8() {
        assert_eq!(rot13("我的心".to_string()), "我的心");
        assert_eq!(rot13("# 今天".to_string()), "# 今天");
        assert_eq!(rot13("白sunn".to_string()), "白fhaa");
        assert_eq!(rot13("白Sunn".to_string()), "白Fhaa");
    }
}
