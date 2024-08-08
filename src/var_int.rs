pub trait VarInt: Sized {
    const MAX_VAR_INT_SPACE: usize;

    fn encode_var<F>(&self, f: F) -> Result<(), ()>
    where
        F: FnMut(u8) -> Result<(), ()>;

    fn decode_var<F>(f: F) -> Result<(Self, usize), ()>
    where
        F: FnMut(usize) -> Result<u8, ()>;
}

const MSB: u8 = 0b1000_0000;
const DROP_MSB: u8 = 0b0111_1111;

impl VarInt for u32 {
    const MAX_VAR_INT_SPACE: usize = 5;

    fn encode_var<F>(&self, mut f: F) -> Result<(), ()>
    where
        F: FnMut(u8) -> Result<(), ()>,
    {
        let mut n = *self as u64;
        while n >= 0x80 {
            f(MSB | (n as u8))?;
            n >>= 7;
        }
        f(n as u8)?;
        Ok(())
    }

    fn decode_var<F>(mut f: F) -> Result<(Self, usize), ()>
    where
        F: FnMut(usize) -> Result<u8, ()>,
    {
        let mut result: u64 = 0;
        let mut shift = 0;

        let mut success = false;
        let mut i = 0;
        while i < Self::MAX_VAR_INT_SPACE {
            let b = f(i)?;
            i += 1;
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;

            if b & MSB == 0 || shift > (9 * 7) {
                success = b & MSB == 0;
                break;
            }
        }

        if success {
            Ok((result as u32, i /*shift / 7*/))
        } else {
            Err(())
        }
    }
}
