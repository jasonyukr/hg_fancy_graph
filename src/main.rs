use std::io::{self, BufRead};

const STRIP_COLOR: bool = false;

enum State {
    Normal,
    Escape,
    Csi,
}

fn print_conv(graph_end: &mut bool, ch: char) {
    if *graph_end == false {
        *graph_end = ch.is_digit(16)
    }
    if *graph_end == true {
        print!("{}", ch);
        return;
    }

    let conv;
    match ch {
//        '|'  => conv = '\u{2502}',
        '/'  => conv = '\u{2571}',
        '\\' => conv = '\u{2572}',
        'o'  => conv = '\u{25cf}',
        '@'  => conv = '\u{25a0}',
//      '+'  => conv = '\u{253c}',
        '-'  => conv = '\u{2500}',
         _   => conv = ch
    }
    print!("{}", conv);
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line_data;
        match line {
            Ok(data) => line_data = data,
            Err(_) => continue
        }
        let mut state = State::Normal;
        let mut graph_end = false;
        for b in line_data.bytes() {
            match &state {
                State::Normal => {
                    if b == 0x1B { // ESC
                        state = State::Escape;
                        if !STRIP_COLOR {
                            print!("{}", b as char);
                        }
                    } else {
                        print_conv(&mut graph_end, b as char);
                    }
                },
                State::Escape => {
                    if !STRIP_COLOR {
                        print!("{}", b as char);
                    }
                    if b == 0x5B { // [
                        state = State::Csi;
                    } else {
                        state = State::Normal;
                    }
                },
                State::Csi => {
                    if !STRIP_COLOR {
                        print!("{}", b as char);
                    }
                    if b >= 0x40 && b < 0x80 {
                        state = State::Normal;
                    }
                },
            }
        }
        println!();
    }
}
