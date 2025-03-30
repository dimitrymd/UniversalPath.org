use rocket::{Route, get};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};
use crate::db::{UniversalPathDb, models::Term};

#[get("/glossary")]
async fn glossary(mut db: Connection<UniversalPathDb>) -> Template {
    // Get all letters
    let letters = match Term::get_all_letters(&mut db).await {
        Ok(letters) => letters,
        Err(_) => vec![],
    };

    // Get all terms
    let terms = match Term::find_all(&mut db).await {
        Ok(terms) => terms,
        Err(_) => vec![],
    };

    Template::render("glossary", context! {
        title: "Глоссарий",
        letters: &letters,
        terms: &terms,
    })
}

#[get("/glossary/<letter>")]
async fn glossary_by_letter(mut db: Connection<UniversalPathDb>, letter: String) -> Template {
    // Get all letters
    let letters = match Term::get_all_letters(&mut db).await {
        Ok(letters) => letters,
        Err(_) => vec![],
    };

    // Get terms for the selected letter
    let terms = match Term::find_by_letter(&mut db, &letter).await {
        Ok(terms) => terms,
        Err(_) => vec![],
    };

    Template::render("glossary", context! {
        title: format!("Глоссарий: {}", letter),
        current_letter: letter,
        letters: &letters,
        terms: &terms,
    })
}

#[get("/glossary/term/<id>")]
async fn view_term(mut db: Connection<UniversalPathDb>, id: u32) -> Template {
    // Get the term
    let term = match Term::find_by_id(&mut db, id).await {
        Ok(Some(term)) => term,
        Ok(None) => return Template::render("term_not_found", context! {
            title: "Термин не найден",
        }),
        Err(_) => return Template::render("term_not_found", context! {
            title: "Термин не найден",
        }),
    };

    // Get all letters for the navigation
    let letters = match Term::get_all_letters(&mut db).await {
        Ok(letters) => letters,
        Err(_) => vec![],
    };

    Template::render("term", context! {
        title: &term.title,
        term: &term,
        letters: &letters,
    })
}

pub fn routes() -> Vec<Route> {
    routes![glossary, glossary_by_letter, view_term]
}