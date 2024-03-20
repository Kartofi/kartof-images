use std::{fs, ptr::null};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Route {
    pub path: String,
    pub content_type: String,
    pub file: String,
}

pub struct Routes {
    pub routes: Vec<Route>,
}
impl Routes {
    pub fn init(&mut self, path: String) {
        let paths = fs::read_dir(path).unwrap();
        for dir in paths.into_iter() {
            let obj = dir.unwrap();

            if obj.path().to_str().unwrap().ends_with(".json") == true {
                let json = fs::read_to_string(obj.path()).unwrap();

                let route: Route = serde_json::from_str(&json).unwrap();
                self.routes.push(route);
            }
        }
    }

    pub fn get_file(&self, path: &str) -> Option<&Route> {
        for route in self.routes.iter() {
            if route.path == path {
                return Some(route);
            }
        }
        None
    }
}
