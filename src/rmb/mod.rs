pub struct Rmb {
    attr: Attribute,
}

#[derive(Debug)]
pub enum Attribute {
    Internal,
    Local,
    Network
}


impl Rmb {
    pub fn new(attr: Attribute) -> Rmb {
        Rmb {
            attr: attr,
        }
    }
    pub fn attr(&self) -> &Attribute {
        &self.attr
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
