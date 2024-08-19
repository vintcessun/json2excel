use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use rust_xlsxwriter::*;
use serde_json::Value;
use std::fmt::Display;
const SPLIT: &str = "|";

pub fn array(sheet: &mut Worksheet, data: Value, tag: &str) -> Result<()> {
    let arr = match data.as_array() {
        Some(ret) => ret,
        None => return Err(anyhow!("the data might not a array")),
    };

    let mut out = Vec::with_capacity(arr.len());
    for per in arr {
        let mut flatten = Flatten::new();
        flatten.run(per)?;
        out.push(flatten.output);
    }

    let mut max_keys = Vec::new();
    for per in &out {
        for key in per.keys() {
            if key != tag && !max_keys.contains(key) {
                max_keys.push(key.to_owned())
            }
        }
    }

    //println!("{:?}", &max_keys);

    sheet.write(0, 0, tag)?;
    let mut max_keys_map = IndexMap::new();
    for (i, key) in max_keys.iter().enumerate() {
        let i = (i + 1) as u16;
        sheet.write(0, i, key.as_str())?;
        max_keys_map.insert(key, i);
    }

    //println!("{:?}", &max_keys_map);

    for (i, per) in out.iter().enumerate() {
        for key in per.keys() {
            let pos = match max_keys_map.get(&key) {
                Some(ret) => *ret,
                None => 0,
            };
            let data = match per.get(key) {
                Some(ret) => ret.to_owned(),
                None => "".to_string(),
            };
            sheet.write(i as u32, pos, data)?;
        }
    }
    Ok(())
}

pub fn first_end_handle(mut val: String) -> String {
    if !val.is_empty() {
        if val.starts_with('"') {
            val = val[1..].to_string();
        }
        if val.ends_with('"') {
            val.pop();
        }
    }
    val
}

#[derive(Debug, Clone, Default)]
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, ele: T) {
        self.elements.push(ele);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
}

impl Stack<String> {
    pub fn tail<T>(&mut self, tag: T)
    where
        T: Display,
    {
        let last = self.pop().unwrap_or_default();
        self.push(format!("{}{}", last, tag));
    }
}

impl ShowStack<String> for Stack<String> {
    fn show(&self, split: &str) -> String {
        let mut ret = "".to_string();
        self.elements.iter().for_each(|x| {
            let f = format!("{}{}", x, &split);
            ret += &f;
        });
        ret.trim_end_matches(split).to_string()
    }
}

pub trait ShowStack<T> {
    fn show(&self, split: &str) -> T;
}

#[derive(Debug, Clone, Default)]
pub struct Flatten {
    stack: Stack<String>,
    output: IndexMap<String, String>,
}

impl Flatten {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(),
            output: IndexMap::new(),
        }
    }

    pub fn run(&mut self, data: &Value) -> Result<()> {
        if data.is_string() {
            let out = self.stack.show(SPLIT);
            //println!("{}", &out);
            self.output.insert(out, first_end_handle(data.to_string()));
        } else if data.is_array() {
            if let Some(data) = data.as_array() {
                for (i, per) in data.iter().enumerate() {
                    let stack = self.stack.clone();
                    self.stack.tail(i);
                    self.run(per)?;
                    self.stack = stack;
                }
            }
        } else {
            let data: IndexMap<String, Value> = serde_json::from_value(data.to_owned())?;
            for (head, val) in data.iter() {
                self.stack.push(head.clone());
                self.run(val)?;
                self.stack.pop();
            }
        }
        Ok(())
    }
}
