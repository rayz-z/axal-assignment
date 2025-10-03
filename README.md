# Axal Rust Project

This project consists of three main components: a monitoring script, a backend server, and a frontend application.

## Getting Started

### Prerequisites

- Rust (for backend)
- Node.js and npm (for frontend)

### Running the Components

#### 1. Monitoring Script

```bash
cd backend
cargo run --bin main
```

#### 2. Backend Server

```bash
cd backend
cargo run --bin dashboardBackend
```

#### 3. Frontend Application

```bash
cd frontend/vault-frontend
npm run dev
```

## Project Structure

- `backend/` - Rust backend with monitoring script and dashboard server
- `frontend/vault-frontend/` - React frontend application

## Notes

- Make sure to run the backend server before starting the frontend for full functionality
- The monitoring script can run independently
- All components should be running simultaneously for the complete application experience
