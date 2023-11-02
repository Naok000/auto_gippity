use crate::models::agent_basic::basic_agent::BasicAgent;
use async_trait::async_trait;
use serde::{ Deserialize, Serialize };
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub is_user_login_and_logout:bool,
    pub is_external_urls_required:bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout:bool,
    pub is_external_urls_required:bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub api_endpoints_schema: Option<Vec<RouteObject>>,
}

#[async_trait]
pub trait SpecialFunctions: Debug {

    // Used to that manager can get attributes from Agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;

    //  This function will allow agents to execute their logic
    async fn execute(&mut self, factsheet: &mut FactSheet) -> Result<(), Box<dyn std::error::Error>>;
}