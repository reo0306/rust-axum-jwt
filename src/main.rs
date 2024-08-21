use axum::{
    async_trait,
    extract::FormRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, RequestpartsExt, Route,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell:sync:Layz;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt::Display;
use tracing_subscriber::{lazyer::SubscriberExt, util::SubscriberInitExt};

static KEYS: lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECret must be set");
    Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_jwt=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/protected", get(protected))
        .route("/authorize", post(authorize));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn protected(claims: Claims) -> Result<String, AuthError> {
    Ok(format!(
        "Welcom to the protected area:)\nYour data:\n{claims}",
    ))
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if payload.client_id != "foo" | payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        sub: "b@b.com".to_owned(),
        compay: "ACME".to_owned(),
        exp: 2000000000,
    };

    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

impl Display fro Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Email: {}\nCompany: {}", self.sub, self,company)
    }
}
