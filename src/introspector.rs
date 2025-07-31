use crate::error::{Error, Result};
use crate::schema::*;
use crate::utils::to_snake_case;
use std::collections::HashMap;

pub struct HelixDBSchemaIntrospector {
    client: reqwest::Client,
    connection: super::connection::HelixDBConnection,
}

impl HelixDBSchemaIntrospector {
    pub fn new(connection: super::connection::HelixDBConnection) -> Self {
        let client = reqwest::Client::new();
        Self { client, connection }
    }

    pub async fn introspect_schema(&self) -> Result<HelixSchema> {
        let url = format!("{}/introspect", self.connection.url);
        let request = self.client.get(&url);

        println!("Fetching schema from: {url}");
        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(Error::Reqwest(response.error_for_status().unwrap_err()));
        }

        let response_text = response.text().await?;
        println!("Raw response: {response_text}");

        let introspection: IntrospectionResponse =
            serde_json::from_str(&response_text).map_err(|e| {
                Error::IO(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Failed to parse introspection response: {e}"),
                ))
            })?;

        let mut schema = HelixSchema {
            nodes: HashMap::new(),
            vectors: HashMap::new(),
            queries: HashMap::new(),
            edges: HashMap::new(),
        };

        // Convert nodes
        for node_info in introspection.schema.nodes {
            let node_def = NodeDefinition {
                name: node_info.name.clone(),
                properties: self.convert_properties(&node_info.properties)?,
            };
            schema.nodes.insert(node_info.name, node_def);
        }

        // Convert vectors
        for vector_info in introspection.schema.vectors {
            let vector_def = VectorDefinition {
                name: vector_info.name.clone(),
                properties: self.convert_properties(&vector_info.properties)?,
            };
            schema.vectors.insert(vector_info.name, vector_def);
        }

        // Convert edges
        for edge_info in introspection.schema.edges {
            let edge_def = EdgeDefinition {
                name: edge_info.name.clone(),
                from_node: edge_info.from,
                to_node: edge_info.to,
                properties: self.convert_properties(&edge_info.properties)?,
            };
            schema.edges.insert(edge_info.name, edge_def);
        }

        // Convert queries
        for query_info in introspection.queries {
            let query_def = QueryDefinition {
                name: query_info.name.clone(),
                parameters: self.convert_properties(&query_info.parameters)?,
                returns: if query_info.returns.len() == 1 {
                    query_info.returns[0].clone()
                } else {
                    format!("{{ {} }}", query_info.returns.join(", "))
                },
                description: None,
            };
            schema.queries.insert(query_info.name, query_def);
        }

        Ok(schema)
    }

    fn convert_properties(
        &self,
        properties: &HashMap<String, String>,
    ) -> Result<HashMap<String, FieldType>> {
        let mut result = HashMap::new();

        for (prop_name, type_str) in properties {
            let field_type = Self::parse_field_type(type_str)?;
            result.insert(to_snake_case(prop_name), field_type);
        }

        Ok(result)
    }

    fn parse_field_type(type_str: &str) -> Result<FieldType> {
        match type_str {
            "String" => Ok(FieldType::String),
            "I8" => Ok(FieldType::Integer),
            "I16" => Ok(FieldType::Integer),
            "I32" => Ok(FieldType::Integer),
            "I64" => Ok(FieldType::Integer),
            "I128" => Ok(FieldType::Integer),
            "U8" => Ok(FieldType::Integer),
            "U16" => Ok(FieldType::Integer),
            "U32" => Ok(FieldType::Integer),
            "U64" => Ok(FieldType::Integer),
            "U128" => Ok(FieldType::Integer),
            "F64" => Ok(FieldType::Float),
            "Boolean" => Ok(FieldType::Boolean),
            "ID" => Ok(FieldType::Integer),
            s if s.starts_with("Array(") && s.ends_with(")") => {
                let inner_type = &s[6..s.len() - 1];
                let inner = Self::parse_field_type(inner_type)?;
                Ok(FieldType::Array(Box::new(inner)))
            }
            s if s.starts_with("Vector<") && s.ends_with(">") => {
                let dim_str = &s[7..s.len() - 1];
                let dim = dim_str.parse::<usize>().map_err(|e| {
                    Error::IO(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid vector dimension '{dim_str}': {e}"),
                    ))
                })?;
                Ok(FieldType::Vector(dim))
            }
            _ => Ok(FieldType::Custom(type_str.to_string())),
        }
    }
}
