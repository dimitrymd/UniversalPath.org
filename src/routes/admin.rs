use rocket::{Route, get, post, uri};
use rocket::response::{Redirect, Flash};
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::{UniversalPathDb, models::{User, LoginUser}};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(FromForm)]
pub struct AdminLogin {
    username: String,
    password: String,
}

#[get("/login")]
fn login_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    // If the user is already logged in, redirect to the dashboard
    if cookies.get_private("admin_token").is_some() {
        return Err(Redirect::to(uri!(dashboard)));
    }

    Ok(Template::render("admin/login", context! {
        title: "Admin Login",
    }))
}

#[post("/login", data = "<login>")]
async fn login_submit(mut db: Connection<UniversalPathDb>, cookies: &CookieJar<'_>, login: Form<AdminLogin>) -> Result<Redirect, Flash<Redirect>> {
    let login_user = LoginUser {
        username: login.username.clone(),
        password: login.password.clone(),
    };

    match User::login(&mut db, &login_user).await {
        Ok(Some(user)) => {
            if !user.is_admin {
                return Err(Flash::error(
                    Redirect::to(uri!(login_page)),
                    "У вас нет прав администратора.",
                ));
            }

            let username = user.username.clone();
                
                // Generate a JWT token
            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() + 24 * 3600; // Token valid for 24 hours

            let claims = crate::db::models::UserToken {
                id: user.id,
                username: username.clone(),
                is_admin: user.is_admin,
                exp: expiration,
            };

            let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_key".to_string());
            
            match encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            ) {
                Ok(token) => {
                    cookies.add_private(Cookie::new("admin_token", token));
                    cookies.add_private(Cookie::new("admin_user", username));
                    Ok(Redirect::to(uri!(dashboard)))
                },
                Err(_) => Err(Flash::error(
                    Redirect::to(uri!(login_page)),
                    "Не удалось создать сессию. Пожалуйста, попробуйте снова.",
                )),
            }
        },
        Ok(None) => Err(Flash::error(
            Redirect::to(uri!(login_page)),
            "Неверное имя пользователя или пароль.",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(login_page)),
            "Произошла ошибка при входе. Пожалуйста, попробуйте снова.",
        )),
    }
}

#[get("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private(Cookie::from("admin_token"));
    cookies.remove_private(Cookie::from("admin_user"));
    
    Flash::success(
        Redirect::to(uri!(login_page)),
        "Вы успешно вышли из системы.",
    )
}

#[get("/")]
fn dashboard(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    if cookies.get_private("admin_token").is_none() {
        return Err(Redirect::to(uri!(login_page)));
    }

    let username = cookies.get_private("admin_user")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| "Administrator".to_string());

    Ok(Template::render("admin/dashboard", context! {
        title: "Admin Dashboard",
        username: username,
    }))
}

#[get("/articles")]
fn articles_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    if cookies.get_private("admin_token").is_none() {
        return Err(Redirect::to(uri!(login_page)));
    }

    let username = cookies.get_private("admin_user")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| "Administrator".to_string());

    Ok(Template::render("admin/articles", context! {
        title: "Управление статьями",
        username: username,
    }))
}

#[get("/categories")]
fn categories_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    if cookies.get_private("admin_token").is_none() {
        return Err(Redirect::to(uri!(login_page)));
    }

    let username = cookies.get_private("admin_user")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| "Administrator".to_string());

    Ok(Template::render("admin/categories", context! {
        title: "Управление категориями",
        username: username,
    }))
}

#[get("/terms")]
fn terms_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    if cookies.get_private("admin_token").is_none() {
        return Err(Redirect::to(uri!(login_page)));
    }

    let username = cookies.get_private("admin_user")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| "Administrator".to_string());

    Ok(Template::render("admin/terms", context! {
        title: "Управление терминами",
        username: username,
    }))
}

#[get("/users")]
fn users_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    if cookies.get_private("admin_token").is_none() {
        return Err(Redirect::to(uri!(login_page)));
    }

    let username = cookies.get_private("admin_user")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| "Administrator".to_string());

    Ok(Template::render("admin/users", context! {
        title: "Управление пользователями",
        username: username,
    }))
}

#[get("/apikeys")]
fn apikeys_page(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    if cookies.get_private("admin_token").is_none() {
        return Err(Redirect::to(uri!(login_page)));
    }

    let username = cookies.get_private("admin_user")
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| "Administrator".to_string());

    Ok(Template::render("admin/apikeys", context! {
        title: "Управление API ключами",
        username: username,
    }))
}

pub fn routes() -> Vec<Route> {
    routes![
        login_page,
        login_submit,
        logout,
        dashboard,
        articles_page,
        categories_page,
        terms_page,
        users_page,
        apikeys_page,
    ]
}