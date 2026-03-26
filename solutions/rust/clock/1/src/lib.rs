use std::fmt;

#[derive(Debug, PartialEq, Eq)]   // 自动实现 debug 和 partialEq 打印和比较
pub struct Clock{
    minutes: i32, // 统一存储为“从 00:00 开始的总分钟数”
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        Self{
            minutes: Self::normalize(hours * 60 + minutes)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time")
        Self::new(0, self.minutes + minutes)
    }

    fn normalize(minutes: i32)->i32{
        let day_minutes = 24 * 60;
        ((minutes % day_minutes) + day_minutes) % day_minutes
    }
}


impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        let h = self.minutes / 60;
        let m = self.minutes % 60;
        write!(f, "{:02}:{:02}", h, m)
    }
}