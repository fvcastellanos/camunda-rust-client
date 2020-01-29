extern crate reqwest;
extern crate serde;

use reqwest::blocking::get;
use reqwest::blocking::Client;

use serde::{ Deserialize };

// fn <T> perform_get(url: &str, headers: Map<String, String>, params: Map<String, String>) -> Result<T, String> {
    
//     let response = get(url, );
    
// }

#[derive(Deserialize)]
struct ExternalTask {

    #[serde(rename="activityId")]
    activity_id : Option<String>,

    #[serde(rename="activityInstanceId")]
    activity_instance_id: Option<String>,

    #[serde(rename="errorMessage")]
    error_message: Option<String>,

    #[serde(rename="errorDetails")]
    error_details: Option<String>,

    #[serde(rename="executionId")]
    execution_id: Option<String>,

    id: Option<String>,

    #[serde(rename="lockExpirationTime")]
    lock_expiration_time: Option<String>,

    #[serde(rename="processDefinitionId")]
    process_definition_id: Option<String>,

    #[serde(rename="processDefinitionKey")]
    process_definition_key: Option<String>,

    #[serde(rename="processInstanceId")]
    process_instance_id: Option<String>,

    #[serde(rename="tenantId")]
    tenant_id: Option<String>,

    retries: u16,
    suspended: bool,

    #[serde(rename="workerId")]
    worker_id: Option<String>,

    #[serde(rename="topicName")]
    topic_name: Option<String>,

    priority: i16
}

struct ExternalTaskBuilder {

    api_url: String,    
    // externalTaskId: Option<String>
}

impl ExternalTaskBuilder {

    fn from_url(url: &str) -> ExternalTaskBuilder {
        
        ExternalTaskBuilder { api_url: String::from(url) }
    }
}

// fn get_external_tasks() -> Result<Vec<ExternalTask>, String> {

//     let client = Client::new();

//     let blah = client.get("")
        
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_external_task_builder() {

        let builder = ExternalTaskBuilder::from_url("blah");        
    }
}
