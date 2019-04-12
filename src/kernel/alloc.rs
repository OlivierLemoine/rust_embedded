const HEAP: [u8; 1024] = [0; 1024];

pub fn malloc(size: u16) -> Result<u32, bool> {
    let mut pos = 0;
    loop {
        if pos >= HEAP.len() {
            break Err(false);
        }

        let v = ((HEAP[pos] as u16) << 8) | (HEAP[pos + 1] as u16);

        let (taken, length) = get_meta_data(v);

        if !taken && length >= size {
            let temp = (pos as u16) | 0x8000;
            HEAP[pos] = ((temp >> 8) & 0x00FF) as u8;
            HEAP[pos] = (temp & 0x00FF) as u8;
            break Ok(pos as u32);
        }

        pos += length as usize;
    }
}


fn get_meta_data(v: u16) -> (bool, u16) {
    ((v & 0x8000) != 0, v & 0x7FFF)
}