pub fn get_challenge_response(code: &str, verification_token: &str, endpoint: &str) -> String {
  sha256::digest(format!("{}{}{}", code, verification_token, endpoint))
}