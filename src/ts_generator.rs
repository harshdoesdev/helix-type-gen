use crate::{
    schema::*,
    utils::{to_pascal_case, to_snake_case},
};

pub struct TypeScriptGenerator {
    schema: HelixSchema,
}

impl TypeScriptGenerator {
    pub fn new(schema: HelixSchema) -> Self {
        Self { schema }
    }

    pub fn generate(&self) -> String {
        let mut output = String::new();

        output.push_str("// Generated TypeScript types for HelixDB schema\n");
        output.push_str("// This file is auto-generated. Do not edit manually.\n");
        output.push_str("// Generated with helix-ts-gen from introspection endpoint\n\n");

        // Import HelixDB (only export available)
        output.push_str("import HelixDB from 'helix-ts';\n\n");

        output.push_str(&self.generate_utility_types());
        output.push_str(&self.generate_helix_types()); // Add HelixDB types since they're not exported
        output.push_str(&self.generate_node_types());
        output.push_str(&self.generate_vector_types());
        output.push_str(&self.generate_edge_types());
        output.push_str(&self.generate_query_types());
        output.push_str(&self.generate_client_interface());
        output.push_str(&self.generate_sdk_helpers());

        output
    }

    fn generate_helix_types(&self) -> String {
        r#"// HelixDB types (extracted from runtime since not exported)
export type HelixDBClient = HelixDB;
export type HelixDBResponse = Awaited<ReturnType<HelixDB['query']>>;
export type HelixDBInput = Parameters<HelixDB['query']>[1];

"#
        .to_string()
    }

    fn generate_utility_types(&self) -> String {
        r#"// Utility types
export type Vector = number[];
export type Optional<T> = T | null | undefined;
export type HelixID = bigint;

export interface BaseNode {
  id: HelixID;
  created_at?: number;
  updated_at?: number;
}

export interface BaseVector {
  id: HelixID;
}

export interface BaseEdge {
  id?: HelixID;
  created_at?: number;
}

"#
        .to_string()
    }

    fn generate_node_types(&self) -> String {
        let mut output = String::new();
        output.push_str("// Node types\n");

        for (name, node) in &self.schema.nodes {
            output.push_str(&format!("export interface {name} extends BaseNode {{\n"));

            for (prop_name, field_type) in &node.properties {
                let ts_type = Self::field_type_to_typescript(field_type);
                output.push_str(&format!("  {prop_name}: {ts_type};\n"));
            }

            output.push_str("}\n\n");
        }

        if !self.schema.nodes.is_empty() {
            let node_names: Vec<String> = self.schema.nodes.keys().cloned().collect();
            output.push_str(&format!(
                "export type AnyNode = {};\n\n",
                node_names.join(" | ")
            ));
        }

        output
    }

    fn generate_vector_types(&self) -> String {
        let mut output = String::new();
        output.push_str("// Vector types\n");

        for (name, vector) in &self.schema.vectors {
            output.push_str(&format!(
                "export interface {name} extends BaseVector {{\n"
            ));

            for (prop_name, field_type) in &vector.properties {
                let ts_type = Self::field_type_to_typescript(field_type);
                output.push_str(&format!("  {prop_name}: {ts_type};\n"));
            }

            output.push_str("}\n\n");
        }

        if !self.schema.vectors.is_empty() {
            let vector_names: Vec<String> = self.schema.vectors.keys().cloned().collect();
            output.push_str(&format!(
                "export type AnyVector = {};\n\n",
                vector_names.join(" | ")
            ));
        }

        output
    }

    fn generate_edge_types(&self) -> String {
        let mut output = String::new();
        output.push_str("// Edge types\n");

        for (name, edge) in &self.schema.edges {
            output.push_str(&format!("export interface {name} extends BaseEdge {{\n"));
            output.push_str(&format!("  from: {};\n", edge.from_node));
            output.push_str(&format!("  to: {};\n", edge.to_node));

            for (prop_name, field_type) in &edge.properties {
                let ts_type = Self::field_type_to_typescript(field_type);
                output.push_str(&format!("  {prop_name}: {ts_type};\n"));
            }

            output.push_str("}\n\n");
        }

        if !self.schema.edges.is_empty() {
            let edge_names: Vec<String> = self.schema.edges.keys().cloned().collect();
            output.push_str(&format!(
                "export type AnyEdge = {};\n\n",
                edge_names.join(" | ")
            ));
        }

        output
    }

    fn generate_query_types(&self) -> String {
        let mut output = String::new();
        output.push_str("// Query parameter types\n");
        output.push_str("// Note: Return types are not generated as they cannot be inferred from schema introspection\n\n");

        if !self.schema.queries.is_empty() {
            for (name, query) in &self.schema.queries {
                let param_type_name = format!("{}Params", to_pascal_case(name));
                output.push_str(&format!("export interface {param_type_name} {{\n"));

                for (param_name, field_type) in &query.parameters {
                    let ts_type = Self::field_type_to_typescript(field_type);
                    output.push_str(&format!("  {param_name}: {ts_type};\n"));
                }

                output.push_str("}\n\n");
            }
        }

        output
    }

    fn generate_client_interface(&self) -> String {
        let mut output = String::new();
        output.push_str("// Client interface for type-safe HelixDB operations\n");

        if !self.schema.queries.is_empty() {
            output.push_str("export interface TypedHelixDBClient {\n");

            for (name, query) in &self.schema.queries {
                let param_type = format!("{}Params", to_pascal_case(name));

                if let Some(ref description) = query.description {
                    output.push_str(&format!("  /**\n   * {description}\n   */\n"));
                }

                // Use the actual HelixDBResponse type since we can't infer return types
                output.push_str(&format!(
                    "  {name}(params: {param_type}): Promise<HelixDBResponse>;\n"
                ));
            }

            output.push_str("}\n\n");
        } else {
            output.push_str("// No queries found in schema\n");
            output.push_str("export interface TypedHelixDBClient {\n");
            output.push_str(
                "  query(endpoint: string, params: HelixDBInput): Promise<HelixDBResponse>;\n",
            );
            output.push_str("}\n\n");
        }

        output
    }

    fn generate_sdk_helpers(&self) -> String {
        let mut output = String::new();

        output.push_str("// SDK Integration Helpers\n");

        output.push_str("/**\n");
        output.push_str(" * Create a type-safe HelixDB client wrapper\n");
        output.push_str(" * Usage:\n");
        output.push_str(" *   const client = new HelixDB();\n");
        output.push_str(" *   const typedClient = createTypedClient(client);\n");
        output.push_str(" */\n");
        output
            .push_str("export function createTypedClient(client: HelixDB): TypedHelixDBClient {\n");
        output.push_str("  return {\n");

        if !self.schema.queries.is_empty() {
            for name in self.schema.queries.keys() {
                output.push_str(&format!("    async {name}(params) {{\n"));
                output.push_str(&format!(
                    "      return await client.query('{name}', params);\n"
                ));
                output.push_str("    },\n");
            }
        } else {
            output.push_str("    async query(endpoint: string, params: HelixDBInput): Promise<HelixDBResponse> {\n");
            output.push_str("      return await client.query(endpoint, params);\n");
            output.push_str("    },\n");
        }

        output.push_str("  };\n");
        output.push_str("}\n\n");

        // Generate query endpoint constants for better DX
        if !self.schema.queries.is_empty() {
            output.push_str("// Query endpoint constants\n");
            output.push_str("export const QueryEndpoints = {\n");
            for name in self.schema.queries.keys() {
                output.push_str(&format!(
                    "  {}: '{}',\n",
                    to_snake_case(name).to_uppercase(),
                    name
                ));
            }
            output.push_str("} as const;\n\n");
        }

        output.push_str("// Type guards and validation helpers\n");
        for name in self.schema.nodes.keys() {
            output.push_str(&format!(
                "export function is{name}(obj: any): obj is {name} {{\n"
            ));
            output.push_str(
                "  return obj && typeof obj === 'object' && typeof obj.id === 'bigint';\n",
            );
            output.push_str("}\n\n");
        }

        for name in self.schema.vectors.keys() {
            output.push_str(&format!(
                "export function is{name}(obj: any): obj is {name} {{\n"
            ));
            output.push_str(
                "  return obj && typeof obj === 'object' && typeof obj.id === 'bigint';\n",
            );
            output.push_str("}\n\n");
        }

        output.push_str("/**\n");
        output.push_str(
            " * Helper to extract data from HelixDB response with basic type assertion\n",
        );
        output.push_str(" * Usage: const users = extractData<User[]>(response, 'users');\n");
        output.push_str(" */\n");
        output.push_str(
            "export function extractData<T>(response: HelixDBResponse, key?: string): T {\n",
        );
        output.push_str("  if (key) {\n");
        output.push_str("    return response[key] as T;\n");
        output.push_str("  }\n");
        output.push_str("  return response as T;\n");
        output.push_str("}\n\n");

        output
    }

    fn field_type_to_typescript(field_type: &FieldType) -> String {
        match field_type {
            FieldType::String => "string".to_string(),
            FieldType::Integer => "number".to_string(),
            FieldType::Float => "number".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::ID => "HelixID".to_string(),
            FieldType::Vector(dim) => format!("number[{dim}]"),
            FieldType::Array(inner) => {
                format!("{}[]", Self::field_type_to_typescript(inner))
            }
            FieldType::Optional(inner) => {
                format!("Optional<{}>", Self::field_type_to_typescript(inner))
            }
            FieldType::Custom(name) => name.clone(),
        }
    }
}
