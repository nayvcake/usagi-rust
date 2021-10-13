use crate::http::client::create_request;


// API Client
pub mod http;

pub fn dance() -> String {
    return create_request("dance").unwrap().get("url").unwrap().to_string();
}


pub fn feed() -> String {
    return create_request("feed").unwrap().get("url").unwrap().to_string();
}


pub fn hug() -> String {
    return create_request("hug").unwrap().get("url").unwrap().to_string();
}


pub fn kiss() -> String {
    return create_request("kiss").unwrap().get("url").unwrap().to_string();
}


pub fn pat() -> String {
    return create_request("pat").unwrap().get("url").unwrap().to_string();
}


pub fn poke() -> String {
    return create_request("poke").unwrap().get("url").unwrap().to_string();
}


pub fn slap() -> String {
    return create_request("slap").unwrap().get("url").unwrap().to_string();
}


pub fn tickle() -> String {
    return create_request("tickle").unwrap().get("url").unwrap().to_string();
}


pub fn bite() -> String {
    return create_request("bite").unwrap().get("url").unwrap().to_string();
}


pub fn bowdown() -> String {
    return create_request("bowdown").unwrap().get("url").unwrap().to_string();
}


pub fn wasted() -> String {
    return create_request("wasted").unwrap().get("url").unwrap().to_string();
}