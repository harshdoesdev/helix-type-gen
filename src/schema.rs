use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelixSchema {
    pub nodes: HashMap<String, NodeDefinition>,
    pub vectors: HashMap<String, VectorDefinition>,
    pub queries: HashMap<String, QueryDefinition>,
    pub edges: HashMap<String, EdgeDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDefinition {
    pub name: String,
    pub properties: HashMap<String, FieldType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorDefinition {
    pub name: String,
    pub properties: HashMap<String, FieldType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeDefinition {
    pub name: String,
    pub from_node: String,
    pub to_node: String,
    pub properties: HashMap<String, FieldType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDefinition {
    pub name: String,
    pub parameters: HashMap<String, FieldType>,
    pub returns: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    ID,
    Vector(usize),
    Array(Box<FieldType>),
    Optional(Box<FieldType>),
    Custom(String),
}

// API response structures
#[derive(Debug, Deserialize)]
pub struct IntrospectionResponse {
    pub schema: SchemaData,
    pub queries: Vec<QueryInfo>,
}

#[derive(Debug, Deserialize)]
pub struct SchemaData {
    pub nodes: Vec<NodeInfo>,
    pub vectors: Vec<VectorInfo>,
    pub edges: Vec<EdgeInfo>,
}

#[derive(Debug, Deserialize)]
pub struct NodeInfo {
    pub name: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct VectorInfo {
    pub name: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct EdgeInfo {
    pub name: String,
    pub from: String,
    pub to: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct QueryInfo {
    pub name: String,
    pub parameters: HashMap<String, String>,
    pub returns: Vec<String>,
}
