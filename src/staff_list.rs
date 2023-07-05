use std::collections::{HashMap, hash_map::Entry};

pub struct Business {
    staff_list: HashMap<String, Vec<String>>
}

impl Business {
    pub fn new() -> Business {
        Business {
            staff_list: HashMap::new()
        }
    }

    pub fn add_dept(&mut self, dept: &str) -> Result<(), String> {
        match self.staff_list.entry(dept.to_string()) {
            Entry::Vacant(list) => {list.insert(Vec::new()); Ok(())},
            Entry::Occupied(_) => Err(String::from("Department already exists"))
        }
        
    }

    pub fn add_employee(&mut self, dept: &str, name: &str) {
        match self.staff_list.entry(dept.to_string()) {
            // if dept does not exist, add dept as new key and push name to dept's vec.
            // Entry::Vacant(list) => list.insert(Vec::new()).push(name.to_string()),
            Entry::Vacant(list) => self.add_dept(dept).unwrap(),
            // if dept does exist, push name to dept's vec
            Entry::Occupied(mut list) => list.get_mut().push(name.to_string())
        };
    }

    pub fn print_list(&self) {
        println!("{:#?}", self.staff_list)
    }
}

