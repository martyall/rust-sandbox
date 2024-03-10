use std::collections::BTreeMap;

macro_rules! btree {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut temp_map = BTreeMap::new();
            $(
                temp_map.insert($x, $y);
            )*
            temp_map
        }
    };
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_macro() {
        let m = btree!((1, 'a'), (2, 'b'));
        assert_eq!(m.get(&1), Some(&'a'));
        assert_eq!(m.get(&2), Some(&'b'))
    }
}
