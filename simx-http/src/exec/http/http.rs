use crate::common::hashmap_to_headerMap;
use engine_share::entity::exception::node::NodeError;
use engine_share::entity::flow::flow::FlowData;
use engine_share::entity::flow::node::Node;
use reqwest::Method;

pub async fn handler_http(node: Node, flow_data: &mut FlowData) -> Result<(), NodeError> {
    let handler_path: Vec<_> = node.handler.split(".").collect();
    match handler_path[2] {
        "get" => { request(node, flow_data, Method::GET).await }
        "post" => { request(node, flow_data, Method::POST).await }
        "put" => { request(node, flow_data, Method::PUT).await }
        "delete" => { request(node, flow_data, Method::DELETE).await }
        "options" => { request(node, flow_data, Method::OPTIONS).await }
        "trace" => { request(node, flow_data, Method::TRACE).await }
        "head" => { request(node, flow_data, Method::HEAD).await }
        _ => { Ok(()) }
    }
}

async fn request(node: Node, flow_data: &mut FlowData, method: Method) -> Result<(), NodeError> {
    let attr = node.attr;
    let url = attr.get("url").expect("url not found").as_str().expect("ip must be string");
    let header = attr.get("header").expect("header not found").as_object().expect("header must be object");
    let data = serde_json::to_string(attr.get("data").expect("data not found").as_object().expect("data must be object")).unwrap();
    let encoding = attr.get("encoding").expect("encoding not found").as_str().expect("encoding must be string");
    let client = reqwest::Client::new();
    // 组装header
    let headers = hashmap_to_headerMap(header);
    let response: String;
    // let resp = client.get(url).headers(headers).send().await.expect("request failed").text_with_charset(encoding);
    response = match method {
        Method::GET => { client.get(url).headers(headers).body(data).send().await.expect("request failed").text_with_charset(encoding).await.expect("parse failed") }
        Method::POST => { client.post(url).headers(headers).body(data).send().await.expect("request failed").text_with_charset(encoding).await.expect("parse failed") }
        Method::PUT => { client.put(url).headers(headers).body(data).send().await.expect("request failed").text_with_charset(encoding).await.expect("parse failed") }
        Method::DELETE => { client.delete(url).headers(headers).body(data).send().await.expect("request failed").text_with_charset(encoding).await.expect("parse failed") }
        _ => { return Ok(()) }
    };
    println!("{:?}", response);
    flow_data.json.insert(node.id.unwrap(), serde_json::to_value(response).unwrap());
    Ok(())
}
