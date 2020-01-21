
extern crate reqwest;

use std::collections::HashMap;

const ENGINE : &str = "/engine";

pub struct WorkflowEngine {
    base_url : String,
}

impl WorkflowEngine {

    pub fn from_url(url: &str) -> WorkflowEngine {

        WorkflowEngine { base_url: String::from(url) }
    }

    pub fn get_engine(&self) -> Result<Vec<HashMap<String, String>>, String> {

        let base_url = self.base_url.to_string();
        let engine_context = ENGINE.to_string();
        let resource_url = base_url + &engine_context;

        let response_result = reqwest::blocking::get(resource_url.as_str());

        if response_result.is_err() {

            return Err(String::from("can't get workflow engine"));
        }
    
        let response = response_result
            .unwrap();

        if response.status().is_success() {
    
            let engine_list = response.json::<Vec<HashMap<String, String>>>();

            return engine_list.map_err(| err | err.to_string());
        }
    
        let error_message = format!("[http error code: {}]", response.status().to_string());
        Err(error_message)
    }
    
}

// -----------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    extern crate httpmock;

    use httpmock::Method::GET;
    use httpmock::{mock, with_mock_server};

    use super::*;

    const TEST_ENGINE_URL : &str = "http://localhost:5000/rest";

    #[test]
    #[with_mock_server]
    fn get_engine_success_test() {

        let engine_mock = mock(GET, "/rest/engine")
            .return_status(200)
            .return_header("ContentType", "application/json")
            .return_body("[{\"name\":\"default\"}]")
            .create();

        let engine : WorkflowEngine = WorkflowEngine::from_url(TEST_ENGINE_URL);

        let result = engine.get_engine();

        assert!(result.is_ok());
        assert_eq!(engine_mock.times_called(), 1);
    }

    #[test]
    #[with_mock_server]
    fn get_engine_not_found_test() {

        let engine_mock = mock(GET, "/rest/engine")
            .return_status(404)
            .create();

        let engine : WorkflowEngine = WorkflowEngine::from_url(TEST_ENGINE_URL);

        let result = engine.get_engine();

        assert!(result.is_err());
        assert_eq!(result.err(), Some("[http error code: 404 Not Found]".to_string()));        
        assert_eq!(engine_mock.times_called(), 1);
    }
}
