use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health,
        crate::handlers::gifts::create_gift,
        crate::handlers::gifts::list_gifts,
        crate::handlers::gifts::get_gift,
        crate::handlers::gifts::update_gift,
        crate::handlers::gifts::delete_gift,
        crate::handlers::users::upsert_user,
        crate::handlers::users::get_user,
        crate::handlers::orders::purchase,
        crate::handlers::orders::get_order,
        crate::handlers::orders::update_order_status,
    ),
    components(
        schemas(
            crate::models::HealthResponse,
            crate::models::Gift,
            crate::models::CreateGiftRequest,
            crate::models::UpdateGiftRequest,
            crate::models::User,
            crate::models::UpsertUserRequest,
            crate::models::OrderStatus,
            crate::models::Order,
            crate::models::PurchaseRequest,
            crate::models::PurchaseResponse,
            crate::models::UpdateOrderStatusRequest,
            crate::handlers::users::UpsertUserResponse,
            crate::auth::AuthTokenResponse,
            crate::errors::ApiErrorBody,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Health", description = "Service health checks"),
        (name = "Gifts", description = "NFT gift catalog management"),
        (name = "Users", description = "Telegram user management"),
        (name = "Orders", description = "Gift purchasing and order lifecycle")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}
