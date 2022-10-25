use std::time::{UNIX_EPOCH, SystemTime};
use chrono::{DateTime, Utc};
use ncurses::*;

fn main() {
    println!("Hello, world!");

    let mut now: std::time::Duration;
    let mut cur_time: DateTime<Utc>;

    initscr();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    loop {
        now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
        cur_time = Utc::now();

        let seconds: i64 = now.as_secs() as i64;
        let bytes = seconds.to_be_bytes();
        let mut byte_string: String = "".to_owned();
        for byte in bytes {
            byte_string.push_str(&format!("{:0>8b}", byte));
        }

        mvprintw(0, 0, "github.com/JoeyShapiro/Sunflower");
        mvprintw(max_y-1, 0, &cur_time.to_rfc2822());
        for (i, c) in byte_string.chars().enumerate() {
            let index = i32::try_from(i).unwrap();
            mvprintw(max_y/2+index/8, max_x/2+index%8, &c.to_string());
        }

        refresh();
    }

    // endwin();
}
