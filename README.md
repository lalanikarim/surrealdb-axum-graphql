# SurrealDB With Axum and GraphQL

GraphQL with Surrealdb until graphql support is added to surrealdb 

[SurrealDB](https://surrealdb.com), at the time of writing this, is still waiting its 1.0 production release. An anticipated feature that is expected to land with 1.0 production release is GraphQL support, which is still missing in the as of current 1.0.0-beta.9.  

This project explores a potential approach to creating GraphQL api backed by SurrealDB and Axum. A rust frontend web library might be included to build applications with this approach.

# Launching

A [docker-compose.yml](https://github.com/lalanikarim/surrealdb-axum-graphql/blob/main/docker-compose.yml) file is included along with [.env.example](https://github.com/lalanikarim/surrealdb-axum-graphql/blob/main/.env.example) file.  
Rename the `.env.example` to `.env` and launch in a container locally using `docker-compose up`.

