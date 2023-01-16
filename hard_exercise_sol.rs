struct Interface<'a1, 'a2> {
    manager: &'a1 mut Manager<'a2>
}

impl<'a1, 'a2> Interface<'a1, 'a2> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'b1> {
    text: &'b1 str
}

struct List<'d1> {
    manager: Manager<'d1>,
}

impl<'d1> List<'d1> {
    pub fn get_interface<'c1, 'c2>(&'c1 mut self) -> Interface<'c2, 'd1>
        where 'd1 : 'c2, 'c1 : 'c2
    {
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
