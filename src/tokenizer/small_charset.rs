/// store a set of small characters, those with unicode value less than 64bits
///
///stored as a bitmap with each bit representing a value
#[derive(Debug, Copy, Clone)]
pub struct SmallCharSet{
    pub bits: u64
}

impl SmallCharSet {
    /// checks whether a u8 value less than 64 is stored in the small char set
    #[inline]
    pub fn contains(&self, n: u8) -> bool {
       0 != (self.bits & (1 << (n as usize)))
    }

    /// count the number of bytes in a buffer not in the set
    pub fn nonmember_prefix_len(&self, buf: &str) -> u32 {
        let mut n = 0;

        for b in buf.bytes(){
            if b >= 64 || !self.contains(b){
                n += 1;
            }else{
                break;
            }
        }

        n
    }
}

/// Create a [`SmallCharSet`] with space separated bytes
#[macro_export]
macro_rules! small_char_set( ($($e:expr)+) => (
    $ crate::tokenizer::small_charset::SmallCharSet{
        bits: $((1 << ($e as usize))) |+
    }
));

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn contains(){
        let set = SmallCharSet{bits: 0b00000000_01000000_00000100_00000000_00000000_00000000_00010000_00000000};
        assert!(set.contains(b'6'));

        let set = small_char_set!(1 2 3);
        assert!(set.contains(1));
        assert!(set.contains(2));
        assert!(set.contains(3));
    }

    #[test]
    fn nonmember_prefix(){
        let set = small_char_set!(b'<' b'>' b'/');
        let test = "some text </tag>";

        assert_eq!(set.nonmember_prefix_len(test), 10);
    }
}