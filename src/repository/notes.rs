use uuid::Uuid;

#[derive(serde::Deserialize, validator::Validate)]
pub struct CreateNoteRequest {
    #[validate(length(min = 1, max = 255))]
    title: String,
    #[validate(length(min = 1))]
    content: String,
}

pub async fn create_note(
    state: web::Data<AppState>,
    form: web::Json<CreatedNoteRequest>,
) -> Result<HttpResponse, ApiError> {
    form.validate()?;

    let note = sqlx::query_as!(
        Note,
        r#"
        INSERT INTO notes (title, content)
        VALUES ($1, $2)
        RETURNING *
        "#,
        form.title,
        form.content
    )
    .fetch_one(&state.db)
    .await?;

    Ok(HttpResponse::Created().json(note))
}

#[derive(sqlx::FromRow)]
struct PopularPost {
    title: String,
    like_count: i64,
}

async fn get_popular_post(pool: &PgPool, threshold: i64) -> Result<Vec<PopularPost>, sqlx::Error> {
    sqlx::query_as!(
        PopularPost,
        r#"
        SELECT p.title, p.like_count
        FROM posts p
        WHERE p.like_count > (
            SELECT AVG(like_count)
            FROM posts
            WHERE user_id = p.user_id -- Correlated
        )
        AND p.like_count > $1
        "#,
        threshold
    )
    .fetch_all(pool)
    .await
}


async fn get_user_with_posts(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT u.*
        FROM users u
        WHERE EXISTS (
            SELECT 1 FROM posts p
            WHERE p.user_id = u.id
            AND p.created_at > now() - interval '7days'
        )
        "#
    )
    .fetch_all(pool)
    .await
}