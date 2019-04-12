const HEAP: [u8; 1024] = [0; 1024];

pub fn init() {
    let temp = HEAP.len();
    HEAP[0] = ((temp >> 8) & 0x00FF) as u8;
    HEAP[1] = (temp & 0x00FF) as u8;
}

pub fn malloc(size: u16) -> Result<u32, bool> {
    let mut pos = 0;
    loop {
        if pos >= HEAP.len() || size > 32768 {
            break Err(false);
        }

        let v = ((HEAP[pos] as u16) << 8) | (HEAP[pos + 1] as u16);

        let (taken, length) = get_meta_data(v);

        if !taken && length >= size {
            let temp = (pos as u16) | 0x8000;
            HEAP[pos] = ((temp >> 8) & 0x00FF) as u8;
            HEAP[pos + 1] = (temp & 0x00FF) as u8;
            break Ok((pos + 2) as u32);
        }

        pos += length as usize;
    }
}

pub fn free(pos: u32) -> Result<bool, bool> {
    let index = (pos - 2) as usize;

    if pos < 2 || index >= HEAP.len() {
        return Err(false);
    }

    HEAP[index] &= 0x7F;

    let v = ((HEAP[index] as u16) << 8) | (HEAP[index + 1] as u16);
    let (taken, length) = get_meta_data(v);

    let size = length as usize;

    if index + size < HEAP.len() {
        let v2 = ((HEAP[index + size] as u16) << 8)
            | (HEAP[index + size + 1] as u16);

        let (taken2, length2) = get_meta_data(v2);

        if !taken2 {
            let to_write = (length + length2) & 0x7FFF;
            HEAP[index] = ((to_write >> 8) & 0x00FF) as u8;
            HEAP[index + 1] = (to_write & 0x00FF) as u8;
        }
    }


    Ok(true)
}


fn get_meta_data(v: u16) -> (bool, u16) {
    ((v & 0x8000) != 0, v & 0x7FFF)
}