
# RUST-web-development

## Nahom Kiros

This is a repo for CS410P: RUST web development homeworks and assignments. 

This Rust project uses Axum to build a RESTful API for managing a list of questions. It features PostgreSQL for persistent storage and supports basic CRUD operations. Additionally, it includes a Yew-based frontend for interacting with the API. Below is an overview of the key components of the project:

## Files Overview

- **`backend/src/main.rs`**:
  - Initializes and runs the Axum web server.
  - Sets up routes and manages shared state for question data.
  - Connects to the PostgreSQL database.

- **`backend/src/api.rs`**:
  - Handles CRUD operations for questions through HTTP endpoints.

- **`backend/src/question_list.rs`**:
  - Manages the `Question` data structure and PostgreSQL storage.
  - Provides methods for initializing data and performing CRUD operations.

- **`frontend/src/lib.rs`**:
  - Implements the Yew-based frontend.
  - Provides UI components and interactions for creating, reading, updating, and deleting questions.

- **`frontend/index.html`**:
  - The entry point for the frontend application.

- **`frontend/style.css`**:
  - Styles for the frontend application.

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

This command will start the PostgreSQL database, the Rust backend server on `http://localhost:3000`, and the frontend server on `http://localhost:8080`.

### Running Only the Backend

If you want to run only the backend service, you can use the following command:

\`\`\`bash
docker compose up --build backend
\`\`\`

The backend API will be accessible at `http://localhost:3000`.

### Running Only the Frontend

If you want to run only the frontend service, you can use the following command:

\`\`\`bash
docker compose up --build frontend
\`\`\`

The frontend application will be accessible at `http://localhost:8080`.

### Running Backend and Frontend Separately

You can start both services separately by running the following commands in two different terminals:

\`\`\`bash
docker compose up --build backend
docker compose up --build frontend
\`\`\`

## Testing the API

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

## Using the Frontend

The frontend can be accessed by navigating to `http://localhost:8080` in your web browser. From there, you can interact with the API using the provided UI.

### Frontend Features

- **Create Question**: Fill in the form fields and click "Create" to add a new question.
- **Fetch Question by ID**: Enter an ID in the input field and click "Fetch" to retrieve a specific question.
- **Update Question**: Fill in the update form fields and click "Update" to modify an existing question.
- **Delete Question**: Enter an ID in the input field and click "Delete" to remove a question.
- **Fetch All Questions**: Click "Fetch All" to list all questions. Click again to hide the list.


### Building and Running Without Docker

If you prefer to run the project without Docker, ensure you have Rust and PostgreSQL installed on your machine. Set up the environment variables as described above and run the following commands:

1. **Start the PostgreSQL Database**:

   \`\`\`bash
   pg_ctl start
   \`\`\`

2. **Run the Backend**:

   \`\`\`bash
   cd backend
   cargo run
   \`\`\`

3. **Run the Frontend**:

   \`\`\`bash
   cd frontend
   trunk serve
   \`\`\`

The backend will be accessible at `http://localhost:3000` and the frontend at `http://localhost:8080`.

