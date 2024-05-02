
fn html_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
    headers
}

fn json_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers
}

fn api_error(errmsg: &str) -> (HeaderMap, String) {
    (json_headers(), json!({"ret":1,"err":errmsg}).to_string())
}

fn api_ok() -> (HeaderMap, String){
    (json_headers(), json!({"ret":0,"ok":true}).to_string())
}

fn api_data_list(jsdts: Vec<Value>) -> (HeaderMap, String){
    let list = jsdts.iter().map(|a|a.to_string()).collect::<Vec<String>>().join(",");
    (json_headers(), format!(r#"{{"ret":0,"list":[{}]}}"#, list))
}
