#[macro_export]
macro_rules! to_u64 {
    ($e:expr) => {
        {
            let from_bytes = $e.to_be_bytes();
            let mut to_bytes: [u8; size_of::<u64>()] = [0; size_of::<u64>()];
            let d = to_bytes.len() - from_bytes.len();
            for (i, j) in (d..to_bytes.len()).enumerate() {
                to_bytes[j] = from_bytes[i];
            }

            u64::from_be_bytes(to_bytes)
        }
    };
}

#[macro_export]
macro_rules! from_u64 {
    ($e:expr; $t:ty) => {
        {
            let from_bytes = $e.to_be_bytes();
            let mut to_bytes: [u8; size_of::<$t>()] = [0; size_of::<$t>()];
            let d = from_bytes.len() - to_bytes.len();
            for (i, j) in (d..from_bytes.len()).enumerate() {
                to_bytes[i] = from_bytes[j];
            }

            <$t>::from_be_bytes(to_bytes)
        }
    };
}

pub use {to_u64, from_u64};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64() {
        type Type = i64;
        let vals: Vec<Type> = vec![-1000 * 1000 * 1000, -100, 0, 100, 1000 * 1000 * 1000];
        for i in vals {
            let b = to_u64!(i);
            let c = from_u64!(b; Type);
            assert_eq!(i, c);
        }
    }

    #[test]
    fn test_i32() {
        type Type = i32;
        let vals: Vec<Type> = vec![-1000 * 1000, -100, 0, 100, 1000 * 1000];
        for i in vals {
            let b = to_u64!(i);
            let c = from_u64!(b; Type);
            assert_eq!(i, c);
        }
    }

    #[test]
    fn test_f64() {
        type Type = f64;
        let vals: Vec<Type> = vec![-1000.5 * 1000.4 * 1000.1, -100.14, 0.0, 100.14, 1000.5 * 1000.4 * 1000.1];
        for i in vals {
            let b = to_u64!(i);
            let c = from_u64!(b; Type);
            assert_eq!(i, c);
        }
    }

    #[test]
    fn test_f32() {
        type Type = f32;
        let vals: Vec<Type> = vec![-1000.5 * 1000.4, -100.14, 0.0, 100.14, 1000.5 * 1000.4];
        for i in vals {
            let b = to_u64!(i);
            let c = from_u64!(b; Type);
            assert_eq!(i, c);
        }
    }

    #[test]
    fn test_u8() {
        type Type = u8;
        let i: Type = 'a' as u8;
        let b = to_u64!(i);
        let c = from_u64!(b; Type);
        assert_eq!(i, c);
    }

    #[test]
    fn test_u16() {
        type Type = u16;
        let i: Type = 100;
        let b = to_u64!(i);
        let c = from_u64!(b; Type);
        assert_eq!(i, c);
    }

    #[test]
    fn test_u32() {
        type Type = u32;
        let i: Type = 100 * 100;
        let b = to_u64!(i);
        let c = from_u64!(b; Type);
        assert_eq!(i, c);
    }

    #[test]
    fn test_u64() {
        type Type = u64;
        let i: Type = 1000 * 1000 * 100;
        let b = to_u64!(i);
        let c = from_u64!(b; Type);
        assert_eq!(i, c);
    }

    #[test]
    fn test_usize() {
        type Type = usize;
        let i: Type = 1000 * 1000 * 100;
        let b = to_u64!(i);
        let c = from_u64!(b; Type);
        assert_eq!(i, c);
    }

    #[test]
    fn test_isize() {
        type Type = isize;
        let i: Type = -1000 * 1000 * 100;
        let b = to_u64!(i);
        let c = from_u64!(b; Type);
        assert_eq!(i, c);
    }
}