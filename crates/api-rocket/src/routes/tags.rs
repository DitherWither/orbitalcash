use db::{services::ApplicationService, models::Tag};
use rocket::{
    delete, get,
    http::Status,
    post, put,
    response::{content::RawJson, status},
    routes,
    serde::{
        self,
        json::{serde_json::json, Json},
    },
    Route, State,
};

use crate::guards::CurrentUser;

pub fn get_routes() -> Vec<Route> {
    routes![get_all, get_by_id, delete, create, update]
}

#[get("/")]
async fn get_all(
    user: CurrentUser,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service.tags.get_all(user.0).await {
        Ok(t) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success",
                    "tags": t
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!({
                "status": "error",
                "error_type": "database_error",
                "error": e.to_string()
            });
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}

#[get("/<tag_id>")]
async fn get_by_id(
    user: CurrentUser,
    app_service: &State<ApplicationService>,
    tag_id: i32,
) -> status::Custom<RawJson<String>> {
    match app_service.tags.get_by_id_checked(tag_id, user.0).await {
        Ok(Some(t)) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success",
                    "tag": t
                })
                .to_string(),
            ),
        ),
        Ok(None) => status::Custom(
            Status::NotFound,
            RawJson(
                json!({
                    "status": "error",
                    "error_type": "not_found",
                    "error": "Tag not found"
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!({
                "status": "error",
                "error_type": "database_error",
                "error": e.to_string()
            });
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateTag {
    tagname: String,
}

#[post("/", data = "<create_tag>")]
async fn create(
    user: CurrentUser,
    app_service: &State<ApplicationService>,
    create_tag: Json<CreateTag>,
) -> status::Custom<RawJson<String>> {
    match app_service
        .tags
        .create(create_tag.tagname.clone(), user.0)
        .await
    { 
        Ok(t) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success",
                    "tag": t
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!({
                "status": "error",
                "error_type": "database_error",
                "error": e.to_string()
            });
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}

#[put("/<tag_id>", data = "<tag>")]
async fn update(
    user: CurrentUser,
    tag_id: i32,
    tag: Json<Tag>, 
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    if tag.0.tag_id != tag_id {
        return status::Custom(
            Status::BadRequest,
            RawJson(
                json!({
                    "status": "error",
                    "error_type": "bad_request",
                    "error": "Tag id in path and body do not match"
                })
                .to_string(),
            ),
        );
    }

    match app_service.tags.update(tag.0, user.0).await {
        Ok(t) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success",
                    "tag": t
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!({
                "status": "error",
                "error_type": "database_error",
                "error": e.to_string()
            });
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}

#[delete("/<tag_id>")]
async fn delete(
    user: CurrentUser,
    tag_id: i32,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service.tags.delete_by_id_checked(tag_id, user.0).await {
        Ok(_) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success"
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!({
                "status": "error",
                "error_type": "database_error",
                "error": e.to_string()
            });
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}
