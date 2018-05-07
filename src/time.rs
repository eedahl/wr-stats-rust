extern crate regex;

#[derive(Clone)]
pub struct Time {
    minutes: i32,
    seconds: i32,
    milliseconds: i32,
}

pub fn compare(t1: &Time, t2: &Time) -> bool {
    t1.minutes * 60 * 100 + t1.seconds * 100 + t1.milliseconds
        <= t2.minutes * 60 * 100 + t2.seconds * 100 + t2.milliseconds
}

pub fn to_string(t: &Time) -> String {
    format!(
        "{min:02}:{sec:02},{ms:02}",
        min = t.minutes,
        sec = t.seconds,
        ms = t.milliseconds
    )
}

pub fn from_string(st: &String) -> Time {
    let re = regex::Regex::new(r":|,").unwrap();
    let t: Vec<&str> = re.split(&st).collect();
    let (m, s, ms) = if t.len() == 3 {
        (
            t[0].parse::<i32>().unwrap(),
            t[1].parse::<i32>().unwrap(),
            t[2].parse::<i32>().unwrap(),
        )
    } else {
        (
            0,
            t[0].parse::<i32>().unwrap(),
            t[1].parse::<i32>().unwrap(),
        )
    };

    Time {
        minutes: m,
        seconds: s,
        milliseconds: ms,
    }
}

pub fn difference(t1: &Time, t2: &Time) -> Time {
    let mut ms = t1.milliseconds - t2.milliseconds;
    let mut s;
    let m;

    if ms < 0 {
        s = t1.seconds - t2.seconds - 1;
        ms = ms + 100;
    } else {
        s = t1.seconds - t2.seconds;
    }

    if s < 0 {
        m = t1.minutes - t2.minutes - 1;
        s = 60 + s;
    } else {
        m = t1.minutes - t2.minutes;
    }

    Time {
        minutes: m,
        seconds: s,
        milliseconds: ms,
    }
}

pub fn add(t1: Time, t2: Time) -> Time {
    let mut ms = t1.milliseconds + t2.milliseconds;
    let mut s;
    let m;

    if ms > 99 {
        s = t1.seconds + t2.seconds + 1;
        ms = ms - 100;
    } else {
        s = t1.seconds + t2.seconds;
    }

    if s > 60 {
        m = t1.minutes + t2.minutes + 1;
        s = s - 60;
    } else {
        m = t1.minutes + t2.minutes;
    }

    Time {
        minutes: m,
        seconds: s,
        milliseconds: ms,
    }
}
