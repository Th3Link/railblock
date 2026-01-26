pub fn write_u8(s: &mut heapless::String<100>, value: u8) -> Result<(), ()> {
    let mut buf = [0u8; 3];
    let mut n = value;
    let mut i = 3;
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10);
        n /= 10;
        if n == 0 {
            break;
        }
    }
    for &b in &buf[i..] {
        s.push(b as char).map_err(|_| ())?;
    }
    Ok(())
}
