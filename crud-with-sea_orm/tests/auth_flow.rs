use actix_web::{App, test, web};
use migration::MigratorTrait;
use serde_json::Value;
use std::env;

/// End-to-end integration test for the authentication lifecycle.
///
/// IMPORTANT: This test writes to the database specified in DATABASE_URL.
/// For safety, it's recommended to use a separate test database:
/// - Set TEST_DATABASE_URL in your environment for tests
/// - Or use a .env.test file
/// - Example: TEST_DATABASE_URL=postgresql://user:pass@localhost/myapp_test
///
/// The test will clean up after itself by deleting the test user.
///
/// Flow tested:
/// 1. POST /api/auth/register
/// 2. GET  /api/auth/me (with access token)
/// 3. POST /api/auth/login
/// 4. POST /api/auth/refresh
/// 5. POST /api/auth/logout
/// 6. Attempt refresh with revoked token -> expect failure
/// 7. Cleanup test data
#[actix_web::test]
async fn auth_lifecycle() {
    // Load .env when present to help local dev
    let _ = dotenvy::dotenv();

    // Use TEST_DATABASE_URL if available, otherwise fall back to DATABASE_URL
    if let Ok(_) = env::var("TEST_DATABASE_URL") {
        eprintln!("Using TEST_DATABASE_URL for integration tests");
    } else if env::var("DATABASE_URL").is_ok() {
        eprintln!("WARNING: Using production DATABASE_URL for tests. Consider setting TEST_DATABASE_URL.");
    } else {
        eprintln!("Skipping integration test: Neither DATABASE_URL nor TEST_DATABASE_URL is set");
        return;
    }

    // Initialize DB connection using the application's shared postgres initializer.
    // NOTE: AuthConfig must be initialized before any auth operations.
    crud_with_sea_orm::shared::auth::AuthConfig::init();
    let db = match crud_with_sea_orm::shared::config::postgres::init_db().await {
        Ok(db) => db,
        Err(e) => panic!("failed to init db: {}", e),
    };

    // Run migrations to ensure schema is present.
    if let Err(e) = migration::Migrator::up(&db, None).await {
        panic!("migration failed: {}", e);
    }

    let state = web::Data::new(crud_with_sea_orm::shared::AppState::new(db));

    // Build the Actix app using the project's router configuration.
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .configure(crud_with_sea_orm::api::routes),
    )
    .await;

    // Use a unique email for each test run to avoid conflicts
    let test_email = format!("testuser_{}@example.com", chrono::Utc::now().timestamp());
    eprintln!("Test email: {}", test_email);

    // --- 1) Register ---
    let register_payload = serde_json::json!({
        "name": "Test User",
        "email": test_email,
        "password": "TestPass123!",
    });

    let req = test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&register_payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    if !status.is_success() {
        let body_bytes = test::read_body(resp).await;
        let body_str = String::from_utf8_lossy(&body_bytes);
        panic!("register failed: status {}, body: {}", status, body_str);
    }

    let body: Value = test::read_body_json(resp).await;
    let access_token = body["access_token"]
        .as_str()
        .expect("no access_token in register response")
        .to_string();
    let _refresh_token = body["refresh_token"]
        .as_str()
        .expect("no refresh_token in register response")
        .to_string();

    // --- 2) Call /me with access token ---
    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", access_token)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success(), "/me failed: {}", resp.status());
    let me_body: Value = test::read_body_json(resp).await;
    assert_eq!(
        me_body["email"].as_str().unwrap(),
        test_email,
        "unexpected email from /me"
    );

    // --- 3) Login ---
    let login_payload = serde_json::json!({
        "email": test_email,
        "password": "TestPass123!",
    });
    let req = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success(),
        "login failed: {}",
        resp.status()
    );
    let body: Value = test::read_body_json(resp).await;
    let access_token2 = body["access_token"]
        .as_str()
        .expect("no access_token from login")
        .to_string();
    let refresh_token2 = body["refresh_token"]
        .as_str()
        .expect("no refresh_token from login")
        .to_string();

    // --- 4) Refresh using refresh_token2 ---
    let refresh_payload = serde_json::json!({ "refresh_token": refresh_token2 });
    let req = test::TestRequest::post()
        .uri("/api/auth/refresh")
        .set_json(&refresh_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success(),
        "refresh failed: {}",
        resp.status()
    );
    let body: Value = test::read_body_json(resp).await;
    let new_access = body["access_token"]
        .as_str()
        .expect("no access_token in refresh response")
        .to_string();
    let new_refresh = body["refresh_token"]
        .as_str()
        .expect("no refresh_token in refresh response")
        .to_string();
    assert_ne!(new_access, access_token2, "new access token should differ");

    // --- 4.5) Verify OLD access token still works (this is expected behavior) ---
    // JWTs are stateless - old tokens remain valid until they expire
    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", access_token2)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_success(),
        "old access token should still work after refresh (JWTs are stateless)"
    );
    eprintln!("✓ Old access token still valid after refresh (expected - JWTs are stateless)");

    // --- 5) Logout using new_refresh ---
    let logout_payload = serde_json::json!({ "refresh_token": new_refresh });
    let req = test::TestRequest::post()
        .uri("/api/auth/logout")
        .set_json(&logout_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 204, "logout should return 204");

    // --- 6) Attempt to refresh with the same token should fail ---
    let req = test::TestRequest::post()
        .uri("/api/auth/refresh")
        .set_json(&logout_payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_client_error(),
        "refresh after logout should fail (got {})",
        resp.status()
    );

    // --- 7) Verify access token is also invalidated after logout ---
    let req = test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {}", new_access)))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(
        resp.status().is_client_error(),
        "access token should be invalidated after logout (got {})",
        resp.status()
    );
    eprintln!("✓ Access token properly invalidated after logout");

    // --- Cleanup: Delete test user and associated data ---
    use crud_with_sea_orm::api::users::repository::UserRepository;
    if let Ok(Some(user)) = UserRepository::find_by_email(&state.db, &test_email).await {
        let _ = UserRepository::delete(&state.db, user.id).await;
        eprintln!("✓ Test cleanup: deleted test user {}", test_email);
    }
}
