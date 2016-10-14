#![allow(unused_variables,unused_imports)]

pub fn enc(plaintext: &[u8], nonce: &[u8]) -> Result<(), ()> {
    Ok(())
}


#[cfg(test)]
mod tests {

    use super::enc;

    #[test]
    fn test_encoding() {
        let mut _plaintext = "test";
        //enc(&mut plaintext)
    }

}
