use ephemerality;

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_to_vec() {
        let bytes = [
            16, 0, 0, 0, 69, 112, 104, 101, 109, 101, 114, 97, 108, 32, 98, 117, 114, 103, 101, 114,
            2, 0, 0, 0, 69, 80,
            63, 0, 0, 0, 104, 116, 116, 112, 115, 58, 47, 47, 97, 114, 119, 101, 97, 118, 101, 46, 110, 101, 116, 47, 110, 86, 82, 118, 90, 68, 97, 79, 107, 53, 89, 65, 100, 114, 52, 90, 66, 69, 101, 77, 106, 79, 86, 104, 121, 110, 117, 118, 56, 80, 51, 118, 121, 119, 118, 117, 78, 53, 115, 89, 83, 80, 111,
            1, 0, 0, 0, 16, 0, 0, 0, 100, 101, 115, 116, 114, 111, 121, 84, 105, 109, 101, 115, 116, 97, 109, 112, 10, 0, 0, 0, 49, 54, 57, 52, 57, 56, 55, 56, 54, 57
        ];
        let res = ephemerality::Metadata::try_from_slice(&bytes).unwrap();

        println!("res{:?}", res);
        assert_eq!(1,0)
    }

}
