

// paths
pub fn query(p: &str) -> String {
    "/query/".to_owned() + p
}

pub fn create(p: &str) -> String {
    "/create/".to_owned() + p
}

pub fn submit(p: &str) -> String {
    "/submit/".to_owned() + p
}

pub fn operate(p: &str) -> String {
    "/operate/".to_owned() + p
}



/**************************/




pub fn html_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/html; charset=utf-8".parse().unwrap());
    headers
}

pub fn json_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
    headers
}

pub fn api_error(errmsg: &str) -> (HeaderMap, String) {
    (json_headers(), json!({"ret":1,"err":errmsg}).to_string())
}

pub fn api_ok() -> (HeaderMap, String){
    (json_headers(), json!({"ret":0,"ok":true}).to_string())
}

pub fn api_data_list(jsdts: Vec<Value>) -> (HeaderMap, String){
    let list = jsdts.iter().map(|a|a.to_string()).collect::<Vec<String>>().join(",");
    (json_headers(), format!(r#"{{"ret":0,"list":[{}]}}"#, list))
}

pub fn api_data(jsdts: HashMap<&'static str, Value>) -> (HeaderMap, String){
    let resjson = jsdts.iter().map(|(k,v)|
        format!(r#""{}":{}"#, k, v.to_string())
    ).collect::<Vec<String>>().join(",");
    (json_headers(), format!(r#"{{"ret":0,{}}}"#, resjson))
}


/**************************/


#[macro_export]
macro_rules! jsondata{
    ( $( $key: expr, $dv: expr,)+ ) => (
        {
            let mut data = HashMap::<&'static str, Value>::new();
            $(
                data.insert($key, json!($dv));
            )+
            data
        }
    )
}

