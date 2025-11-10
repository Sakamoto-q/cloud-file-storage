use actix_web::web::{self, ServiceConfig};
use actix_files as fs;
use crate::service::{
    index_handler,
    get_turnstile,
    create_user_handler, 
    session_info_handler,
    logout_handler,
    login_handler,
    get_file_details_handler, 
    create_file_handler, 
    list_files_handler, 
    delete_file_handler, 
    update_file_access_handler, 
    get_download_url_handler
};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg
    .service(
        web::scope("/api/v1")
            .route("/user", web::post().to(create_user_handler))
            .route("/session", web::get().to(session_info_handler))
            .route("/session", web::post().to(login_handler))
            .route("/session/{session_id}", web::delete().to(logout_handler))
            .route("/content", web::get().to(list_files_handler))
            .route("/content", web::post().to(create_file_handler))
            .route("/content/{file_id}", web::get().to(get_file_details_handler))
            .route("/content/{file_id}", web::delete().to(delete_file_handler))
            .route("/content/{file_id}/share", web::get().to(get_download_url_handler))
            .route("/content/{file_id}/share", web::put().to(update_file_access_handler))
            .route("/turnstile", web::get().to(get_turnstile))
    )
    .route("/dashboard", web::get().to(index_handler))
    .service(
        fs::Files::new("/dashboard", "./dist").show_files_listing()
    );
}