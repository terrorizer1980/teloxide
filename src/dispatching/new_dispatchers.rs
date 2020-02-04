use std::vec::Vec;

struct Observer {
    upd_kind: UpdateKind, 
    func: 
}

impl Observer {
    pub fn new(update_kind, func: ) -> Self {

    } 
}

struct Dispatcher {
    observers: Vec<Observer>,
}

impl Dispatcher {
    pub fn new(observers: [Observer]) -> Self {
        Dispatcher{Vec::from(observers)}
    }

}