use crate::handlers::{
    admin, aplikasi, auth, country_code, dashboard, gateway, groups, provider_prefixes, sms_costs,
    sms_template, users,
};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    .route("", web::get().to(users::get_users))
                    .route("", web::post().to(users::create_user))
                    .route("/{id}", web::put().to(users::update_user))
                    .route("/{id}", web::delete().to(users::delete_user)),
            )
            .service(
                web::scope("/groups")
                    .route("", web::get().to(groups::get_groups))
                    .route("", web::post().to(groups::create_group))
                    .route("/{id}", web::put().to(groups::update_group))
                    .route("/{id}", web::delete().to(groups::delete_group)),
            )
            .service(web::scope("/auth").route("/login", web::post().to(auth::login)))
            .service(
                web::scope("/dashboard").route("/summary", web::get().to(dashboard::get_summary)),
            )
            .service(
                web::scope("/admin")
                    .route("", web::get().to(admin::get_admin))
                    .route("", web::post().to(admin::create_admin)),
            )
            .service(
                web::scope("/sms-template")
                    .route("", web::post().to(sms_template::create_template_sms))
                    .route("", web::get().to(sms_template::get_template))
                    .route("/{id}", web::put().to(sms_template::update_sms_template))
                    .route("/{id}", web::delete().to(sms_template::delete_sms_template)),
            )
            .service(
                web::scope("/provider-prefixes")
                    .route(
                        "",
                        web::post().to(provider_prefixes::create_provider_prefixes),
                    )
                    .route("", web::get().to(provider_prefixes::get_provider_prefixes))
                    .route(
                        "/{id}",
                        web::put().to(provider_prefixes::update_provider_prefixes),
                    )
                    .route(
                        "/{id}",
                        web::delete().to(provider_prefixes::delete_provider_prefixes),
                    ),
            )
            .service(
                web::scope("/sms-cost")
                    .route("", web::get().to(sms_costs::get_sms_cost))
                    .route("", web::post().to(sms_costs::create_sms_cost))
                    .route("/{id}", web::put().to(sms_costs::update_sms_cost))
                    .route("/{id}", web::delete().to(sms_costs::delete_sms_cost)),
            )
            .service(
                web::scope("/country-codes")
                    .route("", web::post().to(country_code::create_country_code))
                    .route("", web::get().to(country_code::get_country_code))
                    .route("/{id}", web::put().to(country_code::update_country_code))
                    .route("/{id}", web::delete().to(country_code::delete_country_code)),
            )
            .service(
                web::scope("/aplikasi")
                    .route("", web::post().to(aplikasi::create_aplikasi))
                    .route("", web::get().to(aplikasi::get_aplikasi))
                    .route("/{id}", web::put().to(aplikasi::update_aplikasi))
                    .route("/{id}", web::delete().to(aplikasi::delete_aplikasi)),
            )
            .service(
                web::scope("/gateway")
                    .route("", web::post().to(gateway::create_gateway))
                    .route("", web::get().to(gateway::get_gateway))
                    .route("/{id}", web::put().to(gateway::update_gateway))
                    .route("/{id}", web::delete().to(gateway::delete_gateway)),
            ),
    );
}
