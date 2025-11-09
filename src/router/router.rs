use actix_web::web::{self, ServiceConfig};
use actix_files as fs;
use crate::service::{
    index_handler,
    get_turnstile,
    create_user_handler, 
    get_user_handler, 
    file_info_handler, 
    file_upload_handler, 
    file_list_handler, 
    file_delete_handler, 
    file_share_update_handler, 
    file_share_handler
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg
    .service(
        web::scope("/api/v1")
            .route("/user", web::get().to(get_user_handler))
            .route("/user", web::post().to(create_user_handler))
            .route("/content", web::get().to(file_list_handler))
            .route("/content", web::post().to(file_upload_handler))
            .route("/content/{file_id}", web::get().to(file_info_handler))
            .route("/content/{file_id}", web::delete().to(file_delete_handler))
            .route("/content/{file_id}/share", web::get().to(file_share_handler))
            .route("/content/{file_id}/share", web::put().to(file_share_update_handler))
            .route("/turnstile", web::get().to(get_turnstile))
    )
    .route("/dashboard", web::get().to(index_handler))
    .service(
        fs::Files::new("/dashboard", "./dist").show_files_listing()
    );
}