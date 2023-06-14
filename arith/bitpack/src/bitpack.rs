//use std::convert::TryInto;
//use std::fmt;
//use std::mem::size_of;

/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
    if n < 0 {
        (-n) as u64 <= (1 << (width - 1))
    } 
    else {
        (n as u64) < (1 << (width - 1))
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if n>>width == 0{
        return true;
    }
    if width == 0 {
        return n == 0;
    }
    else{
        return false;
    }
}

/// Retrieve a signed value from word, represented by width bits
/// beginning at least-significant bit lsb.
///
/// # Arguments:
/// * word: An unsigned word
/// * width: the width of a bit field
/// * lsb: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    return (word << (64 - width - lsb)) as i64 >> (64 - width)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    return (word << (64 - width - lsb)) >> (64 - width)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    if fitsu(value, width){
        return Some(value << lsb | word)
    } 
    else {
        return None
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitss(value, width){
        return Some(((value & (!(-1_i64 << width)) as i64) as u64) << lsb | word)
    } 
    else {
        return None
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /// TESTING FITSS
    #[test]
    fn test_fitss() {
        assert_eq!(fitss(-13, 5), true);
    }

    #[test]
    fn test_fitss_false() {
        assert_eq!(fitss(3000, 2), false);
    }

    #[test]
    fn test_fitss_edge() {
        assert_eq!(fitss(31, 5), true);
    }

    /// TESTING FITSU
    #[test]
    fn test_fitsu() {
        assert_eq!(fitsu(6, 5), true);
    }

    #[test]
    fn test_fitsu_false() {
        assert_eq!(fitsu(263, 3), false);
    }

    #[test]
    fn test_fitsu_edge() {
        assert_eq!(fitsu(31, 5), true);
    }
    
    /// TESTING NEWS
    #[test]
    fn test_news_none() {
        assert_eq!(news(0_u64, 9, 23, 48759347853985), None);
    }

    #[test]
    fn test_news() {
        assert_eq!(news(0_u64, 9, 23, -0.98 as i64).unwrap(), 0);
    }

    // TESTING NEWU
    #[test]
    fn test_newu_none() {
        assert_eq!(newu(0_u64, 5, 34, 48759347853985), None);
    }

    #[test]
    fn test_newu() {
        assert_eq!(news(2122317824, 3, 2, 1 as i64).unwrap(), 2122317828);
    }

    // TESTING GETS
    #[test]
    fn test_gets() {
        assert_eq!(gets(2124398510, 2, 1), -1);
    }

    #[test]
    fn test_gets2() {
        assert_eq!(gets(420202, 21, 6), 6565);
    }

    // TESTING GETU
    #[test]
    fn test_getu() {
        assert_eq!(getu(2124398510, 2, 1), 3);
    }

    #[test]
    fn test_getu2() {
        assert_eq!(getu(420202, 21, 6), 6565);
    }

}