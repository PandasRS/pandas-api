![PandaAPI](panda.png)

# Pandas API in Rust

## 
Overview

Welcome to `pandas-api`, a simple yet illustrative example of how to create RESTful APIs in Rust, specifically tailored for developers transitioning from Node.js. This project demonstrates the fundamental aspects of building a web service in Rust, including routing, database operations, and structured project organization. The focus is on interfacing with MongoDB, one of the popular databases often used in conjunction with Node.js applications.

## Features

* **RESTful API Endpoints** : Create, Read (single and all), Update, and Delete (CRUD) operations for 'panda' entities.
* **MongoDB Integration** : Demonstrates how to connect and interact with MongoDB from Rust.
* **Modular Structure** : Organized project structure with separate modules for configuration, database access, and entity-specific logic.
* **Swagger Documentation** : Includes Swagger UI setup for API documentation and testing.

## Getting Started

### Prerequisites

* Rust Programming Environment: Ensure you have [Rust installed]().
* MongoDB: Make sure MongoDB is [installed and running]() on your local machine or accessible via a network connection.

### Installation

1. **Clone the Repository** :

```
git clone https://github.com/viniciusgomes/pandas-api
cd pandas-api

```

2. **Build the Project** :

```
cargo build

```

3. **Run the Application** :

```
cargo run

```

   The API will be available at `http://localhost:8000`.

### API Endpoints

* `POST /pandas`: Create a new panda.
* `GET /pandas`: Retrieve all pandas.
* `GET /pandas/:id`: Retrieve a single panda by ID.
* `PUT /pandas/:id`: Update a panda by ID.
* `DELETE /pandas/:id`: Delete a panda by ID.

Swagger UI for the API documentation and testing can be accessed at `http://localhost:8000/swagger-ui`.

## Contributing

Contributions to enhance `pandas-api` are welcome. Feel free to fork the repository and submit pull requests.

## License

This project is licensed under the [MIT License]().
