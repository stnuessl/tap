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

use std::ops::Range;
use std::collections::HashMap;
use std::cmp::min;
 
#[allow(dead_code)]
pub struct ArgInfo<'a> {
   name: &'a str,
   begin: usize,
   end: usize,
}
 
pub struct ArgParser<'a> {
   map: HashMap<&'a str, ArgInfo<'a>>,
}
 
impl<'a> ArgInfo<'a> {
   pub fn new(name: &'a str) -> ArgInfo<'a> {
       ArgInfo { 
           name: name, 
           begin: 0,
           end: 0,
       }
   }
   
   #[allow(dead_code)]
   pub fn name(&self) -> &'a str {
       self.name
   }
    
   pub fn has_args(&self) -> bool {
       self.end > self.begin
   }
    
   pub fn set_begin(&mut self, begin: usize) {
       self.begin = begin;
   }
    
   pub fn begin(&self) -> usize {
       self.begin
   }

   pub fn set_end(&mut self, end: usize) {
       self.end = end;
   }
      
   pub fn end(&self) -> usize {
       self.end
   }
   
   pub fn range(&self) -> Range<usize> {
       self.begin..self.end
   }

   pub fn is_passed(&self) -> bool {
       self.begin != 0
   }
}
 
impl<'a> ArgParser<'a> {
   pub fn new() -> ArgParser<'a> {
       ArgParser { map: HashMap::new() }
   }
    
   pub fn parse(&mut self, args: &'a [String], r: Range<usize>) -> Vec<String> {
       let mut unknown = vec![];
       let mut i = r.start;
       let len = min(args.len(), r.end);
       
       while i < len {
           let arg = args[i].as_ref();
           
           self.map.remove(arg).map(|mut x| {
               i += 1;

               x.set_begin(i);
               
               while i < len {
                    if self.map.contains_key(AsRef::<str>::as_ref(&args[i])) {
                        break;
                    }
                    i += 1;
               }
               
               x.set_end(i);
                
               self.map.insert(arg, x);
           }).or_else(|| {
               unknown.push(arg.to_string());
               i += 1;
               None
           });
       }
       
       unknown
   }
    
   pub fn add_opt(&mut self, name: &'a str) {
       self.map.insert(name, ArgInfo::new(&name));
   }
   
   pub fn get_arginfo(&self, name: &'a str) -> Option<&ArgInfo> {
       self.map.get(name)
   }
}
