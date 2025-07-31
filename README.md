# helix-ts-gen

Generate TypeScript types and a typed client from your HelixDB schema and queries.

## Description

`helix-ts-gen` is a command-line tool that introspects your HelixDB database to generate TypeScript types for your schema and a typed client for your queries.

## Installation

You can install `helix-ts-gen` by running the following command in your terminal:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/harshdoesdev/helix-type-gen/main/install.sh)"
```

This script will download the latest release from GitHub, unzip it, and install the `helix-ts-gen` binary to `/usr/local/bin`.

## Usage

Run the following command to generate the TypeScript client:

```bash
helix-ts-gen --endpoint <your-helixdb-endpoint> --output-file <output-path>
```

-   `--endpoint`: The URL of your HelixDB instance (e.g., `http://localhost:6969`).
-   `--output-file`: The path where the generated TypeScript file will be saved (e.g., `helix-client.ts`).

This will generate a TypeScript file at the specified output path, containing the types for your schema and a typed client for your queries.

### Example

To generate a TypeScript client from a HelixDB instance running on `http://localhost:6969` and save it to `helix-client.ts`:

```bash
helix-ts-gen --endpoint http://localhost:6969 --output-file helix-client.ts
```

You can then import the generated client in your TypeScript code:

```typescript
import { TypedHelixDBClient, createTypedClient } from './helix-client';

const client: TypedHelixDBClient = createTypedClient({
    url: 'http://localhost:6969',
});

// Now you can use the typed client to interact with your HelixDB instance
```