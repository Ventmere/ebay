use hmac_sha256::HMAC;

pub fn get_challenge_response(code: &str, verification_token: &str, endpoint: &str) -> String {
  let mut hasher = HMAC::new(code.as_bytes());
  hasher.update(verification_token.as_bytes());
  hasher.update(endpoint.as_bytes());
  let bytes = hasher.finalize();
  hex::encode(&bytes)
}