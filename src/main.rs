extern crate time;
use time::*;

#[derive(PartialEq)]
enum CourseType {
    CS,
    CSMATHS,
    BOTH
}

struct Exam {
    title: &'static str,
    datetime: &'static str,
    course: CourseType
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

fn print_exam(e: &Exam, diff: &Duration) {
    print!("{}", e.title);

    if e.course == CourseType::CSMATHS {
        print!(" (lol CS-Maths)");
    }

    if diff.num_minutes() < 0 {
        print!(" is done :)\n");
    } else {
        print!(" is in {}\n", mins_to_nice_string(diff.num_minutes()));
    }
}

fn main() {
    // TIMES IN UTC
    let exams = [
        Exam{title: "EMPR REPORT", datetime: "29-04-2015 11:00", course: CourseType::CS},
        Exam{title: "SYAC OS", datetime: "13-05-2015 17:00", course: CourseType::CS},
        Exam{title: "POPL", datetime: "14-05-2015 17:00", course: CourseType::BOTH},
        Exam{title: "ARIN", datetime: "15-05-2015 08:00", course: CourseType::BOTH},
        Exam{title: "Linear Algebra", datetime: "19-05-2015 08:00", course: CourseType::CSMATHS},
        Exam{title: "Real Analysis", datetime: "26-05-2015 08:00", course: CourseType::CSMATHS},
        Exam{title: "COCO", datetime: "26-05-2015 17:00", course: CourseType::BOTH},
        Exam{title: "SYAC DB", datetime: "28-05-2015 12:30", course: CourseType::CS},
    ];

    let localtime = time::now_utc();

    for e in exams.iter() {
        let exam_time = time::strptime(e.datetime, "%d-%m-%Y %H:%M");
        match exam_time {
            Ok(time) => {
                let diff = time - localtime;
                print_exam(&e, &diff);
            }
            Err(err) => {
                println!("Error parsing datestring \"{}\": {}", e.datetime, err);
            }
        }
    }
}
