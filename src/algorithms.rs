use std::collections::HashMap;

const MODULUS: u16 = 101;
const BASE: u16 = 256;

pub fn rabin_karp(target: String, pattern: String) -> Vec<usize> {
    // Quick exit
    if target.is_empty() || pattern.is_empty() || pattern.len() > target.len() {
        return vec![];
    }

    let pattern_hash = hash(pattern.as_str());

    // Pre-calculate BASE^(n-1)
    let mut pow_rem: u16 = 1;
    for _ in 0..pattern.len() - 1 {
        pow_rem *= BASE;
        pow_rem %= MODULUS;
    }

    let mut rolling_hash = 0;
    let mut ret = vec![];
    for i in 0..=target.len() - pattern.len() {
        rolling_hash = if i == 0 {
            hash(&target[0..pattern.len()])
        } else {
            recalculate_hash(
                target.as_str(),
                i - 1,
                i + pattern.len() - 1,
                rolling_hash,
                pow_rem,
            )
        };
        if rolling_hash == pattern_hash && pattern[..] == target[i..i + pattern.len()] {
            ret.push(i);
        }
    }
    ret
}

// hash(s) is defined as BASE^(n-1) * s_0 + BASE^(n-2) * s_1 + ... + BASE^0 * s_(n-1)
fn hash(s: &str) -> u16 {
    let mut res: u16 = 0;
    for &c in s.as_bytes().iter() {
        res = (res * BASE % MODULUS + c as u16) % MODULUS;
    }
    res
}

// new_hash = (old_hash - BASE^(n-1) * s_(i-n)) * BASE + s_i
fn recalculate_hash(
    s: &str,
    old_index: usize,
    new_index: usize,
    old_hash: u16,
    pow_rem: u16,
) -> u16 {
    let mut new_hash = old_hash;
    let (old_ch, new_ch) = (
        s.as_bytes()[old_index] as u16,
        s.as_bytes()[new_index] as u16,
    );
    new_hash = (new_hash + MODULUS - pow_rem * old_ch % MODULUS) % MODULUS;
    new_hash = (new_hash * BASE + new_ch) % MODULUS;
    new_hash
}


pub fn boyer_moore_search(text: &str, pattern: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let n = text.len() as i32;
    let m = pattern.len() as i32;
    let pattern: Vec<char> = pattern.chars().collect();
    let text: Vec<char> = text.chars().collect();
    if n == 0 || m == 0 {
        return positions;
    }
    let mut collection = HashMap::new();
    for (i, c) in pattern.iter().enumerate() {
        collection.insert(c, i as i32);
    }
    let mut shift: i32 = 0;
    while shift <= (n - m) {
        let mut j = m - 1;
        while j >= 0 && pattern[j as usize] == text[(shift + j) as usize] {
            j -= 1;
        }
        if j < 0 {
            positions.push(shift as usize);
            let add_to_shift = {
                if (shift + m) < n {
                    let c = text[(shift + m) as usize];
                    let index = collection.get(&c).unwrap_or(&-1);
                    m - index
                } else {
                    1
                }
            };
            shift += add_to_shift;
        } else {
            let c = text[(shift + j) as usize];
            let index = collection.get(&c).unwrap_or(&-1);
            let add_to_shift = std::cmp::max(1, j - index);
            shift += add_to_shift;
        }
    }
    positions
}

