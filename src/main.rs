extern crate time;
use time::*;

struct Exam {
    title: &'static str,
    datetime: &'static str
}

fn mins_to_nice_string(mins_in: i64) -> String {
    let days: i64 = (mins_in / 60) / 24;
    let hours: i64 = (mins_in / 60) - (days * 24);
    let minutes: i64 = mins_in - ((days * 24 * 60) + (hours * 60));
    format!("{} days, {} hours, and {} minutes",
            days,
            hours,
            minutes)
}

fn main() {
    // TIMES IN UTC
    let exams = [
        Exam{title: "EMPR REPORT", datetime: "29-04-2015 11:00"},
        Exam{title: "SYAC OS", datetime: "13-05-2015 17:00"},
        Exam{title: "POPL", datetime: "14-05-2015 17:00"},
        Exam{title: "ARIN", datetime: "15-05-2015 08:00"},
        Exam{title: "COCO", datetime: "26-05-2015 17:00"},
        Exam{title: "SYAC DB", datetime: "28-05-2015 12:30"},
    ];

    let localtime = time::now_utc();

    for e in exams.iter() {
        let exam_time = time::strptime(e.datetime, "%d-%m-%Y %H:%M");
        match exam_time {
            Ok(time) => {
                let diff = time - localtime;
                println!("{} is in {}",
                         e.title,
                         mins_to_nice_string(diff.num_minutes()));
            }
            Err(err) => {
                println!("Error parsing datestring \"{}\": {}", e.datetime, err);
            }
        }
    }
}
