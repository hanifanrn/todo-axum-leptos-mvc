pub async fn hello_world() -> String {
    return "hello world from router".to_owned();
}

pub async fn mirror_body_string(body: String) -> String {
    return body;
}

pub async fn mirror_body_json() -> () {}
