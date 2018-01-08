#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Clock {
    hours: isize,
    minutes: isize
}

impl Clock {
    pub fn new(hours: isize, minutes: isize) -> Self {
        let (hours, minutes) = normalize_time((hours, minutes));

        Clock { hours, minutes }
    }

    pub fn to_string(&self) -> String {
        let hours = format_num(self.hours);
        let minutes = format_num(self.minutes);

        format!("{}:{}", hours, minutes)
    }

    pub fn add_minutes(&self, minutes: isize) -> Self {
        let (hours, minutes) = normalize_time((self.hours, self.minutes + minutes));

        Clock { hours, minutes }
    }
}

fn normalize_time((mut hours, mut minutes): (isize, isize)) -> (isize, isize) {
    while hours < 0 {
        hours += 24;
    }

    let mut extra_hours = 0;
    while minutes < 0 {
        minutes += 60;
        extra_hours += 1;
    }
    hours = (hours - extra_hours + minutes / 60) % 24;
    minutes = minutes % 60;

    if hours < 0 {
        normalize_time((hours, minutes))
    } else {
        (hours, minutes)
    }
}

fn format_num(num: isize) -> String {
    match num / 10 {
        0 => format!("0{}", num),
        _ => format!("{}", num)
    }
}
