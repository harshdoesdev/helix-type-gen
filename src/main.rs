use clap::Parser;
use helix_ts_gen::{
    HelixDBConnection, HelixDBSchemaIntrospector, TypeScriptGenerator, error::Result,
};
use std::fs;

#[derive(Parser)]
#[command(
    about = "Generate TypeScript types and typed client from HelixDB schema and queries using /introspect endpoint"
)]
struct Args {
    #[arg(
        short = 'u',
        long = "endpoint",
        default_value = "http://localhost:6969"
    )]
    endpoint: String,

    #[arg(short = 'o', long = "output-file", default_value = "helix-client.ts")]
    output_file: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("Connecting to HelixDB at: {}", args.endpoint);

    let connection = HelixDBConnection { url: args.endpoint };
    let introspector = HelixDBSchemaIntrospector::new(connection);

    println!("Introspecting schema...");
    let schema = introspector.introspect_schema().await?;

    println!(
        "Found {} nodes, {} vectors, {} edges, {} queries",
        schema.nodes.len(),
        schema.vectors.len(),
        schema.edges.len(),
        schema.queries.len()
    );

    let generator = TypeScriptGenerator::new(schema);
    let typescript_code = generator.generate();

    fs::write(&args.output_file, typescript_code)?;

    println!(
        "TypeScript types generated successfully: {}",
        args.output_file
    );
    println!(
        "Import with: import {{ TypedHelixDBClient, createTypedClient }} from './{}'",
        args.output_file
    );

    Ok(())
}
