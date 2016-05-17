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


use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::io::Error;
use std::vec::Vec;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Index, IndexMut};

use rustc_serialize::json;

use timestamp::Timestamp;


#[derive(RustcDecodable, RustcEncodable)]
pub struct Task {
    created: Timestamp,
    deadline: Timestamp,
    completed: Timestamp,

    text: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct TaskList {
    tasks: Vec<Task>,
}

pub struct TaskFile {
    file: File,
}


impl Task {
    pub fn new() -> Task {
        let now = Timestamp::now();
        
        Task { 
            created: now,
            deadline: Timestamp::new(),
            completed: Timestamp::new(),
            text: "".to_string(),
        }
    }
    
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
    
    pub fn text(&self) -> &String {
        &self.text
    }
    
    pub fn set_completed(&mut self, ts: Timestamp) {
        if ts <= Timestamp::now() {
            self.completed = ts;
        }
    }
    
    pub fn is_completed(&self) -> bool {
        self.completed.valid() && self.completed <= self.deadline
    }
    
    pub fn deadline_missed(&self) -> bool {
        let ts = Timestamp::now();
        
        if self.completed.valid() {
            self.completed > self.deadline
        } else {
            self.deadline.valid() && ts > self.deadline
        }
    }
    
    pub fn set_deadline(&mut self, ts: Timestamp) {
        if ts >= Timestamp::now() {
            self.deadline = ts;
        }
    }
}


impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    
        let fmt : &str;
        let ts : &Timestamp;

        if self.is_completed() {
            fmt = "[x] : completed at    ";
            ts = &self.completed;
        } else if self.deadline_missed() {
            fmt = "[ ] : deadline missed ";
            ts = &self.deadline;
        } else {
            fmt = "[ ] : deadline        ";
            ts = &self.deadline;
        }
        
        write!(f, "{} -- {} -- \"{}\"", fmt, ts, self.text)
    }
}

impl TaskList {
    pub fn new() -> TaskList {
        TaskList { tasks: vec![] }
    }
    
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }
    
    pub fn remove(&mut self, i: usize) {
        self.tasks.remove(i);
    }
    
    pub fn remove_all(&mut self) {
        self.tasks.clear();
    }
    
    pub fn complete(&mut self, i: usize) {
        if !self.tasks[i].is_completed() {
            self.tasks[i].set_completed(Timestamp::now());
        }
    }
    
    pub fn complete_all(&mut self) {
        for i in 0..self.tasks.len() {
            self.complete(i);
        }
    }
    
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

impl Index<usize> for TaskList {
    type Output = Task;
    
    fn index<'a>(&'a self, i: usize) -> &'a Task {
        &self.tasks[i]
    }
}

impl IndexMut<usize> for TaskList {    
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Task {
        &mut self.tasks[i]
    }
}

impl Display for TaskList {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        static RED: &'static str = "\x1B[1;31m";
        static GREEN: &'static str =  "\x1B[1;32m";
        static YELLOW: &'static str = "\x1B[1;33m";
        static DEFAULT: &'static str = "\x1B[0m";
        
        for i in 0..self.tasks.len() {
            let task = &self.tasks[i];
            let color: &str;
            
            if task.is_completed() {
                color = GREEN;
            } else if task.deadline_missed() {
                color = RED;
            } else {
                color = YELLOW;
            }
        
            try!(writeln!(f, "{} {:2} : {}{}", color, i + 1, task, DEFAULT));
        }
        
        Ok(())
    }
}

impl TaskFile {
    pub fn new(path: &str) -> Result<TaskFile, Error> {
        let mut open_opts = OpenOptions::new();
        
        open_opts.read(true)
            .write(true)
            .create(true);
        
        let file = try!(open_opts.open(path));
        
        Ok(TaskFile { file: file })
    }
    
    pub fn load(&mut self) -> TaskList {
        let mut s = String::new();
        
        self.file.read_to_string(&mut s).unwrap();

        json::decode(&s).unwrap_or(TaskList::new())
    }
    
    pub fn save(&mut self, tasks: &TaskList) {
        json::encode(tasks).map(|x| {
            self.file.seek(SeekFrom::Start(0)).unwrap();
            self.file.set_len(0).unwrap();
            
            let data = x.into_bytes();
            self.file.write_all(&data).unwrap();
        }).unwrap();
    }
}


