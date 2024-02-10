macro_rules! rts {
    ($path:literal) => {
        std::fs::read_to_string($path).unwrap()
    };
}
pub(crate) use rts;
