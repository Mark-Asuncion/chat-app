use rand::{self, Rng};

pub fn gen_uuid(len: u16) -> String {
    let v = "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbnm1234567890".as_bytes();
    let mut rnd = rand::thread_rng();
    let mut uuid = String::new();
    for _ in 0..len {
        let rndn: usize = ( rnd.gen::<f64>() * ( v.len() as f64 ) ).floor() as usize;
        uuid += ( v[rndn] as char )
            .to_string()
            .as_str();
    }
    uuid
}
