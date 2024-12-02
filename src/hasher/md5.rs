use super::Hasher;
use bitvec::prelude::*;

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

pub struct Md5 {
    curr: Vec<u8>,
}

impl Hasher for Md5 {
    fn new() -> Self {
        Self { curr: Vec::new() }
    }

    fn update(&mut self, input: &[u8]) {
        self.curr.extend(input);
    }

    fn reset(&mut self) {
        self.curr = Vec::new()
    }

    fn finalize(&mut self) -> String {
        Md5::hash(&self.curr)
    }

    fn hash(input: &[u8]) -> String {
        let mut a0: u32 = 0x67452301;
        let mut b0: u32 = 0xefcdab89;
        let mut c0: u32 = 0x98badcfe;
        let mut d0: u32 = 0x10325476;

        // 1. Convert the input to a bitvec
        let mut bitvec: BitVec<u8, Msb0> = BitVec::from_slice(input);

        // 2. Pad the input
        let original_len_bits = (bitvec.len() as u64).to_le_bytes();

        bitvec.push(true);
        while bitvec.len() % 512 != 448 {
            bitvec.push(false);
        }
        bitvec.extend(original_len_bits);

        for block in bitvec.chunks(512) {
            let mut words = [0u32; 16];
            block.chunks(32).enumerate().for_each(|(i, v)| {
                words[i] = v.load::<u32>();
            });

            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;

            for i in 0..64 {
                let (f, g) = match i {
                    0..=15 => ((b & c) | ((!b) & d), i),
                    16..=31 => ((d & b) | ((!d) & c), (5 * i + 1) % 16),
                    32..=47 => (b ^ c ^ d, (3 * i + 5) % 16),
                    48..=63 => (c ^ (b | (!d)), (7 * i) % 16),
                    _ => unreachable!(),
                };

                let temp = d;
                d = c;
                c = b;
                b = b.wrapping_add(u32::rotate_left(
                    a.wrapping_add(f).wrapping_add(K[i]).wrapping_add(words[g]),
                    S[i],
                ));
                a = temp;
            }

            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }

        // Convert a0, b0, c0, d0 to a hexadecimal string
        format!(
            "{:08x}{:08x}{:08x}{:08x}",
            a0.to_be(),
            b0.to_be(),
            c0.to_be(),
            d0.to_be()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_md5() {
        let mut hasher = Md5::new();
        hasher.update(b"PASS");

        assert_eq!("7a95bf926a0333f57705aeac07a362a2", hasher.finalize());

        hasher.reset();
        hasher.update(b"CODE");
        assert_eq!("08054846bbc9933fd0395f8be516a9f9", hasher.finalize());
    }
}
