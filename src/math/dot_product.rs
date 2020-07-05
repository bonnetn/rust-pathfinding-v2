#[macro_export]
macro_rules! dot_product {
    ($x:expr, $y:expr) => ( $x.0 * $y.0 + $x.1 * $y.1 );
}

#[cfg(test)]
mod tests {
    #[test]
    fn happy_path() {
        assert_eq!(dot_product!((-1, 2), (3, -4)), -11)
    }
}
