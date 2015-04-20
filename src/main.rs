extern crate time;
use time::*;

struct Exam {
    title: &'static str,
    datetime: &'static str
}

fn main() {
    let exams = [
        Exam{title: "EMPR REPORT", datetime: "29-04-2015 12:00"},
        Exam{title: "SYAC OS", datetime: "13-05-2015 18:00"},
        Exam{title: "POPL", datetime: "14-05-2015 18:00"},
        Exam{title: "ARIN", datetime: "15-05-2015 09:00"},
        Exam{title: "COCO", datetime: "26-05-2015 18:00"},
        Exam{title: "SYAC DB", datetime: "28-05-2015 13:30"},
    ];

    let localtime = time::now();

    for e in exams.iter() {
        let exam_time = time::strptime(e.datetime, "%d-%m-%Y %H:%M");
        match exam_time {
            Ok(time) => {
                let diff = time - localtime;
                println!("{} is in {} days", e.title, diff.num_days());
            }
            Err(err) => {
                println!("Error parsing datestring \"{}\": {}", e.datetime, err);
            }
        }
    }
}
