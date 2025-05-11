const WORD_SIZE: usize = size_of::<u64>();

macro_rules! to_u64 {
    ($e:expr) => {
        {
            let from_bytes = $e.to_be_bytes();
            let mut to_bytes: [u8; WORD_SIZE] = [0; WORD_SIZE];
            let d = to_bytes.len() - from_bytes.len();
            for (i, j) in (d..to_bytes.len()).enumerate() {
                to_bytes[j] = from_bytes[i];
            }

            u64::from_be_bytes(to_bytes)
        }
    };
}

macro_rules! from_u64 { // TODO need to support any size int
    ($e:expr, $t:ty) => {
        {
            let bytes = $e.to_be_bytes();
            <$t>::from_be_bytes(bytes)
        }
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64() {
        type Type = i64;
        let i: Type = 100;
        let b = to_u64!(i);
        let c = from_u64!(b, Type);
        assert_eq!(i, c);
    }

    // #[test]
    // fn test_i32() {
    //     type Type = i32;
    //     let i: Type = 100;
    //     let b = to_u64!(i);
    //     let c = from_u64!(b, Type);
    //     assert_eq!(i, c);
    // }
    
    #[test]
    fn test_f64() {
        type Type = f64;
        let i: Type = 100.14;
        let b = to_u64!(i);
        let c = from_u64!(b, Type);
        assert_eq!(i, c);
    }
    
    // #[test]
    // fn test_f32() {
    //     type Type = f32;
    //     let i: Type = 100.14;
    //     let b = to_u64!(i);
    //     let c = from_u64!(b, Type);
    //     assert_eq!(i, c);
    // }
}