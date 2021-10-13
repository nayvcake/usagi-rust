use crate::http::client::create_request;

pub mod http;

pub fn dance() -> Option<&String> {
    return create_request("dance").unwrap().get("url");
}


pub fn feed() -> Option<&String> {
    return create_request("feed").unwrap().get("url");
}


pub fn kiss() -> Option<&String> {
    return create_request("kiss").unwrap().get("url");
}


pub fn pat() -> Option<&String> {
    return create_request("pat").unwrap().get("url");
}


pub fn poke() -> Option<&String> {
    return create_request("poke").unwrap().get("url");
}


pub fn slap() -> Option<&String> {
    return create_request("slap").unwrap().get("url");
}


pub fn tickle() -> Option<&String> {
    return create_request("tickle").unwrap().get("url");
}


pub fn bite() -> Option<&String> {
    return create_request("bite").unwrap().get("url");
}


pub fn bowdown() -> Option<&String> {
    return create_request("bowdown").unwrap().get("url");
}


pub fn wasted() -> Option<&String> {
    return create_request("wasted").unwrap().get("url");
}
