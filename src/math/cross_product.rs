#[macro_export]
macro_rules! cross_product {
    ($x:expr, $y:expr) => ( $x.0 * $y.1 - $x.1 * $y.0 );
}

#[cfg(test)]
mod tests {
    #[test]
    fn happy_path() {
        assert_eq!(cross_product!((-1, 2), (3, -4)), -2)
    }
}
