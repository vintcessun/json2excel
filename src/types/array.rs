use std::{
    fmt::Display,
    ops::{Add, AddAssign, Deref},
};

use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use rust_xlsxwriter::*;
use serde_json::Value;
const SPLIT: &str = "|";

pub fn array(sheet: &mut Worksheet, data: Value, tag: &str) -> Result<()> {
    let arr = match data.as_array() {
        Some(ret) => ret,
        None => return Err(anyhow!("the data might not a array")),
    };

    let first = remove_tag(&arr[0], tag)?;
    sheet.write(0, 0, tag)?;
    let mut writetitle = WriteTitle::new(sheet);
    writetitle.run(&first)?;

    for (i, per) in arr.iter().enumerate() {
        sheet.write((i + 1) as u32, 0, first_end_handle(per[tag].to_string()))?;
        let per = remove_tag(per, tag)?;
        let mut writedata = WriteData::new(sheet, (i + 1) as u32);
        writedata.run(&per)?;
    }
    Ok(())
}

fn remove_tag(data: &Value, tag: &str) -> Result<Value> {
    let mut json: IndexMap<String, Value> = serde_json::from_value(data.to_owned())?;
    json.swap_remove(tag);
    let ret = serde_json::to_value(json)?;
    Ok(ret)
}

struct WriteTitle<'a> {
    col: u16,
    sheet: &'a mut Worksheet,
    stack: Stack<String>,
}

impl<'a> WriteTitle<'a> {
    fn new(sheet: &'a mut Worksheet) -> Self {
        Self {
            col: 1,
            sheet,
            stack: Stack::new(),
        }
    }

    fn run(&mut self, data: &Value) -> Result<()> {
        if data.is_string() {
            let out = self.stack.show(SPLIT);
            //println!("{}", &out);
            self.sheet.write(0, self.col, out)?;
            self.col += 1;
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

struct WriteData<'a> {
    row: u32,
    col: u16,
    sheet: &'a mut Worksheet,
}

impl<'a> WriteData<'a> {
    fn new(sheet: &'a mut Worksheet, row: u32) -> Self {
        Self { col: 1, sheet, row }
    }

    fn run(&mut self, data: &Value) -> Result<()> {
        if data.is_string() {
            self.sheet
                .write(self.row, self.col, first_end_handle(data.to_string()))?;
            self.col += 1;
        } else if data.is_array() {
            if let Some(data) = data.as_array() {
                for per in data {
                    //let stack = self.stack.clone();
                    //self.stack.tail(i);
                    self.run(per)?;
                    //self.stack = stack;
                }
            }
        } else {
            let data: IndexMap<String, Value> = serde_json::from_value(data.to_owned())?;
            for (_, val) in data.iter() {
                self.run(val)?;
            }
        }
        Ok(())
    }
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

    fn push(&mut self, ele: T) {
        self.elements.push(ele);
    }

    fn pop(&mut self) -> Option<T> {
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
        ret
    }
}

pub trait ShowStack<T> {
    fn show(&self, split: &str) -> T;
}
