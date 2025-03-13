// src/controllers/user_controller.rs
// Request Handlers
use crate::models::document::{CreateDocumentPayload, Document, UpdateDocumentPayload};
use crate::models::permission::{CreatePermissionPayload, UpdatePermissionPayload};
use crate::{Error, Result};
use axum::routing::{get, post, delete, put};
use axum::{
    extract::{Extension, Json, Path},
    Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;

/// GET handler for retrieving a document by ID.
/// Accessible via: GET /api/document/:id
pub async fn api_get_document(
    Path(document_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Document>> {
    println!("->> {:<12} - get_document", "HANDLER");

    let result = sqlx::query_as!(
        Document,
        r#"SELECT 
            id, 
            name, 
            content, 
            created_at, 
            updated_at, 
            user_id 
        FROM documents WHERE id = $1"#,
        document_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(document) => Ok(Json(document)),
        Err(_) => Err(Error::UserNotFoundError),
    }
}

/// POST handler for creating a new document.
/// Accessible via: POST /api/document
pub async fn api_create_document(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateDocumentPayload>,
) -> Result<Json<Document>> {
    println!("->> {:<12} - create_document", "HANDLER");

    // First insert the document
    let result = sqlx::query!(
        "INSERT INTO documents (name, content, user_id, created_at, updated_at) 
         VALUES ($1, $2, $3, $4, $5) RETURNING id",
        payload.name,
        payload.content,
        payload.user_id,
        payload.created_at,
        payload.updated_at
    )
    .fetch_one(&pool)
    .await;

    // Check if insertion was successful
    match result {
        Ok(record) => {
            // Then fetch the document by id
            let document = sqlx::query_as!(
                Document,
                r#"SELECT 
                    id, 
                    name, 
                    content,
                    created_at,
                    updated_at,
                    user_id 
                FROM documents WHERE id = $1"#,
                record.id
            )
            .fetch_one(&pool)
            .await;

            match document {
                Ok(document) => Ok(Json(document)),
                Err(e) => {
                    println!("Error fetching user: {:?}", e);
                    Err(Error::DocumentNotFoundError)
                }
            }
        }
        Err(e) => {
            println!("Error creating user: {:?}", e);
            Err(Error::UserCreationError)
        }
    }
}

async fn check_document_permission(
    pool: &PgPool,
    user_id: i32,
    document_id: i32,
    required_role: &str,
) -> Result<bool> {
    let result = sqlx::query!(
        r#"SELECT role FROM document_permissions 
           WHERE document_id = $1 AND user_id = $2"#,
        document_id,
        user_id
    )
    .fetch_optional(pool)
    .await;

    match result {
        Ok(Some(record)) => {
            let has_permission = match required_role {
                "viewer" => true, // Any role can view
                "editor" => record.role == "editor" || record.role == "owner",
                "owner" => record.role == "owner",
                _ => false,
            };

            Ok(has_permission)
        }
        Ok(None) => Ok(false),
        Err(e) => {
            println!("Error checking permission: {:?}", e);
            Err(Error::PermissionError)
        }
    }
}

pub async fn api_update_document(
    Path(document_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<UpdateDocumentPayload>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - update_document", "HANDLER");

    // Get user_id from token (for now hardcoded)
    let user_id = 1;

    // Check if user has editor or owner permission
    let has_permission = check_document_permission(&pool, user_id, document_id, "editor").await?;

    if !has_permission {
        return Err(Error::PermissionDeniedError);
    }

    // Proceed with update...
    let result = sqlx::query!(
        "UPDATE documents
        SET name = $1, content = $2, updated_at = $3
        WHERE id = $4",
        payload.name,
        payload.content,
        payload.updated_at,
        document_id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(json!({
            "result": {
                "success": true
            }
        }))),
        Err(e) => {
            println!("Error updating document: {:?}", e);
            Err(Error::DocumentUpdateError)
        }
    }
}

/// Grant permission to a user for a document
pub async fn grant_document_permission(Path(document_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreatePermissionPayload>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - grant_document_permission", "HANDLER");

    // First check if the current user has owner permission
    // (This would be implemented with cookie/auth check)
    let user_id = 1; // for now we will hardcode and use user 1

    let has_permission = check_document_permission(&pool, user_id, document_id, "owner").await?;

    if !has_permission {
        return Err(Error::PermissionError)
    }

    // Insert the permission
    let result = sqlx::query!(
        "INSERT INTO document_permissions (document_id, user_id, role)
        VALUES ($1, $2, $3)
        ON CONFLICT (document_id, user_id) 
        DO UPDATE SET role = $3",
        document_id,
        payload.user_id,
        payload.role
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(Json(json!({
            "result": {
                "success": true
            }
        }))),
        Err(e) => {
            println!("Error granting permission: {:?}", e);
            Err(Error::PermissionError)
        }
    }
}

/// Get all users with access to a document
pub async fn get_document_users(
    Path(document_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - get_document_users", "HANDLER");

    // need to check if current logged in user can view this type of information
    // hardcode user_id 1 for now
    let user_id = 1;

    let permissions = check_document_permission(&pool, user_id, document_id, "viewer").await?;

    if !permissions {
        return Err(Error::PermissionError)
    }
    let result = sqlx::query!(
        r#"SELECT dp.user_id, u.name, u.email, dp.role 
           FROM document_permissions dp
           JOIN users u ON dp.user_id = u.id
           WHERE dp.document_id = $1"#,
        document_id
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(users) => {
            let users_json: Vec<Value> = users
                .into_iter()
                .map(|u| {
                    json!({
                        "user_id": u.user_id,
                        "name": u.name,
                        "email": u.email,
                        "role": u.role
                    })
                })
                .collect();

            Ok(Json(json!({ "users": users_json })))
        }
        Err(e) => {
            println!("Error fetching document users: {:?}", e);
            Err(Error::DocumentNotFoundError)
        }
    }
}

/// Update a user's permission for a document
pub async fn update_document_permission(
    Path(document_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<UpdatePermissionPayload>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - update_document_permission", "HANDLER");
    
    // Get user_id from auth (for now hardcoded)
    let user_id = 1;
    
    // Check if user has owner permission
    let has_permission = check_document_permission(&pool, user_id, document_id, "owner").await?;
    
    if !has_permission {
        return Err(Error::PermissionDeniedError);
    }
    
    // Update the permission
    let result = sqlx::query!(
        "UPDATE document_permissions 
         SET role = $1
         WHERE document_id = $2 AND user_id = $3",
        payload.role,
        document_id,
        payload.user_id  // The user whose permission is being updated
    )
    .execute(&pool)
    .await;
    
    match result {
        Ok(_) => Ok(Json(json!({
            "result": {
                "success": true,
                "message": "Permission updated successfully"
            }
        }))),
        Err(e) => {
            println!("Error updating permission: {:?}", e);
            Err(Error::PermissionError)
        }
    }
}

/// Remove a user's permission for a document
pub async fn remove_document_permission(
    Path((document_id, target_user_id)): Path<(i32, i32)>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - remove_document_permission", "HANDLER");
    
    // Get user_id from auth (for now hardcoded)
    let user_id = 1;
    
    // Check if user has owner permission
    let has_permission = check_document_permission(&pool, user_id, document_id, "owner").await?;
    
    if !has_permission {
        return Err(Error::PermissionDeniedError);
    }
    
    // Prevent removing the last owner
    let owners_count_result = sqlx::query!(
        "SELECT COUNT(*) as count FROM document_permissions 
         WHERE document_id = $1 AND role = 'owner'",
        document_id
    )
    .fetch_one(&pool)
    .await;
    
    let is_target_owner = sqlx::query!(
        "SELECT role FROM document_permissions 
         WHERE document_id = $1 AND user_id = $2",
        document_id,
        target_user_id
    )
    .fetch_optional(&pool)
    .await;
    
    // If we're removing an owner and there's only one owner, prevent it
    if let (Ok(owners_count), Ok(Some(record))) = (&owners_count_result, &is_target_owner) {
        if record.role == "owner" && owners_count.count.unwrap_or(0) <= 1 {
            return Err(Error::PermissionDeniedError);
        }
    }
    
    // Remove the permission
    let result = sqlx::query!(
        "DELETE FROM document_permissions 
         WHERE document_id = $1 AND user_id = $2",
        document_id,
        target_user_id
    )
    .execute(&pool)
    .await;
    
    match result {
        Ok(_) => Ok(Json(json!({
            "result": {
                "success": true,
                "message": "Permission removed successfully"
            }
        }))),
        Err(e) => {
            println!("Error removing permission: {:?}", e);
            Err(Error::PermissionError)
        }
    }
}

pub fn doc_routes() -> Router {
    Router::new()
        .route("/", post(api_create_document))
        .route("/:id", get(api_get_document))
        .route("/:id", post(api_update_document))
        .route("/:id/permissions", get(get_document_users))
        .route("/:id/permissions", post(grant_document_permission))
        .route("/:id/permissions/:user_id", delete(remove_document_permission))
        .route("/:id/permissions", put(update_document_permission))
}