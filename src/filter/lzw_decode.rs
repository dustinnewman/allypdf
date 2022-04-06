use std::collections::HashMap;

pub fn lwz_decode(mut data: &[u32]) -> Option<Vec<u8>> {
    // Build the dictionary.
    let mut dictionary: HashMap<u32, Vec<u8>> = (0u32..=255).map(|i| (i, vec![i as u8])).collect();

    let mut w = dictionary[&data[0]].clone();
    data = &data[1..];
    let mut decompressed = w.clone();

    for &k in data {
        let entry = if dictionary.contains_key(&k) {
            dictionary[&k].clone()
        } else if k == dictionary.len() as u32 {
            let mut entry = w.clone();
            entry.push(w[0]);
            entry
        } else {
            return None;
        };

        decompressed.extend_from_slice(&entry);

        // New sequence; add it to the dictionary.
        w.push(entry[0]);
        dictionary.insert(dictionary.len() as u32, w);

        w = entry;
    }

    Some(decompressed)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_lzw_to_be_or_not_to_be() {
        let text: [u32; 16] = [
            84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263,
        ];
        let decompressed = lwz_decode(&text);
        assert_eq!(
            String::from_utf8(decompressed.unwrap()).unwrap(),
            "TOBEORNOTTOBEORTOBEORNOT"
        );
    }
}
