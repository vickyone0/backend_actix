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