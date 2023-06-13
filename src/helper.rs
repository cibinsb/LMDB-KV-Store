use base64::URL_SAFE;
use rand::Rng;

pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let mut buffer: [u8; 20] = [0; 20];
    rng.fill(&mut buffer);
    base64::encode_config(&buffer, URL_SAFE)
}
