use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::utils::firebase::verify_firebase_token;

#[derive(Deserialize)]
pub struct TokenRequest {
    pub id_token: String,
}

#[derive(Deserialize)]
pub struct LogoutRequest {
    pub id_token: String,
}


pub async fn login_check(payload: web::Json<TokenRequest>) -> HttpResponse{

    match verify_firebase_token(&payload.id_token).await {
        Ok(claims) => {
            //println!("✅ Verified user: {:?}", claims.email);
            use std::collections::HashSet;
            use std::env;

            let allowed_users: HashSet<String> = env::var("ALLOWED_USERS")
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            let user_email = claims.email.clone().unwrap_or_default();
            if !allowed_users.contains(&user_email) {
                println!("🚫 Unverified User: {}", user_email);
                return HttpResponse::Unauthorized().body("Unverifed user, Access denied");
            }

            println!("✅ Verified User: {:?}", user_email);
            HttpResponse::Ok().json(claims)
        },
        Err(err) => {
            println!("❌ Token verification failed: {}", err);
            HttpResponse::Unauthorized().body("Invalid or expired token")
        }
    }
}

pub async fn logout(payload: web::Json<LogoutRequest>) -> HttpResponse {
    println!("📡 /auth/logout 요청 도착");
    match verify_firebase_token(&payload.id_token).await {
        Ok(claims) => {
            println!("🛑 로그아웃한 사용자: {:?}", claims.email);
            HttpResponse::Ok().body("Logout acknowledged")
        }
        Err(err) => {
            println!("❌ 로그아웃 토큰 검증 실패: {}", err);
            HttpResponse::Unauthorized().body("Invalid token on logout")
        }
    }
}
