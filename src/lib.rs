pub enum MenuItem {
    Menu(Menu),
    Func(Box<dyn MenuFunc>),
}

pub trait MenuFunc {
    fn call(&mut self);
    fn get_title(&self) -> String;
}

pub struct Menu {
    elements: Vec<MenuItem>,
    title: String,
    selected: u32,
}

pub struct MenuBuilder {
    elements: Vec<MenuItem>,
    title: String,
}

impl Menu {
    pub fn builder() -> MenuBuilder {
        MenuBuilder::empty()
    }

    pub fn print_title(self) -> String {
        self.title
    }

    fn print_menu(self) {
        for item in self.elements {
            match item {
                MenuItem::Menu(menu) => println!("Menu: {}", menu.title),
                MenuItem::Func(func) => println!("Func: {}", func.get_title()),
            }
        }
    }

    pub fn run(self) {
        self.print_menu();

    }
}

impl MenuBuilder {
    pub fn empty() -> MenuBuilder {
        MenuBuilder {
            elements: Vec::new(),
            title: "Title".to_string(),
        }
    }

    pub fn build(self) -> Menu {
        Menu {
            elements: self.elements,
            title: self.title,
            selected: 0,
        }
    }

    pub fn push_menu(mut self, menu: Menu) -> MenuBuilder {
        self.elements.push(MenuItem::Menu(menu)); 
        self
    }

    pub fn push_func(mut self, func: Box<dyn MenuFunc>) -> MenuBuilder {
        self.elements.push(MenuItem::Func(func));
        self
    }

    pub fn set_title(mut self, title: String) -> MenuBuilder {
        self.title = title;
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn it_works() {
    //    let result = add(2, 2);
    //    assert_eq!(result, 4);
    //}
}
