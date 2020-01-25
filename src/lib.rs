
extern crate reqwest;
extern crate serde;

use std::collections::HashMap;
use serde::{ Deserialize };

const ENGINE : &str = "/engine";
const PROCESS_DEFINITION : &str = "/process-definition";

// Structs definition

pub struct WorkflowEngine {
    base_url : String,
}

#[derive(Deserialize)]
pub struct ProcessDefinition {
    id: Option<String>,
    key: Option<String>,
    category: Option<String>,
    description: Option<String>,
    name: Option<String>,
    version: u16,
    resource: Option<String>,
    #[serde(rename="deploymentId")]
    deployment_id: Option<String>,
    diagram: Option<String>,
    suspended: bool,
    #[serde(rename="tenantId")]
    tenant_id: Option<String>,
    #[serde(rename="versionTag")]
    version_tag: Option<String>,
    #[serde(rename="historyTimeToLive")]
    history_time_to_live: Option<String>,
    #[serde(rename="startableInTasklist")]
    startable_in_tasklist: bool
}

impl WorkflowEngine {

    pub fn from_url(url: &str) -> WorkflowEngine {

        WorkflowEngine { base_url: String::from(url) }
    }

    pub fn get_engine(&self) -> Result<Vec<HashMap<String, String>>, String> {

        let resource_url = self.build_resource_uri(ENGINE);

        let response_result = reqwest::blocking::get(resource_url.as_str());

        if response_result.is_err() {

            return Err("can't get workflow engine".to_string());
        }
    
        let response = response_result
            .unwrap();

        if response.status().is_success() {
    
            return self.deserialize_response::<Vec<HashMap<String, String>>>(response);
        }
    
        let error_message = format!("[http error code: {}]", response.status().to_string());
        Err(error_message)
    }

    pub fn get_process_definitions(&self) -> Result<Vec<ProcessDefinition>, String> {

        let resource_url = self.build_resource_uri(PROCESS_DEFINITION);

        let response_result = reqwest::blocking::get(resource_url.as_str());

        if response_result.is_err() {

            return Err("can't get process definitions".to_string());
        }

        let response = response_result.unwrap();

        let response_status = response.status();

        if response_status.is_success() {

            return self.deserialize_response::<Vec<ProcessDefinition>>(response);
        }

        let error_message = format!("[http error code: {}]", response_status.to_string());
        Err(error_message)
    }

    // ------------------------------------------------------------------

    fn build_resource_uri(&self, resource: &str) -> String {

        let base_url = self.base_url.to_string();
        let engine_context = resource.to_string();
        
        return base_url + &engine_context;
    }

    fn deserialize_response<T: serde::de::DeserializeOwned>(&self, response : reqwest::blocking::Response) -> Result<T, String> {

        let result = response.json::<T>();

        return result
            .map_err(| err | err.to_string());    
    }
}

// -----------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    extern crate httpmock;

    use httpmock::Method::GET;
    use httpmock::{mock, with_mock_server};

    use std::fs;
    
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

    #[test]
    #[with_mock_server]
    fn get_process_definitions_test() {

        let process_definitions = read_sample("process-definition-response.json");

        let engine_mock = mock(GET, "/rest/process-definition")
            .return_status(200)
            .return_header("ContentType", "application/json")
            .return_body(process_definitions.as_str())
            .create();

        let engine : WorkflowEngine = WorkflowEngine::from_url(TEST_ENGINE_URL);

        let result = engine.get_process_definitions();
		
        assert!(result.is_ok());
        assert_eq!(engine_mock.times_called(), 1);
    }

    // Fixtures
    // -----------------------------------------------------------------------------

    fn read_sample(sample_name: &str) -> String {
        
        let resource_dir = format!("resources/test/samples/{}", sample_name);

        return fs::read_to_string(&resource_dir)
            .expect(&resource_dir.as_str());
    }
}
