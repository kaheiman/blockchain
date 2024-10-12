### Setup
```
cp .env.example .env
docker compose up --build
```

### Folder Structure
The folder structure is crafted to utilize the Service Adapter Pattern, enhancing both flexibility and scalability, particularly when managing multiple blockchain platforms. This architectural approach enables your application to seamlessly interact with various blockchain services by abstracting their distinct implementations behind a unified interface.

The environment folder is organized to facilitate straightforward deployment across different environments.
```

### Implementation
- UUID Transaction IDs: Incorporated UUIDs as transaction IDs in each request to improve observability.
- Enhanced Logging: Implemented logging for requests and their latencies to aid in monitoring and debugging.
- Graceful Shutdown: Added graceful shutdown capabilities for the API server to ensure smooth termination of processes.
- Request Timeout Handling: Introduced timeout handling for requests to enhance reliability and prevent hanging operations.
