mod rmb;

pub fn init(attr: rmb::Attribute) -> Result<rmb::Rmb,String> {
    let r = rmb::Rmb::new(attr);
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() {
        let r = init(rmb::Attribute::Local).unwrap();
        match r.attr() {
            rmb::Attribute::Local => (),
            _ => panic!("Attribute is wrong"),
        }
    }
}
