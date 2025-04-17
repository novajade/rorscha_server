use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct FirebaseClaims {
    pub email: Option<String>,
    pub user_id: Option<String>,
    pub aud: String,
    pub iss: String,
    pub sub: String,
}

//Google로그인 후, 받은 id_token을 Firebase
pub async fn verify_firebase_token(id_token: &str) -> Result<FirebaseClaims, String> {
    let project_id = env::var("FIREBASE_PROJECT_ID").map_err(|_| "Missing FIREBASE_PROJECT_ID")?;

    // 1. Decode header to get kid
    let header = decode_header(id_token).map_err(|e| format!("Failed to decode header: {}", e))?;
    let kid = header.kid.ok_or("No 'kid' found in token header")?;

    // 2. Get Firebase public keys
    let client = Client::new();
    let resp = client
        .get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    let keys: HashMap<String, String> = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse keys: {}", e))?;
    let cert_pem = keys.get(&kid).ok_or("Public key not found for token")?;

    // 3. Validate token
    
    let mut aud_set: std::collections::HashSet<String> = std::collections::HashSet::new();
    aud_set.insert(project_id.clone().to_string());
    let mut iss_set = std::collections::HashSet::new();
    iss_set.insert(format!("https://securetoken.google.com/{}", project_id));

    let mut validation = Validation::new(Algorithm::RS256);
    validation.aud = Some(aud_set);
    validation.iss = Some(iss_set);

    let token_data = decode::<FirebaseClaims>(
        id_token,
        &DecodingKey::from_rsa_pem(cert_pem.as_bytes())
            .map_err(|e| format!("Failed to parse public key: {}", e))?,
        &validation,
    )
    .map_err(|e| format!("Token validation error: {}", e))?;

    Ok(token_data.claims)
}