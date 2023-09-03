use std::{thread, time};

use rcm;

extern crate termion;

struct TestMenuItem {
    n: u32,
}

impl TestMenuItem {
    fn new() -> Self {
        Self {
            n: 0,
        }
    }
}

impl rcm::MenuFunc for TestMenuItem {
    fn call(&mut self) {
        self.n = self.n + 1;
    }

    fn get_title(&self) -> String { 
        format!("n: {}", self.n)
    }
}

fn main() {
    let test_menu_1 = rcm::MenuBuilder::empty()
        .push_func(Box::new(TestMenuItem::new()))
        .build();
    test_menu_1.run();
}
