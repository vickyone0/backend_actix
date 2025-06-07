use validator::Validate;


struct SignupForm{
    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(
        min = 8,
        max = 32,
        message = "Password must be 8-32 characters"
    ))]
    password: String,

    #[validate(custom = "validate_age")]
    age: u8
}


fn validate_age(age: &u8) -> Result<(),validator::ValidationError> {
    if *age >= 13 {
         age
    }
    else {
        Err(ValidationError::new("under age"))
    }
}

//async validator

#[derive(Validate)]
struct UniqueEmail {
    #[validate(custom(function = "check_email_availability", use_context))]
    email: String,
}


async fn check_email_availability(
    email: &str,
    db: &PgPool
) -> Result<(), ValidationError> {
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
        email
    )
    .fetch_one(db)
    .await?;

    if exists {
        Err(ValidationError::new("Email already taken"))
    }
    else {
        Ok(())
    }
}

//Error Responses

#[derive(Debug, Serialize)]
struct ValidationErrorResponse {
    field: String,
    message: String,
    error_code: String,
}

impl From<ValidationErrors> for HttpResponse {
    fn from(errors : ValidationErrors) -> Self {
        let formatted_errors: Vec<_> = errors
            .field_errors()
            .iter()
            .map(|(field, err)| ValidationErrorResponse {
                field: field.to_string(),
                message: err[0].message.as_ref().unwrap().to_string(),
                error_code: "VALIDATION_FAILED".into(),
            })
            .collect();

        HttpResponse::UnprocessableEntity().json(formatted_errors)
    }
}