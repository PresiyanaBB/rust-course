pub struct FibIter {
    first: u32,
    second: u32,
    count: u32,
}

impl FibIter {
    pub fn new() -> FibIter {
        return FibIter {
            first: 1,
            second: 1,
            count: 1,
        };
    }

    pub fn next(&mut self) -> u32 {
        if self.count == 1 {
            self.count += 1;
            return self.first;
        }
        if self.count == 2 {
            self.count += 1;
            return self.second;
        }

        let next = self.first + self.second;
        self.first = self.second;
        self.second = next;
        self.count += 1;
        return next;
    }

    pub fn get_nth_number(n: u32) -> u32 {
        let mut iter = FibIter::new();
        for _ in 1..n {
            iter.next();
        }
        return iter.second;
    }

    pub fn get_nth_sum(n: u32) -> u32 {
        let mut iter = FibIter::new();
        let mut result = 0;
        for _ in 0..n {
            result += iter.next();
        }
        return result;
    }

    pub fn rev(self) -> RevFibIter {
        return RevFibIter {
            first: self.second,
            second: self.first,
            count: self.count,
            start_position: self.count,
        };
    }
}

pub fn fib_split(text: &str) -> Vec<String> {
    let mut fib_iter = FibIter::new();
    let mut result: Vec<String> = Vec::new();
    let mut fib_n;
    let mut round = 1;
    let mut text_iter = text.chars();
    let text_size = text.chars().count();
    let mut str = String::from("");
    loop {
        if text_size > FibIter::get_nth_sum(round) as usize {
            round += 1;
            fib_n = fib_iter.next();
            for _ in 0..fib_n {
                let item = text_iter.next();
                if item.is_none() {
                    break;
                }
                str.push_str(&item.unwrap().to_string());
            }
            result.push(str);
            str = String::from("");
        } else {
            while let Some(item) = text_iter.next() {
                let ch_item = item.to_string();
                str.push_str(&ch_item);
            }
            result.push(str);
            break;
        }
    }

    return result;
}

pub fn fib_split_n(text: &str, n: u32) -> (Vec<String>, &str) {
    let mut fib_iter = FibIter::new();
    let mut result: (Vec<String>, &str) = (Vec::new(), "");
    let mut fib_n;
    let mut text_iter = text.chars();
    let text_size = text.chars().count();
    let mut str = String::from("");
    let mut modified_chars = 0;

    if text_size < FibIter::get_nth_sum(n) as usize {
        panic!("Text is too small");
    }

    for _ in 0..n {
        fib_n = fib_iter.next();
        for _ in 0..fib_n {
            let item = text_iter.next();
            if item.is_none() {
                break;
            }
            str.push_str(&item.unwrap().to_string());
        }
        modified_chars += fib_n as usize;
        result.0.push(str);
        str = String::from("");
    }

    result.1 = text
        .char_indices()
        .nth(modified_chars)
        .map(|(byte_index, _)| &text[byte_index..])
        .unwrap_or("");

    return result;
}

pub struct RevFibIter {
    first: u32,
    second: u32,
    count: u32,
    start_position: u32,
}

impl RevFibIter {
    pub fn next(&mut self) -> Option<u32> {
        if self.start_position == 1 {
            return None;
        } else if self.start_position == self.count {
            self.start_position -= 1;
            return Some(self.first);
        } else if self.start_position == self.count - 1 {
            self.start_position -= 1;
            return Some(self.second);
        }

        let next = self.first - self.second;
        self.first = self.second;
        self.second = next;
        self.start_position -= 1;
        return Some(next);
    }
}

pub fn fib_split_n_symmetric(text: &str, n: u32) -> (Vec<String>, &str) {
    let mut fib_iter = FibIter::new();
    let mut result: (Vec<String>, &str) = (Vec::new(), "");
    let mut fib_n;
    let mut text_iter = text.chars();
    let text_size = text.chars().count();
    let mut str = String::from("");
    let mut modified_chars = 0;

    if text_size < (FibIter::get_nth_sum(n) as usize * 2) as usize {
        panic!("Text is too small");
    }

    for _ in 0..n {
        fib_n = fib_iter.next();
        for _ in 0..fib_n {
            let item = text_iter.next();
            if item.is_none() {
                break;
            }
            str.push_str(&item.unwrap().to_string());
        }
        modified_chars += fib_n as usize;
        result.0.push(str);
        str = String::from("");
    }

    let mut rev_fib_iter: RevFibIter = fib_iter.rev();

    for _ in 0..n {
        fib_n = rev_fib_iter.next().unwrap();
        for _ in 0..fib_n {
            let item = text_iter.next();
            if item.is_none() {
                break;
            }
            str.push_str(&item.unwrap().to_string());
        }
        modified_chars += fib_n as usize;
        result.0.push(str);
        str = String::from("");
    }

    result.1 = text
        .char_indices()
        .nth(modified_chars)
        .map(|(byte_index, _)| &text[byte_index..])
        .unwrap_or("");

    return result;
}
