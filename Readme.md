### Goal
To setup a boilerplate for fetching data from private eth node. Docker-compose will create geth-node and validate the service through api server

### Setup
```
cp .env.example .env
docker compose up --build
```

### Assumption
- The system is flexible enough in handling different blockchain data formats


### Folder Structure
The folder structure is crafted to utilize the Service Adapter Pattern, enhancing both flexibility and scalability, particularly when managing multiple blockchain platforms. This architectural approach enables your application to seamlessly interact with various blockchain services by abstracting their distinct implementations behind a unified interface.

The environment folder is organized to facilitate straightforward deployment across different environments.

### Enginnering Excellent
- [DONE] UUID Transaction IDs: Incorporated UUIDs as transaction IDs in each request to improve observability.
- [DONE] Enhanced Logging: Implemented logging for requests and their latencies to aid in monitoring and debugging.
- [DONE] Graceful Shutdown: Added graceful shutdown capabilities for the API server to ensure smooth termination of processes.
- [DONE] Request Timeout Handling: Introduced timeout handling for requests to enhance reliability and prevent hanging operations.
- [TODO] batch async abi request in parallel

### Task Requirement
1. API Client print addresses.json at server startup
2. API Endpoint on ERC20 token info
3. API Endpoint on ERC20 token balance
4. Setup Geth docker node
5. Load genesis.json data into Geth node
