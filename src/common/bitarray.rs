#[derive(Clone)]
pub struct BitArray {
    pub bits: Vec<i32>,
    pub size: isize,
}

impl BitArray {
    pub fn new() -> BitArray {
        return BitArray {
            bits: vec!(),
            size: 0,
        }
    }

    pub fn new_with_size(size: isize) -> BitArray {
        return BitArray {
            bits: vec![0; (size as usize + 31) / 32],
            size: size,
        }
    }

    pub fn get_size(&self) -> isize {
        return self.size;
    }

    pub fn get_size_in_bytes(&self) -> isize {
        return (self.size + 7) / 8;
    }

    fn ensure_capacity(&mut self, size: isize) {
        if size > self.bits.len() as isize * 32 {
            self.bits.resize(size as usize, 0);
        }
    }

    pub fn get(&self, i: isize) -> bool {
        return self.bits[i as usize / 32] & (1 << (i & 0x1f)) != 0;
    }

    pub fn set(&mut self, i: isize) {
        self.bits[i as usize / 32] |= 1 << (i & 0x1f);
    }

    pub fn flip(&mut self, i: isize) {
        self.bits[i as usize / 32] ^= 1 << (i & 0x1f);
    }

    pub fn get_next_set(&self, from: isize) -> isize {
        if from >= self.size {
            return self.size;
        }

        let mut bits_offset = self.size as usize / 32;
        let mut current_bits = self.bits[bits_offset];

        current_bits &= -(1 << (from & 0x1f));
        while current_bits == 0 {
            bits_offset += 1;
            if bits_offset == self.bits.len() {
                return self.size;
            }
            current_bits = self.bits[bits_offset];
        }

        let result = bits_offset as isize * 32 + self.number_of_trailing_zeros(current_bits);
        if result > self.size {
            return self.size;
        } else {
            return result;
        }
    }

    fn number_of_trailing_zeros(&self, mut n: i32) -> isize {
        let mut i: isize = 0;
        if n == 0 {
            return 32;
        }
        while n & 1 == 0 {
            n >>= 1;
            i += 1;
        }
        return i;
    }

    pub fn get_next_unset(&self, from: isize) -> isize {
        if from >= self.size {
            return self.size;
        }

        let mut bits_offset = from as usize / 32;
        let mut current_bits = self.bits[bits_offset];
        current_bits &= -(1 << (from & 0x1f));
        while current_bits == 0 {
            bits_offset += 1;
            if bits_offset == self.bits.len() {
                return self.size;
            }
            current_bits = self.bits[bits_offset];
        }

        let result = bits_offset as isize * 32 + self.number_of_trailing_zeros(current_bits);
        if result > self.size {
            return self.size;
        } else {
            return result;
        }
    }

    pub fn set_bulk(&mut self, i: isize, new_bits: i32) {
        self.bits[i as usize / 32] = new_bits;
    }

    pub fn set_range(&mut self, start: isize, mut end: isize) {
        if end < start || end > self.size {
            unimplemented!();   // TODO IllegalArgumentException
        }

        if end == start {
            return;
        }

        end -= 1;
        let first_int = start /32;
        let last_int = end / 32;
        for i in first_int..(last_int + 1) {
            let first_bit = if i > first_int { 0 } else { start & 0x1f };
            let last_bit = if i < last_int { 31 } else { end & 0x1f };
            let mask = (2 << last_bit) - (1 << first_bit);
            self.bits[i as usize] = mask;
        }
    }

    pub fn clear(&mut self) {
        let max = self.bits.len();
        for i in 0..max {
            self.bits[i] = 0;
        }
    }

    pub fn is_range(&self, start: isize, mut end: isize, value: bool) -> bool {
        if end < start || end > self.size {
            unimplemented!();   // TODO IllegalArgumentException
        }

        if end == start {
            return true;
        }

        end -= 1;
        let first_int = start / 32;
        let last_int = end / 32;
        for i in first_int..(last_int + 1) {
            let first_bit = if i > first_int { 0 } else { start & 0x1f };
            let last_bit = if i < last_int { 31 } else { end & 0x1f };
            let mask = (2 << last_bit) - (1 << first_bit);

            if self.bits[i as usize] & mask != (if value { mask } else { 0 }) {
                return false;
            }
        }

        return true;
    }

    pub fn append_bit(&mut self, bit: bool) {
        self.ensure_capacity(self.size + 1);
        if bit {
            self.bits[self.size as usize / 32] |= 1 << (self.size & 0x1f);
        }
        self.size += 1;
    }

    pub fn append_bit_array(&mut self, other: BitArray) {
        if self.size != other.size {
            unimplemented!();   // TODO IllegalArgumentException
        }

        for i in 0..self.bits.len() {
            self.bits[i] ^= other.bits[i];
        }
    }

    pub fn xor(&mut self, other: BitArray) {
        if self.size != other.size {
            unimplemented!();   // TODO IllegalArgumentException
        }

        for i in 0..self.bits.len() {
            self.bits[i] ^= other.bits[i];
        }
    }

    pub fn to_bytes(&self, mut bit_offset: isize, mut array: Vec<u8>, offset: isize, num_bytes: isize) {
        for i in 0..num_bytes {
            let mut the_byte = 0;
            for j in 0..8 {
                if self.get(bit_offset) {
                    the_byte |= 1 << (7 - j);
                }
                bit_offset += 1;
            }
            array[(offset + i) as usize] = the_byte;
        }
    }

    pub fn get_bit_array(&self) -> &Vec<i32> {
        return &self.bits;
    }

    pub fn reverse(&mut self) {
        let mut new_bits = vec![0; self.bits.len()];

        let len = (self.size - 1) / 32;
        let old_bits_len = len + 1;

        for i in 0..old_bits_len {
            let mut x: i32 = self.bits[i as usize];
            x = ((x >>  1) & 0x55555555) | ((x & 0x55555555) <<  1);
            x = ((x >>  2) & 0x33333333) | ((x & 0x33333333) <<  2);
            x = ((x >>  4) & 0x0f0f0f0f) | ((x & 0x0f0f0f0f) <<  4);
            x = ((x >>  8) & 0x00ff00ff) | ((x & 0x00ff00ff) <<  8);
            x = ((x >> 16) & 0x0000ffff) | ((x & 0x0000ffff) << 16);
            new_bits[(len - i) as usize] = x;
        }

        if self.size != old_bits_len * 32 {
            let left_offset = old_bits_len * 32 - self.size;
            let mut current_int = new_bits[0] >> left_offset;
            for i in 1..old_bits_len {
                let next_int = new_bits[i as usize];
                current_int |= next_int << (32 - left_offset);
                new_bits[(i - 1) as usize] = current_int;
                current_int = next_int >> left_offset;  // TODO unsigned right shift
            }
            new_bits[(old_bits_len - 1) as usize] = current_int;
        }

        self.bits = new_bits;
    }
}