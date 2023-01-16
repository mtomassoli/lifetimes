struct Interface {
    manager: &mut Manager
}

impl Interface {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager {
    text: &str
}

struct List {
    manager: Manager,
}

impl List {
    pub fn get_interface(&mut self) -> Interface {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
