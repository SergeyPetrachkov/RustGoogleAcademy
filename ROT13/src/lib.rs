use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Implement the `Read` trait for `RotDecoder`.
impl<R: Read> Read for RotDecoder<R> {
   fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
       // we're reading from the wrapped reader into the buffer
       let bytes_read = self.input.read(buf)?;

       for byte in &mut buf[..bytes_read] {
           if byte.is_ascii_alphabetic() {
               let base = if byte.is_ascii_uppercase() { 'A' } else { 'a' } as u8;
               // This is the core ROT-N transformation:
               // *byte - base: Get the 0-based index in the alphabet (e.g., 'C' → 2)
               // + self.rot: Add the rotation offset (e.g., ROT13 → shift 13 places)
               // % 26: Wrap around after Z to stay within 0–25
               // + base: Convert the result back to a valid ASCII character
               *byte = (*byte - base + self.rot) % 26 + base;
           }
       }
       Ok(bytes_read)
   }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_slice(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}