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

use std::fs::{File, OpenOptions, DirBuilder};
use std::io::{Error, Read, Write, Seek, SeekFrom};
use std::path::PathBuf;
 
pub struct Config {
    file: File,
}

fn trim(s: &mut String) {
    let mut i = 0;
    
    while s.starts_with(" ") || s.starts_with("\t") || s.starts_with("\n") {
        i += 1;
    }
    
    s.drain(0..i);
    
    while s.ends_with(" ") || s.ends_with("\t") || s.ends_with("\n") {
        s.pop().unwrap();
    }
}

impl Config {

    
    pub fn new(path: &PathBuf) -> Result<Config, Error> {
        let dir = path.parent().unwrap();
        
        match DirBuilder::new().recursive(true).create(dir) {
            Ok(_) => {
                let mut open_opts = OpenOptions::new();
                
                open_opts.read(true)
                    .write(true)
                    .create(true);
                
                let file = try!(open_opts.open(path));
                
                Ok(Config { file: file })
            }
            Err(err) => {
                Err(err)
            }
        }
    }
    
    pub fn task_file(&mut self) -> String {
        let mut s = String::new();
        
        self.file.read_to_string(&mut s).unwrap();
        
        trim(&mut s);
        
        s
    }
    
    pub fn set_task_file(&mut self, name: &String) -> Result<(), Error> {
        let mut s = name.clone();
        
        trim(&mut s);
        
        try!(self.file.seek(SeekFrom::Start(0)));
        try!(self.file.set_len(0));
        try!(self.file.write_all(&s.into_bytes()));
        
        Ok(())
    }
}
