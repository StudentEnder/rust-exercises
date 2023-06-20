use std::collections::{HashMap, hash_map::Entry};

pub struct Business {
    dept_list: Vec<String>,
    staff_list: HashMap<String, Vec<String>>
}

impl Business {
    pub fn new() -> Business {
        Business {
            dept_list: Vec::new(),
            staff_list: HashMap::new()
        }
    }

    pub fn add_dept(&mut self, dept: &str) {
        self.staff_list.entry(dept.to_string())
        .or_insert(Vec::new());
    }

    pub fn add_employee(&mut self, dept: &str, name: &str) {
        match self.staff_list.entry(dept.to_string()) {
            Entry::Vacant(list) => list.insert(Vec::new()).push(name.to_string()),
            Entry::Occupied(mut list) => list.get_mut().push(name.to_string())
        };
    }

    pub fn print_list(&self) {
        println!("{:#?}", self.staff_list)
    }
}

