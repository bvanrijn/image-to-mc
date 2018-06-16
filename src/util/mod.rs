use std::{thread, time};

pub fn exclude_range(it: &mut Vec<u32>, start: u32, stop: u32) {
    for i in start..=stop {
        it.push(i)
    }
}

pub fn color_to_string(color: [u8; 3]) -> String {
    format!("{},{},{}", color[0], color[1], color[2])
}

pub fn sleep(s: f64) {
    let ten_millis = time::Duration::from_millis((s * 1000.0) as u64);

    thread::sleep(ten_millis);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exclude_range_len() {
        let mut it: Vec<u32> = vec![0, 1, 2, 3];
        exclude_range(&mut it, 4, 10);
    
        assert_eq!(it.len(), 11);
    }

    #[test]
    fn test_exclude_range_last_element() {
        let mut it: Vec<u32> = vec![0, 1, 2, 3];
        exclude_range(&mut it, 4, 10);

        assert_eq!(it[it.len()-1], 10);
    }

    #[test]
    fn test_color_to_string() {
        assert_eq!(color_to_string([12,34,56]), "12,34,56");
    }
}
