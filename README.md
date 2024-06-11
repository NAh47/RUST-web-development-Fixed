
# RUST-web-development

## Nahom Kiros
This is a repo for CS410P:RUST web development homeworks and assignments. 

This Rust project uses Axum to build a RESTful API for managing a list of questions. It features PostgreSQL for persistent storage and supports basic CRUD operations. Below is an overview of the key components of the project:

## Files Overview

- **`main.rs`**:
  - Initializes and runs the Axum web server.
  - Sets up routes and manages shared state for question data.
  - Connects to the PostgreSQL database.

- **`api.rs`**:
  - Handles CRUD operations for questions through HTTP endpoints.

- **`question_list.rs`**:
  - Manages the `Question` data structure and PostgreSQL storage.
  - Provides methods for initializing data and performing CRUD operations.

- **`docker-compose.yml`**:
  - Defines the Docker services for the Rust application and PostgreSQL database.

- **`Dockerfile`**:
  - Builds the Rust application in a Docker container.

- **`Cargo.toml`** and **`Cargo.lock`**:
  - Define the Rust project dependencies and metadata.

## Getting Started

To run this project locally, ensure you have Docker and Docker Compose installed. Clone the repository, navigate to the directory containing the `docker-compose.yml`, and run the following command:

\`\`\`bash
docker compose up --build
\`\`\`

This command will start the PostgreSQL database and the Rust web server.

### Testing the API

You can test the API using `curl` or any API client (like Postman). Here are some example `curl` commands:

- **Create a Question**:

  \`\`\`bash
  curl -X POST http://127.0.0.1:3000/questions -H "Content-Type: application/json" -d '{
    "id": "1",
    "header": "What is Rust?",
    "body": "A systems programming language that aims for memory safety and concurrency.",
    "categories": ["programming", "rust"]
  }'
  \`\`\`

- **Fetch All Questions**:

  \`\`\`bash
  curl http://127.0.0.1:3000/questions
  \`\`\`

- **Fetch a Specific Question**:

  \`\`\`bash
  curl http://127.0.0.1:3000/questions/1
  \`\`\`

- **Update a Question**:

  \`\`\`bash
  curl -X PUT http://127.0.0.1:3000/questions/1 -H "Content-Type: application/json" -d '{
    "id": "1",
    "header": "What is Rust?",
    "body": "A systems programming language that aims for memory safety and concurrency, and high performance.",
    "categories": ["programming", "rust"]
  }'
  \`\`\`

- **Delete a Question**:

  \`\`\`bash
  curl -X DELETE http://127.0.0.1:3000/questions/1
  \`\`\`

## Additional Information

- **Environment Variables**:
  - `PG_DBNAME`: Name of the PostgreSQL database (default: `rustdb`)
  - `PG_HOST`: Host of the PostgreSQL database (default: `db`)
  - `PG_USER`: PostgreSQL user (default: `NahomRust`)
  - `PG_PASSWORD`: PostgreSQL user password (default: `rustwebdev`)

These should be configured in a `.env` file in the root directory of the project.
