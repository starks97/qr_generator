use actix_web::{delete, get, http, patch, post, web, HttpResponse, Responder};

use crate::{
    app_state::AppState,
    custom_error::{handle_validation_error, CustomError},
    models::qr_main_mods::{QRDataType, QrDataModel, QrDataPath, QrQuery},
};

#[post("/qr")]
pub async fn create_qr_data(
    body: web::Json<QRDataType>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
    if let Err(validation_error) = body.0.validate_inner() {
        return handle_validation_error(validation_error);
    }

    let data_type = body.0.data_type();

    let data_json = body
        .0
        .into_value_inner()
        .map_err(|e| CustomError::OtherError(e.to_string()))?;

    let new_data = match sqlx::query_as!(
        QrDataModel,
        r#"
            INSERT INTO qr_data (data_type, views, data_json)
            VALUES ($1, $2, $3)
            RETURNING id, data_type, views, data_json, created_at, updated_at
            "#,
        data_type,
        0,
        data_json
    )
    .fetch_one(&state.db)
    .await
    {
        Ok(data) => data,
        Err(sqlx::Error::Database(db_err))
            if db_err.constraint() == Some("qr_data_data_json_key") =>
        {
            return Err(CustomError::OtherError(
                "Data already exists, please provide new data".to_string(),
            ));
        }
        Err(e) => {
            println!("Error creating data: {:?}", e);
            return Err(CustomError::DataBaseError(e));
        }
    };

    Ok(HttpResponse::Created().json(serde_json::json!({
        "status": "success",
        "data": new_data
    })))
}

#[get("/qr")]
pub async fn get_all_qr_data(
    state: web::Data<AppState>,
    query: web::Query<QrQuery>,
) -> Result<HttpResponse, CustomError> {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(10);
    let data_type = query.data_type.clone();

    let qr_data = match data_type {
        Some(data_type) => {
            sqlx::query_as!(
                QrDataModel,
                r#"
                    SELECT id, data_type, views, data_json, created_at, updated_at
                    FROM qr_data
                    WHERE data_type = $1
                    ORDER BY created_at DESC
                    OFFSET $2 LIMIT $3
                "#,
                data_type.to_string(),
                offset,
                limit
            )
            .fetch_all(&state.db)
            .await
        }
        None => {
            sqlx::query_as!(
                QrDataModel,
                r#"
                    SELECT id, data_type, views, data_json, created_at, updated_at
                    FROM qr_data
                    ORDER BY created_at DESC
                    OFFSET $1 LIMIT $2
                "#,
                offset,
                limit
            )
            .fetch_all(&state.db)
            .await
        }
    };

    match qr_data {
        Ok(data) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "success",
            "data": data
        }))),
        Err(e) => {
            println!("Error retrieving data: {:?}", e);
            Err(CustomError::DataBaseError(e))
        }
    }
}

#[get("/qr/{qr_id}")]
pub async fn get_qr_data(
    data: web::Data<AppState>,
    path: web::Path<QrDataPath>,
) -> Result<HttpResponse, CustomError> {
    let qr_id = path.qr_id.clone();

    let qr_data = match sqlx::query_as!(
        QrDataModel,
        r#"
            UPDATE qr_data
            SET views = views + 1
            WHERE id = $1
            RETURNING id, data_type, views, data_json, created_at, updated_at
            "#,
        qr_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(data) => data,
        Err(e) => {
            println!("Error retrieving data: {:?}", e);
            return Err(CustomError::DataBaseError(e));
        }
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": qr_data
    })))
}

#[patch("/qr")]
pub async fn update_qr() -> Result<HttpResponse, CustomError> {}
