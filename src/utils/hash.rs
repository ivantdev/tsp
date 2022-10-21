use sha2::{Sha512, Digest};

fn hash(s: &String) -> String {
    let mut hasher = Sha512::new();
    hasher.update(&s);
    let s_hash: String = format!("{:x}", hasher.finalize());
    s_hash
}

pub fn hash_password(salt: &String, pass: &str) -> String {
    let s = format!("{salt}{pass}");
    let pass_hash: String = hash(&s);
    pass_hash
}