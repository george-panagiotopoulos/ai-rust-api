#!/bin/bash

# AI-Rust-API - RAG System Startup Script
# Performs clean restart of all services with port cleanup

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RAGAPI_PORT=9101
BEDROCKAPI_PORT=9100
AUTHAPI_PORT=9102
POSTGRES_CONTAINER_NAME="shared_postgres"

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if a port is in use
check_port() {
    local port=$1
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        return 0  # Port is in use
    else
        return 1  # Port is free
    fi
}

# Function to kill process on a specific port
kill_port() {
    local port=$1
    local service_name=$2

    if check_port $port; then
        print_warning "Port $port ($service_name) is in use. Killing process..."
        lsof -ti :$port | xargs kill -9 2>/dev/null || true
        sleep 2
        if check_port $port; then
            print_error "Failed to free port $port"
            return 1
        else
            print_success "Successfully freed port $port"
        fi
    else
        print_status "Port $port ($service_name) is already free"
    fi
}

# Function to check if we're in the right directory
check_directory() {
    if [[ ! -f "start.sh" ]] || [[ ! -d "services" ]]; then
        print_error "Please run this script from the ai-rust-api root directory."
        print_error "Current directory: $(pwd)"
        exit 1
    fi
}

# Function to check if Docker is running
check_docker() {
    if ! docker info >/dev/null 2>&1; then
        print_error "Docker is not running. Please start Docker and try again."
        exit 1
    fi
}

# Function to check if required tools are available
check_dependencies() {
    local missing_deps=()

    if ! command -v cargo >/dev/null 2>&1; then
        missing_deps+=("cargo (Rust)")
    fi

    if ! command -v python3 >/dev/null 2>&1; then
        missing_deps+=("python3")
    fi

    if ! command -v curl >/dev/null 2>&1; then
        missing_deps+=("curl")
    fi

    if [ ${#missing_deps[@]} -ne 0 ]; then
        print_error "Missing required dependencies:"
        for dep in "${missing_deps[@]}"; do
            echo "  - $dep"
        done
        exit 1
    fi
}

# Function to clean up Docker containers
cleanup_docker() {
    print_status "Cleaning up Docker containers..."

    # Stop and remove existing containers
    docker stop $POSTGRES_CONTAINER_NAME 2>/dev/null || true
    docker rm $POSTGRES_CONTAINER_NAME 2>/dev/null || true

    # Remove any orphaned containers
    docker container prune -f >/dev/null 2>&1 || true

    print_success "Docker cleanup completed"
}

# Function to start PostgreSQL
start_postgres() {
    print_status "Starting PostgreSQL with pgvector..."

    cd services/Database

    # Start PostgreSQL container
    docker-compose up -d

    # Wait for database to be ready
    print_status "Waiting for PostgreSQL to be ready..."
    local db_ready=false
    for i in {1..30}; do
        if docker-compose exec -T postgres pg_isready -U raguser -d ragdb >/dev/null 2>&1; then
            print_success "PostgreSQL is ready!"
            db_ready=true
            break
        fi
        sleep 2
    done

    if [ "$db_ready" = false ]; then
        print_error "PostgreSQL failed to start within 60 seconds"
        exit 1
    fi

    # Initialize database schema
    print_status "Initializing database schema..."
    if docker-compose exec -T postgres psql -U raguser -d ragdb -f /docker-entrypoint-initdb.d/init.sql >/dev/null 2>&1; then
        print_success "Database initialized successfully"
    else
        print_warning "Database schema initialization completed with warnings (this is normal for first run)"
    fi

    cd ../..
}

# Function to start a service
start_service() {
    local service_name=$1
    local service_dir=$2
    local port=$3

    print_status "Starting $service_name on port $port..."

    cd services/$service_dir

    # Load environment variables from .env file if it exists
    if [ -f ".env" ]; then
        print_status "Loading environment variables from .env file..."
        source .env
    fi

    # Build and start the service
    if [ "$service_name" = "AuthAPI" ]; then
        print_status "Building AuthAPI..."
        cargo build --release --bin auth-api

        print_status "Starting AuthAPI..."
        DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb \
        HOST=127.0.0.1 \
        PORT=9102 \
        ./target/release/auth-api &
    elif [ "$service_name" = "BedrockAPI" ]; then
        print_status "Building BedrockAPI..."
        cargo build --release --bin bedrock-chat-api

        print_status "Starting BedrockAPI..."
        SERVER_HOST=127.0.0.1 \
        SERVER_PORT=9100 \
        DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb \
        AUTH_API_URL=http://127.0.0.1:9102 \
        ./target/release/bedrock-chat-api &
    else
        print_status "Building RAGAPI..."
        cargo build --release --bin ragapi

        print_status "Starting RAGAPI..."
        DATABASE_URL=postgresql://raguser:password@localhost:5434/ragdb \
        PORT=9101 \
        BEDROCK_API_URL=http://127.0.0.1:9100 \
        AUTH_API_URL=http://127.0.0.1:9102 \
        ./target/release/ragapi &
    fi

    # Wait for service to start
    print_status "Waiting for $service_name to start..."
    for i in {1..30}; do
        if check_port $port; then
            print_success "$service_name started successfully on port $port"
            break
        fi
        sleep 2
    done

    if ! check_port $port; then
        print_error "$service_name failed to start on port $port"
        return 1
    fi

    cd ../..
}

# Function to display service information
show_services() {
    echo
    echo "========================================"
    echo "🚀 Services Started Successfully!"
    echo "========================================"
    echo
    echo "🔐 AuthAPI (Authentication Service)"
    echo "   URL: http://localhost:9102"
    echo "   Health: http://localhost:9102/health"
    echo "   Login: curl -X POST http://localhost:9102/login \\"
    echo "         -H \"Content-Type: application/json\" \\"
    echo "         -d '{\"username\": \"admin\", \"password\": \"Admin@1234!\"}'"
    echo
    echo "🤖 BedrockAPI (AI Chat Service) - Requires Authentication"
    echo "   URL: http://localhost:9100"
    echo "   Health: http://localhost:9100/health"
    echo
    echo "🌐 RAGAPI (Document Q&A Service) - Requires Authentication"
    echo "   URL: http://localhost:9101"
    echo "   Health: http://localhost:9101/health"
    echo "   Stats: http://localhost:9101/stats"
    echo
    echo "🗄️  PostgreSQL Database"
    echo "   Container: shared_postgres"
    echo "   Database: ragdb"
    echo "   User: raguser"
    echo
    echo "📁 Documents Directory"
    echo "   Location: services/RAGAPI/documents/"
    echo "   Add your documents here for automatic embedding"
    echo "   Vectorize: python3 services/RAGAPI/scripts/vectorize.py"
    echo
    echo "🔑 Authentication Flow:"
    echo "   1. Login: curl -X POST http://localhost:9102/login -H \"Content-Type: application/json\" -d '{\"username\":\"admin\",\"password\":\"Admin@1234!\"}'"
    echo "   2. Use token in Authorization header: -H \"Authorization: Bearer <token>\""
    echo
    echo "========================================"
}

# Main execution function
main() {
    local mode=${1:-"all"}

    echo "🚀 AI-Rust-API - RAG System Startup"
    echo "===================================="

    # Check prerequisites
    check_directory
    check_dependencies
    check_docker

    # Clean up existing services
    print_status "Performing clean restart..."

    # Kill existing services
    kill_port $RAGAPI_PORT "RAGAPI"
    kill_port $BEDROCKAPI_PORT "BedrockAPI"
    kill_port $AUTHAPI_PORT "AuthAPI"

    # Clean up Docker
    cleanup_docker

    # Start PostgreSQL
    start_postgres

    case $mode in
        "all")
            # Start all services
            start_service "AuthAPI" "AuthAPI" $AUTHAPI_PORT
            start_service "BedrockAPI" "BedrockAPI" $BEDROCKAPI_PORT
            start_service "RAGAPI" "RAGAPI" $RAGAPI_PORT
            ;;
        "auth")
            start_service "AuthAPI" "AuthAPI" $AUTHAPI_PORT
            ;;
        "bedrock")
            start_service "AuthAPI" "AuthAPI" $AUTHAPI_PORT
            start_service "BedrockAPI" "BedrockAPI" $BEDROCKAPI_PORT
            ;;
        "rag")
            start_service "AuthAPI" "AuthAPI" $AUTHAPI_PORT
            start_service "RAGAPI" "RAGAPI" $RAGAPI_PORT
            ;;
        *)
            print_error "Invalid mode. Use: all, auth, bedrock, or rag"
            exit 1
            ;;
    esac

    # Show service information
    show_services

    print_success "All services started successfully! 🎉"
    print_status "You can now authenticate and use the RAG system. Default admin credentials: admin / Admin@1234!"
}

# Function to show help
show_help() {
    echo "AI-Rust-API - RAG System Startup Script"
    echo
    echo "Usage: $0 [MODE]"
    echo
    echo "Modes:"
    echo "  all      Start all services (PostgreSQL, AuthAPI, BedrockAPI, RAGAPI) - default"
    echo "  auth     Start only AuthAPI service"
    echo "  bedrock  Start AuthAPI and BedrockAPI services"
    echo "  rag      Start AuthAPI and RAGAPI services"
    echo
    echo "Examples:"
    echo "  $0              # Start everything"
    echo "  $0 all          # Start everything"
    echo "  $0 auth         # Start only authentication service"
    echo "  $0 bedrock      # Start authentication + AI chat services"
    echo "  $0 rag          # Start authentication + document Q&A services"
    echo
    echo "Services:"
    echo "  - AuthAPI: http://localhost:9102 (Authentication)"
    echo "  - BedrockAPI: http://localhost:9100 (AI Chat)"
    echo "  - RAGAPI: http://localhost:9101 (Document Q&A)"
    echo "  - PostgreSQL: Container 'shared_postgres'"
}

# Handle help flag
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    show_help
    exit 0
fi

# Run main function
main "$@"
