// src/debug_routes.rs
use rocket::Route;

pub fn print_routes(routes: &[Route]) {
    for route in routes {
        println!("Method: {:?}, URI: {}", route.method, route.uri);
    }
}

pub fn debug_routes() {
    println!("\n=== WEB ROUTES ===");
    let web_routes = crate::routes::web_routes();
    print_routes(&web_routes);
    
    // Print API routes by HTTP method
    println!("\n=== API GET ROUTES ===");
    let api_get_routes = crate::api::get_routes();
    print_routes(&api_get_routes);
    
    println!("\n=== API POST ROUTES ===");
    let api_post_routes = crate::api::post_routes();
    print_routes(&api_post_routes);
    
    println!("\n=== API PUT ROUTES ===");
    let api_put_routes = crate::api::put_routes();
    print_routes(&api_put_routes);
    
    println!("\n=== API DELETE ROUTES ===");
    let api_delete_routes = crate::api::delete_routes();
    print_routes(&api_delete_routes);
    
    println!("\n=== ADMIN ROUTES ===");
    let admin_routes = crate::routes::admin_routes();
    print_routes(&admin_routes);
}