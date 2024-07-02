use serde::Deserialize;
use serde_partial_deserialization::ApiResponse;

#[derive(Deserialize)]
struct Data {
    some_data: i32,
}

fn main() {
    let successful_json = r#"
        {
            "status": "success",
            "message": "",
            "data": {"some_data": 42}
        }"#;

    let unsuccessful_json = r#"
        {
            "status": "error",
            "message": "error message",
            "data": {"unknown_field": "some_string"}
        }"#;

    let _api_response: (ApiResponse<Data>, _) =
        serde_json_core::from_slice(successful_json.as_bytes())
            .expect("Body is a 'Data' structure");

    let result: Result<(ApiResponse<Data>, usize), _> =
        serde_json_core::from_slice(unsuccessful_json.as_bytes());

    match result {
        Ok((a, _)) => println!("Status: {:?};; {}", a.status, a.message),
        Err(e) => println!("Error: {e:?};; {e}"),
    }
}
