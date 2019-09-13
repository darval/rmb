pub mod rmb;
pub mod local;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init() {
        let t = local::TransportLocal::new();
        let mut r = rmb::Rmb::new(&t);
        r.init().unwrap();
    }
}
