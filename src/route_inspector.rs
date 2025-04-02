// src/route_inspector.rs
// This version doesn't try to detect conflicts since we're mounting by HTTP method
pub fn inspect_routes() {
    println!("\n=== ROUTE COUNTS BY HTTP METHOD ===");
    println!("GET routes: {}", crate::api::get_routes().len());
    println!("POST routes: {}", crate::api::post_routes().len());
    println!("PUT routes: {}", crate::api::put_routes().len());
    println!("DELETE routes: {}", crate::api::delete_routes().len());
    println!("=== ROUTE INSPECTION COMPLETE ===");
}