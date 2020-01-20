
extern crate reqwest;

use std::collections::HashMap;

// const ENGINE : String = String::from("/engine");

pub struct WorkflowEngine {
    base_url : String,
}

impl WorkflowEngine {

    // pub fn workflow_engine_from_url(url: String) -> WorkflowEngine {
    pub fn workflow_engine_from_url() -> WorkflowEngine {

        WorkflowEngine { base_url: "".to_string() }
    }

    pub fn get_engine(&self) {

        // let mut url = String::from(&self.base_url)
        //     .push_str("/engine");
    
        let response = reqwest::blocking::get("http://localhost:8080/api/workflow/engine")
            .unwrap();
    
        if response.status().is_success() {
    
            let engine = response.json::<Vec<HashMap<String, String>>>()
                .unwrap();
    
            println!("response was 200");    
            println!("result: {:#?}", engine);
        }
    
        // println!("response was: {}", response.status())
    }
    
}


// -----------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn request_test() {
        let engine : WorkflowEngine = WorkflowEngine::workflow_engine_from_url();

        engine.get_engine();

    }
}
