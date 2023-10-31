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
    pub external_urls: String,
    pub backend_code: String,
    pub api_endpoints_schema: String,
}