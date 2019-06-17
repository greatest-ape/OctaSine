use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

use array_init::array_init;


pub const ATOMIC_STRING_NUM_U64S: usize = 4;


/// Experiment with atomic string consisting of severeral u64s.
/// 
/// Doesn't really work.
pub struct AtomicString {
    storage: [AtomicU64; ATOMIC_STRING_NUM_U64S],
    num_bytes: AtomicUsize,
}


impl AtomicString {
    pub fn new(value: String) -> Self {
        let atomic_string = Self {
            storage: array_init(|_| AtomicU64::new(0)),
            num_bytes: AtomicUsize::new(0),
        };

        atomic_string.set(value);

        atomic_string
    }

    pub fn set(&self, string_value: String){
        let value = Self::trim_string(string_value);
        let string_bytes = value.as_bytes();

        for storage_index in 0..ATOMIC_STRING_NUM_U64S {
            let mut number_bytes = [0u8; 8];

            for i in 0..8 {
                let string_byte_index = i + (storage_index * 8);

                if let Some(byte) = string_bytes.get(string_byte_index){
                    number_bytes[i] = *byte;
                }
            }

            self.storage[storage_index]
                .store(u64::from_le_bytes(number_bytes), Ordering::SeqCst);
        }

        self.num_bytes.store(string_bytes.len(), Ordering::SeqCst);
    }

    pub fn get(&self) -> Option<String> {
        let mut string_bytes = Vec::new();

        for n in self.storage.iter(){
            let n = n.load(Ordering::SeqCst);

            string_bytes.extend_from_slice(&n.to_le_bytes());
        }

        let num_bytes = self.num_bytes.load(Ordering::SeqCst);

        let string_bytes: Vec<u8> = string_bytes.into_iter()
            .take(num_bytes)
            .collect();

        String::from_utf8(string_bytes).ok()
    }

    pub fn trim_string(value: String) -> String {
        let max_num_bytes = ATOMIC_STRING_NUM_U64S * 8;

        if value.len() < max_num_bytes {
            return value
        }

        for i in 0..=4 {
            let index = max_num_bytes - i;

            if value.is_char_boundary(index){
                return String::from_utf8(value.as_bytes()[0..index].to_vec())
                    .expect("AtomicString.trim_string()");
            }
        }

        unreachable!();
    }

    pub fn max_len() -> usize {
        ATOMIC_STRING_NUM_U64S * 8
    }
}


#[cfg(test)]
mod tests {
    use quickcheck::{TestResult, quickcheck};

    use super::*;


    #[test]
    fn test_atomic_string_multiple_threads(){
        use std::sync::Arc;
        use std::{thread, time};
        use std::collections::HashSet;

        use rand::{Rng, thread_rng};

        let all_strings = Arc::new({
            let all_chars = vec!['𠸎', '肉', '©', 'a'];

            let mut rng = thread_rng();
            let mut all_strings = Vec::new();

            for _ in 0..1000 {
                let mut string = String::new();

                for _ in 0..ATOMIC_STRING_NUM_U64S * 9 {
                    let i = rng.gen_range(0, all_chars.len() - 1);
                    string.push(all_chars[i])
                }

                all_strings.push(string);
            }

            all_strings
        });

        let atomic_string = Arc::new(AtomicString::new("".to_string()));
        let mut handles = Vec::new();

        for _ in 0..8 {
            let atomic_string = atomic_string.clone();
            let all_strings = all_strings.clone();

            handles.push(std::thread::spawn(move || {
                let mut rng = thread_rng();

                let mut reported_strings = HashSet::new();
                let mut failure_reported = false;

                for _ in 0..100000 {
                    thread::sleep(time::Duration::from_nanos(rng.gen_range(0, 1000)));

                    let string_index = rng.gen_range(0, all_strings.len() - 1);

                    atomic_string.set(all_strings[string_index].clone());

                    thread::sleep(time::Duration::from_nanos(rng.gen_range(0, 1000)));

                    if let Some(string) = atomic_string.get(){
                        reported_strings.insert(string);
                    } else {
                        failure_reported = true;

                        break;
                    }
                }

                (reported_strings, failure_reported)
            }));
        }
        
        let all_strings = all_strings.iter()
            .map(|s| AtomicString::trim_string(s.clone()))
            .collect::<HashSet<String>>();

        for handle in handles {
            let (reported_strings, failure_reported) = handle.join().unwrap();

            assert!(!failure_reported);
            assert!(reported_strings.is_subset(&all_strings));
        }
    }

    #[test]
    fn test_atomic_string_trim_string(){
        fn check(a: &str, b: &str){
            assert_eq!(AtomicString::trim_string(a.to_string()), b.to_string());
        }

        check("", "");
        check("a", "a");
        check("0123456789abcdef0123456789abcdefg", "0123456789abcdef0123456789abcdef");

        check("0123456789abcdef0123456789abcdef",  "0123456789abcdef0123456789abcdef");
        check("0123456789abcdef0123456789abcde虎", "0123456789abcdef0123456789abcde");
        check("0123456789abcdef0123456789abcd虎", "0123456789abcdef0123456789abcd");
        check("0123456789abcdef0123456789abc虎", "0123456789abcdef0123456789abc虎");
        check("0123456789abcdef0123456789ab虎0", "0123456789abcdef0123456789ab虎0");
        check("0123456789abcdef0123456789ab虎", "0123456789abcdef0123456789ab虎");
    }

    #[test]
    fn test_atomic_string_set_get(){
        fn prop(value: String) -> TestResult {
            let atomic_string = AtomicString::new("".to_string());

            atomic_string.set(value.clone());

            TestResult::from_bool(
                atomic_string.get() == Some(AtomicString::trim_string(value))
            )
        }

        quickcheck(prop as fn(String) -> TestResult);
    }
}