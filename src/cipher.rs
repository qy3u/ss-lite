use rand::prelude::*;
const PASSWORD_LENGTH: usize = 256;

type Password = [u8; PASSWORD_LENGTH];

pub fn gen_rand_password() -> Password {
    let mut pw: Vec<u8> = (0..255).collect();
    let mut rng = rand::thread_rng();
    pw.shuffle(&mut rng);

    let mut result = [0u8; PASSWORD_LENGTH];
    result.copy_from_slice(&pw);
    result
}

pub struct Cipher {
    encode_password: Password,
    decode_password: Password,
}

impl Cipher {
    pub fn new(encode_password: Password) -> Self {
        let mut decode_password: Password = [0; PASSWORD_LENGTH];
        decode_password
            .iter_mut()
            .enumerate()
            .for_each(|(i, v)| *v = i as u8);
        Self {
            encode_password,
            decode_password,
        }
    }

    pub fn encode(&self, bs: &mut [u8]) {
        bs.iter_mut()
            .for_each(|x| *x = self.encode_password[*x as usize]);
    }

    pub fn decode(&self, bs: &mut [u8]) {
        bs.iter_mut()
            .for_each(|x| *x = self.decode_password[*x as usize]);
    }
}
