use std::{
    any::TypeId,
    collections::HashMap,
    io::{Error, Result},
};

struct App {
    systems: Systems,
    datas: Datas,
}

impl App {
    fn new() -> App {
        App {
            systems: Systems { content: vec![] },
            datas: Datas { content: vec![] },
        }
    }

    fn add_system<S: System>(&mut self, f: S) {
        self.systems.add_system(f);
    }

    fn add_data<T: Arg + 'static>(&mut self, data: T) {
        let id = TypeId::of::<T>();
        if let Some(datas) = self.datas.content.get_mut(&id) {
            datas.push(Box::new(data));
        }
    }

    fn run(&self) {
        for system in self.systems.content.iter() {
            todo!();
            // let data = self.datas.content[0].as_ref();
            // system.run(data.to_string());
        }
    }
}

struct Datas {
    content: HashMap<TypeId, Vec<Box<dyn Arg>>>,
}

struct Systems {
    content: Vec<Box<dyn System>>,
}

impl Systems {
    fn add_system<T: System>(&mut self, f: T) {
        self.content.push(Box::new(f))
    }
}

trait System: 'static {
    fn run(&self, arg: String);
}

trait Arg {}

impl<T: Fn(String) + 'static> System for T {
    fn run(&self, arg: String) {
        (self)(arg);
    }
}

fn system(s: String) {
    println!("{}", s);
}

fn main() {
    let mut app = App::new();
    app.add_system(system);

    app.run();
}

trait If<T> {
    fn more_if(self, f: impl FnOnce(&T) -> bool) -> Result<T>;
}

impl<T> If<T> for std::result::Result<T, Error> {
    fn more_if(self, f: impl FnOnce(&T) -> bool) -> Result<T> {
        self.and_then(|t| {
            if f(&t) {
                Ok(t)
            } else {
                Err(Error::other("error"))
            }
        })
    }
}
