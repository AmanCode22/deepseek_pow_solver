const RC: [u64; 24] = [
    0x0000000000000001,
    0x0000000000008082,
    0x800000000000808A,
    0x8000000080008000,
    0x000000000000808B,
    0x0000000080000001,
    0x8000000080008081,
    0x8000000000008009,
    0x000000000000008A,
    0x0000000000000088,
    0x0000000080008009,
    0x000000008000000A,
    0x000000008000808B,
    0x800000000000008B,
    0x8000000000008089,
    0x8000000000008003,
    0x8000000000008002,
    0x8000000000000080,
    0x000000000000800A,
    0x800000008000000A,
    0x8000000080008081,
    0x8000000000008080,
    0x0000000080000001,
    0x8000000080008008,
];

const ROTC: [u32; 24] = [
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 2, 14, 27, 41, 56, 8, 25, 43, 62, 18, 39, 61, 20, 44,
];

const PILN: [usize; 24] = [
    10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4, 15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
];

#[inline(always)]
fn rotl64(x: u64, y: u32) -> u64 {
    (x << y) | (x >> (64 - y))
}

fn keccak_f1600(state: &mut [u64; 25]) {
    for round in 1..24 {
        let c0 = state[0] ^ state[5] ^ state[10] ^ state[15] ^ state[20];
        let c1 = state[1] ^ state[6] ^ state[11] ^ state[16] ^ state[21];
        let c2 = state[2] ^ state[7] ^ state[12] ^ state[17] ^ state[22];
        let c3 = state[3] ^ state[8] ^ state[13] ^ state[18] ^ state[23];
        let c4 = state[4] ^ state[9] ^ state[14] ^ state[19] ^ state[24];

        let d0 = c4 ^ rotl64(c1, 1);
        let d1 = c0 ^ rotl64(c2, 1);
        let d2 = c1 ^ rotl64(c3, 1);
        let d3 = c2 ^ rotl64(c4, 1);
        let d4 = c3 ^ rotl64(c0, 1);

        state[0] ^= d0;
        state[5] ^= d0;
        state[10] ^= d0;
        state[15] ^= d0;
        state[20] ^= d0;
        state[1] ^= d1;
        state[6] ^= d1;
        state[11] ^= d1;
        state[16] ^= d1;
        state[21] ^= d1;
        state[2] ^= d2;
        state[7] ^= d2;
        state[12] ^= d2;
        state[17] ^= d2;
        state[22] ^= d2;
        state[3] ^= d3;
        state[8] ^= d3;
        state[13] ^= d3;
        state[18] ^= d3;
        state[23] ^= d3;
        state[4] ^= d4;
        state[9] ^= d4;
        state[14] ^= d4;
        state[19] ^= d4;
        state[24] ^= d4;

        let mut temp = *state;
        let mut t = state[1];
        for i in 0..24 {
            let j = PILN[i];
            temp[j] = rotl64(t, ROTC[i]);
            t = state[j];
        }
        *state = temp;

        for y in (0..25).step_by(5) {
            let b0 = state[y];
            let b1 = state[y + 1];
            let b2 = state[y + 2];
            let b3 = state[y + 3];
            let b4 = state[y + 4];
            state[y] = b0 ^ ((!b1) & b2);
            state[y + 1] = b1 ^ ((!b2) & b3);
            state[y + 2] = b2 ^ ((!b3) & b4);
            state[y + 3] = b3 ^ ((!b4) & b0);
            state[y + 4] = b4 ^ ((!b0) & b1);
        }

        state[0] ^= RC[round];
    }
}

static mut BUMP_PTR: usize = 1024;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    let ptr: *mut u8;
    unsafe {
        ptr = BUMP_PTR as *mut u8;
        BUMP_PTR += size;
        if BUMP_PTR > 65536 {
            BUMP_PTR = 1024;
        }
    }
    ptr
}

#[no_mangle]
pub extern "C" fn solve_pow(
    challenge_hex_ptr: *const u8,
    challenge_hex_len: u32,
    salt_ptr: *const u8,
    salt_len: u32,
    expire_at: u64,
    difficulty: u64,
) -> u64 {
    let challenge_hex = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            challenge_hex_ptr,
            challenge_hex_len as usize,
        ))
    };
    let salt = unsafe {
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(salt_ptr, salt_len as usize))
    };

    let mut challenge_bytes = [0u8; 32];
    for i in 0..32 {
        let byte_str = &challenge_hex[i * 2..i * 2 + 2];
        challenge_bytes[i] = u8::from_str_radix(byte_str, 16).unwrap();
    }

    let prefix = format!("{}_{}_", salt, expire_at);
    let prefix_bytes = prefix.as_bytes();

    let mut base_st: [u64; 25] = [0; 25];
    for (i, &b) in prefix_bytes.iter().enumerate() {
        let lane_idx = i / 8;
        let byte_idx = i % 8;
        base_st[lane_idx] |= (b as u64) << (byte_idx * 8);
    }

    let mut nonce_buf = [0u8; 20];

    for nonce in 0..difficulty {
        let mut st = base_st;

        let mut n = nonce;
        let mut len = 0;
        if n == 0 {
            nonce_buf[0] = b'0';
            len = 1;
        } else {
            while n > 0 {
                nonce_buf[len] = b'0' + (n % 10) as u8;
                n /= 10;
                len += 1;
            }
            nonce_buf[..len].reverse();
        }
        let nonce_bytes = &nonce_buf[..len];

        for (i, &b) in nonce_bytes.iter().enumerate() {
            let idx = prefix_bytes.len() + i;
            let lane_idx = idx / 8;
            let byte_idx = idx % 8;
            st[lane_idx] ^= (b as u64) << (byte_idx * 8);
        }

        let idx_06 = prefix_bytes.len() + nonce_bytes.len();
        let lane_idx_06 = idx_06 / 8;
        let byte_idx_06 = idx_06 % 8;
        st[lane_idx_06] ^= 0x06u64 << (byte_idx_06 * 8);

        st[16] ^= 0x8000000000000000;

        keccak_f1600(&mut st);

        let mut out_bytes = [0u8; 32];
        out_bytes[0..8].copy_from_slice(&st[0].to_le_bytes());
        out_bytes[8..16].copy_from_slice(&st[1].to_le_bytes());
        out_bytes[16..24].copy_from_slice(&st[2].to_le_bytes());
        out_bytes[24..32].copy_from_slice(&st[3].to_le_bytes());

        if out_bytes == challenge_bytes {
            return nonce;
        }
    }
    u64::MAX
}

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub extern "C" fn solve_pow_native(
    challenge_hex_ptr: *const u8,
    challenge_hex_len: u32,
    salt_ptr: *const u8,
    salt_len: u32,
    expire_at: u64,
    difficulty: u64,
) -> u64 {
    solve_pow(
        challenge_hex_ptr,
        challenge_hex_len,
        salt_ptr,
        salt_len,
        expire_at,
        difficulty,
    )
}
