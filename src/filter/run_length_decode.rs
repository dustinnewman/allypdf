const EOD: u8 = 128;

pub fn run_length_decode(content: &[u8]) -> Option<Vec<u8>> {
    let mut i = 0;
    let content_len = content.len();
    let mut vec = vec![];
    while i < content_len {
        let len = content[i];
        if len <= 127 {
            let end = i + 2 + len as usize;
            let slice = &content[i + 1..end];
            vec.extend_from_slice(slice);
            i = end;
        } else if len != EOD {
            let byte = content[i + 1];
            let times: u32 = 257 - len as u32;
            for _ in 0..times {
                vec.push(byte);
            }
            i += 2;
        } else {
            break;
        }
    }
    Some(vec)
}

// TODO: Test
