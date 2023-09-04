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
    selected: usize,
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

    fn print_menu(&self) {
        for (ind, item) in self.elements.iter().enumerate() {
            if ind == self.selected {
                match item {
                    MenuItem::Menu(menu) => println!("{}",
                                                    console::style(
                                                        format!(">> Menu: {}", menu.title)
                                                        ).bold()
                                                    ),
                    MenuItem::Func(func) => println!("{}",
                                                    console::style(
                                                        format!(">> Func: {}", func.get_title())
                                                        ).bold()
                                                    ),
                }
                continue;
            }

            match item {
                MenuItem::Menu(menu) => println!("   Menu: {}", menu.title),
                MenuItem::Func(func) => println!("   Func: {}", func.get_title()),
            }
        }
    }

    pub fn run(mut self) {
        println!("{}", termion::clear::All);
        self.print_menu();
        let term = console::Term::stdout();

        loop {
            let res = term.read_char();

            match res {
                Ok('q') | Err(_) => break,
                Ok('k') => self.nav_up(),
                Ok('j') => self.nav_down(),
                Ok('l') => self.call_selected(),
                _ => continue,
            };

            println!("{}", termion::clear::All);
            self.print_menu();
        }
    }

    fn nav_up(&mut self) {
        if self.selected == 0 {
            self.selected = self.elements.len() - 1;
            return;
        }
        self.selected -= 1;
    }

    fn nav_down(&mut self) {
        if self.selected >= self.elements.len() - 1 {
            self.selected = 0;
            return;
        }
        self.selected += 1;
    }

    fn call_selected(&mut self) {
        let el = &self.elements[self.selected];

        match el {
            MenuItem::Func(f) => {},
            MenuItem::Menu(m) => {},
        };
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
