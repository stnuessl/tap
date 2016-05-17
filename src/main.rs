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
 
extern crate rustc_serialize;
extern crate time;

mod task;
mod timestamp;
mod config;
mod argparser;

use std::env;
use std::vec::Vec;
use std::process::exit;
use std::path::{PathBuf};
use std::usize;

use config::Config;
use timestamp::Timestamp;
use task::{Task, TaskFile};
use argparser::{ArgParser};

fn ignore_args(cmd: &str, args: &[String]) {
    if !args.is_empty() {
        print!("tap: {}: ignoring superfluous argument(s) -", cmd);
        
        for x in args.iter() {
            print!(" \"{}\"", x);
        }
        
        println!("");
    }

}

fn missing_arg(cmd: &str) {
    println!("tap: {}: missing argument(s)", cmd);
}

fn home_dir() -> PathBuf {
    env::home_dir().unwrap()
}

fn config_path() -> PathBuf {
    let mut path = home_dir();
    path.push(".config/tap/tap.conf");
    
    path
}

fn main() {
    let mut conf = Config::new(&config_path()).unwrap();
    let args : Vec<_> = env::args().collect();
    
    let mut parser = ArgParser::new();
    parser.add_opt("add");
    parser.add_opt("complete");
    parser.add_opt("file");
    parser.add_opt("remove");
    
    let unknown = parser.parse(&args.as_slice(), 1..args.len());
    if !unknown.is_empty() {
        for x in &unknown {
            println!("tap: unknown argument \"{}\"", x);
        }
        
        exit(1);
    }
    
    let mut filename = conf.task_file();
    let mut name_changed = false;
    
    let file_info = parser.get_arginfo("file").unwrap();
    if file_info.is_passed() {
        if !file_info.has_args() {
            missing_arg("file");
            exit(1);
        }
        
        let mut range = file_info.range();
        range.start += 1;
        if range.start < range.end {
            ignore_args("file", &args[range]);
        }
                
        filename = args[file_info.begin()].clone();
        
        /* delay writing to disk until we know we can actually use the file */
        name_changed = true;
    }
    
    let mut taskfile = match TaskFile::new(filename.as_ref()) {
        Ok(x) => x,
        Err(err) => { 
            println!("tap: failed to load \"{}\" - {}", filename, err);
            exit(1);
        }
    };
    
    let mut tasks = taskfile.load();
    
    if name_changed {
        conf.set_task_file(&filename).unwrap();
    }

    let add_info = parser.get_arginfo("add").unwrap();
    if add_info.is_passed() {
        if !add_info.has_args() {
            missing_arg("add");
            exit(1);
        }
        
        let mut i = add_info.begin();
        let j = add_info.end();
        
        let mut task = Task::new();
        task.set_text(args[i].as_ref());
        
        i += 1;
        
        if i < j {
            let ts = Timestamp::from_string(&args[i]);
            if ts.is_err() {
                let s = ts.unwrap_err();
                println!("tap: add: invalid time format \"{}\" - {}", 
                         args[i], s);
                exit(1);
            }
            
            task.set_deadline(ts.unwrap());
        }
        
        i += 1;
        
        if i < j {
            ignore_args("add", &args[i..j]);
        }
                    
        if task.text().is_empty() {
            println!("tap: add: missing task description");
            exit(1);
        }
        
        tasks.add(task);
    }

    let complete_info = parser.get_arginfo("complete").unwrap();
    if complete_info.is_passed() {

        for i in complete_info.range() {
            let arg = &args[i];
            
            match arg.as_ref() {
                "--all" => {
                    tasks.complete_all();
                    break;
                },
                _ => { 
                    let result = usize::from_str_radix(arg, 10);
                    if result.is_err() {
                        println!("tap: complete: invalid argument \"{}\"", arg);
                        exit(1);
                    }
                    
                    tasks.complete(result.unwrap() - 1);
                },
            }
        }
    }
    
    let remove_info = parser.get_arginfo("remove").unwrap();
    if remove_info.is_passed() {
        if !remove_info.has_args() {
            missing_arg("remove");
            exit(1);
        }
        
        let mut v = vec![];
        
        for i in remove_info.range() {
            let arg = &args[i];

            match arg.as_ref() {
                "--all" => {
                    tasks.remove_all();
                    break;
                }
                "--all-completed" => {
                    for i in 0..tasks.len() {
                        if tasks[i].is_completed() {
                            v.push(i);
                        }
                    }
                    
                    break;
                }
                _ => {
                    let result = usize::from_str_radix(arg, 10);
                    if result.is_err() {
                        println!("tap: remove: invalid argument \"{}\"", arg);
                        exit(1);
                    }
                    
                    v.push(result.unwrap());
                }
            }
        }
        
        if !v.is_empty() && tasks.len() > 0 {
            v.sort_by(|a, b| b.cmp(a));
            v.dedup();
            
            for x in v {
                tasks.remove(x);
            }
        }
    }
    
    print!("{}", tasks);
    
    taskfile.save(&tasks);
}
