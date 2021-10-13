
use std::collections::HashMap;



pub fn create_request(route: &str) -> reqwest::Result<HashMap<String, String>> {
    let url = format!("https://usagiapi.danielagc.repl.co/api/{}", route);
    let resp = reqwest::blocking::get(url)?
        .json::<HashMap<String, String>>();
    return resp;
}
