use actix_web::{HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

pub async fn create_user(
    pool: web::Data<PgPool>,
    form: web::Json<CreateUser>,
) -> Result<impl Responder> {
    form.validate()?;

    //Password hashing 
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(form.password.as_bytes(), &salt)?.to_string();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, username, email,  password_hash, created_at
        "#,
        form.username,
        form.email,
        hash
    )
    .fetch_one(pool.as_ref())
    .await?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn get_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<impl Responder> {
    let user = sqlx::query_as!(
        User,
        r#"SELECT id, username, email, password_hash, created_at
           FROM users WHERE id = $1"#,
        user_id.into_inner()   
    )
    .fetch_optional(pool.as_ref())
    .await?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(ErrorNotFound("User not found")),
    }
}


//paginated list

pub async fn list_users(
    pool: web::Data<PgPool>,
    query: web::Query<Pagination>,
) -> Result<impl Responder> {
    let users = sqlx::query_as!(
        User,
        r#"SELECT id, username, email, password_hash, created_at
           FROM users
           ORDER BY created_at DESC
           LIMIT $1 OFFSET $2"#,
        query.limit.unwarp_or(20),
        query.offset.unwrap_or(0)   
    )
    .fetch_all(pool.as_ref())
    .await?;

    Ok(Json(users))
}



#[derive(Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 3, max = 24))]
    pub username: Option<String>,

    #[validate(email)]
    pub email: Option<String>
}

pub async fn update_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
    form: web::Json<UpdateUser>,
) -> Result<impl Responder> {
    form.validate()?;

    let user = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET
            username = COALESCE($1, username),
            email = COALESCE($2, email)
        WHERE id = $3
        RETURING id, username, email, password_hash, created_at
        "#,
        form.username,
        form.email,
        user_id.into_inner()
    )
    .fetch_optional(pool.as_ref())
    .await?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(ErrorNotFound("User not found")),
    }
}


//delete user
pub async fn delete_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<Uuid>,
) -> Result<impl Responder> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id.into_inner()
    )
    .execute(pool.as_ref())
    .await?;

    if result.rows_affected() == 0 {
        return Err(ErrorNotFound("User not found"));
    }
    Ok(HttpResponse::NoContent())
}