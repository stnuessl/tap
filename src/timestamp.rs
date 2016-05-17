/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2016 Steffen Nuessle
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
 
use std::fmt;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::ops::{Add, Sub};
use std::i64;

use time;

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Timestamp {
    seconds: i64,
}

impl Timestamp {
    fn from_absolute_time(s: &String) -> Result<Timestamp, String> {
        enum State { YEAR, MONTH, DAY, HOUR, MINUTE, SECOND, DONE }
        
        if s.is_empty() {
            return Err(format!("empty input string"));
        }
            
        let mut tm = time::now();
        tm.tm_mon = 0;
        tm.tm_mday = 1;
        tm.tm_hour = 0;
        tm.tm_min = 0;
        tm.tm_sec = 0;
        tm.tm_isdst = 1;
        
        let mut cur = 0;
        let mut state = State::YEAR;
        
        for x in s.chars() {
            if x.is_digit(10) {
                cur = cur * 10 + x.to_digit(10).unwrap() as i32;

                match state {
                    State::YEAR => tm.tm_year = cur - 1900,
                    State::MONTH => tm.tm_mon = cur - 1,
                    State::DAY => tm.tm_mday = cur,
                    State::HOUR => tm.tm_hour = cur,
                    State::MINUTE => tm.tm_min = cur,
                    State::SECOND => tm.tm_sec = cur,
                    State::DONE => break,
                }
                
            } else if x == '-' || x == '/' || x == ' ' || x == ':' {
                state = match state {
                    State::YEAR => State::MONTH,
                    State::MONTH => State::DAY,
                    State::DAY => State::HOUR,
                    State::HOUR => State::MINUTE,
                    State::MINUTE => State::SECOND,
                    State::SECOND => State::DONE,
                    State::DONE => break,
                };
                cur = 0;
            } else {
                return Err(format!("invalid character {}", x));
            }
        }
        
        Ok(Timestamp::from_tm(&tm))
    }
    
    fn from_relative_time(s: &String) -> Result<Timestamp, String> {
        let mut num = 0 as i64;
        let mut offset = 0 as i64;
        
        for x in s.chars() {
            if x.is_digit(10) {
                num = num * 10 + x.to_digit(10).unwrap() as i64;
            } else {
                match x {
                    'y' => offset += num * 3600 * 24 * 365,
                    'm' => offset += num * 3600 * 24 * 30,
                    'd' => offset += num * 3600 * 24,
                    'h' => offset += num * 3600,
                    's' => offset += num,
                    _ => { 
                        return Err(format!("invalid time specifier {}", x));
                    }
                }
                num = 0;
            }
        }
    
        Ok(Timestamp::now() + offset)
    }
    
    pub fn new() -> Timestamp {
        Timestamp { seconds: i64::MAX }
    }
    
    pub fn from_seconds(seconds: i64) -> Timestamp {
        Timestamp { seconds: seconds }
    }
    
    pub fn from_timespec(ts: &time::Timespec) -> Timestamp {
        Timestamp::from_seconds(ts.sec)
    }
    
    pub fn from_tm(tm: &time::Tm) -> Timestamp {
        Timestamp::from_timespec(&tm.to_timespec())
    }
    
    pub fn from_string(s: &String) -> Result<Timestamp, String> {
        if s.chars().all(|x| !x.is_alphabetic()) {
            Timestamp::from_absolute_time(s)
        } else {
            Timestamp::from_relative_time(s)
        }
    }
    
    pub fn now() -> Timestamp {
        Timestamp::from_timespec(&time::get_time())
    }
    
    pub fn valid(&self) -> bool {
        self.seconds != i64::MAX
    }
    
//     pub fn seconds(&self) -> i64 {
//         self.seconds
//     }
    
    fn to_timespec(&self) -> time::Timespec {
        time::Timespec::new(self.seconds, 0)
    }
}

impl Add<i64> for Timestamp {
    type Output = Timestamp;
    
    fn add(mut self, sec: i64) -> Timestamp {
        self.seconds += sec;
        self
    }
}

impl Sub<i64> for Timestamp {
    type Output = Timestamp;
    
    fn sub(mut self, sec: i64) -> Timestamp {
        self.seconds -= sec;
        self
    }
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Timestamp) -> Ordering {
        if self.seconds < other.seconds {
            Ordering::Less
        } else if self.seconds > other.seconds {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Timestamp) -> bool {
        self.seconds == other.seconds
    }
    
    fn ne(&self, other: &Timestamp) -> bool {
        self.seconds != other.seconds
    }
}

impl Eq for Timestamp {
    /* nice trait */
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Timestamp) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    
    fn lt(&self, other: &Timestamp) -> bool {
        self.seconds < other.seconds
    }
    
    fn le(&self, other: &Timestamp) -> bool {
        self.seconds <= other.seconds
    }
    
    fn gt(&self, other: &Timestamp) -> bool {
        self.seconds > other.seconds
    }
    
    fn ge(&self, other: &Timestamp) -> bool {
        self.seconds >= other.seconds
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.valid() {
            let ts = self.to_timespec();
            let tm = time::at(ts);
            
            let year = 1900 + tm.tm_year;
            let month = 1 + tm.tm_mon;
            let day = tm.tm_mday;
            let hour = tm.tm_hour;
            let min = tm.tm_min;
            let sec = tm.tm_sec;
        
            write!(f, "{}-{:02}-{:02} :: {:02}:{:02}:{:02}",
                year, month, day, hour, min, sec)
        } else {
            write!(f, "unspecified           ")
        }
    }
}
