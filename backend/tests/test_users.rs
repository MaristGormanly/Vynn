/*
 / HOW TO USE BACKEND TESTS
 / ENSURE WATCH IS INSTALLED '$ cargo install cargo-watch --locked'
 / In Terminal 1: 'cargo watch -q -c -w src/ -x run'
 / In Terminal 2: 'cargo watch -q -c -w tests/ -x "test -q test_testname -- --nocapture"'
 / Now you can see LIVE Updates of API calls
*/

#![allow(unused)]

use std::any;

use anyhow::Result;
use axum::http::response;
use backend::result_to_string;
use httpc_test::Client;
use serde_json::json;

#[tokio::test]
async fn test_users() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3001")?;

    println!("\n===== RUNNING USER API TESTS =====\n");

    // Run all tests and collect results
    let user_result = test_create_user(&hc).await;
    let good_login_result = test_good_login(&hc).await;
    let bad_login_result = test_bad_login(&hc).await;
    let get_user_result = test_get_user(&hc).await;
    let logout_result = test_logout(&hc).await;
    let wipe_db_result = backend::test_wipe_db(&hc).await;

    // Print summary
    println!("\n===== TEST RESULTS =====");
    println!("User Creation: {}", result_to_string(&user_result));
    println!("Good Login: {}", result_to_string(&good_login_result));
    println!("Bad Login: {}", result_to_string(&bad_login_result));
    println!("Get User: {}", result_to_string(&get_user_result));
    println!("Logout: {}", result_to_string(&logout_result));
    println!("Wipe Database: {}", result_to_string(&wipe_db_result));
    println!("=====================\n");

    Ok(())
}

async fn test_create_user(hc: &Client) -> Result<()> {
    println!("TEST - User Creation");
    let create_response = hc
        .do_post(
            "/api/users",
            json!({
                "name": "Test User",
                "email": "testcreate@example.com",
                "password": "password123"
            }),
        )
        .await?;

    create_response.print().await?;

    if !create_response.status().is_success() {
        return Err(anyhow::anyhow!(
            "User creation failed with status: {}",
            create_response.status()
        ));
    }

    let body = create_response
        .json_body()
        .expect("Failed to get JSON body");
    let user_id = body["id"].as_i64().unwrap_or(1);

    let get_response = hc.do_get(&format!("/api/users/{}", user_id)).await?;
    get_response.print().await?;

    if !get_response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to get created user"));
    }

    Ok(())
}

//async fn test_update_user(hc: &Client)

async fn test_good_login(hc: &Client) -> Result<()> {
    print!("TEST - Good Login");
    let response = hc
        .do_post(
            "/api/login",
            json!({
                "email": "testcreate@example.com",
                "password": "password123"
            }),
        )
        .await?;
    response.print().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Login failed with status: {}",
            response.status()
        ));
    }

    Ok(())
}

async fn test_bad_login(hc: &Client) -> Result<()> {
    print!("TEST - Bad Login");
    let response = hc
        .do_post(
            "/api/login",
            json!({
                "email": "Hell2o@gmail.com",
                "password": "bad password"
            }),
        )
        .await?;
    response.print().await?;

    if response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Bad login Succeed with status: {}",
            response.status()
        ));
    }

    Ok(())
}

async fn test_get_user(hc: &Client) -> Result<()> {
    print!("TEST - Get User");
    let response = hc.do_get("/api/users/1").await?;
    response.print().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Get User failed with status: {}",
            response.status()
        ));
    }

    Ok(())
}

async fn test_logout(hc: &Client) -> Result<()> {
    print!("TEST - Logout");
    let response = hc.do_get("/api/users/logout").await?;
    response.print().await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Logout failed with status: {}",
            response.status()
        ));
    }

    Ok(())
}
