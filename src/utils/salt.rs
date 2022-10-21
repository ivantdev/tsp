use md5;
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

fn gen_str() -> String {
    let rnd_str: String = Alphanumeric.sample_string(
        &mut rand::thread_rng(),
        rand::thread_rng().gen_range(0..100),
    );
    rnd_str
}

pub fn gen_salt() -> String {
    let s: String = gen_str();
    let salt = format!("{:x}", md5::compute(&s));
    salt
}
