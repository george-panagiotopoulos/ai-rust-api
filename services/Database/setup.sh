#!/bin/bash
# ============================================================================
# AI-Rust-API Database Setup Script
# ============================================================================
# This script sets up the complete database environment for the RAG system
# including PostgreSQL with pgvector, database initialization, and data loading
# ============================================================================

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
DB_HOST="${DB_HOST:-localhost}"
DB_PORT="${DB_PORT:-5434}"
DB_NAME="${DB_NAME:-ragdb}"
DB_USER="${DB_USER:-raguser}"
DB_PASSWORD="${DB_PASSWORD:-password}"
DB_URL="postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

print_header() {
    echo -e "${BLUE}============================================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}============================================================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_info() {
    echo -e "${BLUE}ℹ $1${NC}"
}

check_dependencies() {
    print_header "Checking Dependencies"
    
    # Check if Docker is installed
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    print_success "Docker is installed"
    
    # Check if Docker Compose is installed
    if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
        print_error "Docker Compose is not installed. Please install Docker Compose first."
        exit 1
    fi
    print_success "Docker Compose is installed"
    
    # Check if psql is installed
    if ! command -v psql &> /dev/null; then
        print_warning "psql is not installed. Database operations will be limited."
        print_info "To install psql on macOS: brew install postgresql"
        print_info "To install psql on Ubuntu: sudo apt-get install postgresql-client"
    else
        print_success "psql is installed"
    fi
}

start_database() {
    print_header "Starting PostgreSQL with pgvector"
    
    cd "$PROJECT_ROOT/services/RAGAPI"
    
    if [ -f "docker-compose.yml" ]; then
        print_info "Starting PostgreSQL container..."
        docker-compose up -d postgres
        
        # Wait for PostgreSQL to be ready
        print_info "Waiting for PostgreSQL to be ready..."
        for i in {1..30}; do
            if docker-compose exec -T postgres pg_isready -h localhost -p 5432 -U "$DB_USER" > /dev/null 2>&1; then
                print_success "PostgreSQL is ready"
                break
            fi
            sleep 1
            echo -n "."
        done
        
        if [ $i -eq 30 ]; then
            print_error "PostgreSQL failed to start within 30 seconds"
            exit 1
        fi
    else
        print_error "docker-compose.yml not found in services/RAGAPI"
        exit 1
    fi
}

initialize_database() {
    print_header "Initializing Database Schema"
    
    if command -v psql &> /dev/null; then
        print_info "Running database initialization script..."
        psql "$DB_URL" -f "$SCRIPT_DIR/init.sql" -v ON_ERROR_STOP=1
        print_success "Database schema initialized successfully"
    else
        print_warning "psql not available - cannot initialize database schema"
        print_info "Please install psql and run: psql $DB_URL -f $SCRIPT_DIR/init.sql"
    fi
}

create_sample_data() {
    print_header "Creating Sample Data (Optional)"
    
    if command -v psql &> /dev/null; then
        read -p "Do you want to create sample data for testing? (y/N): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            if [ -f "$SCRIPT_DIR/sample_data.sql" ]; then
                print_info "Creating sample data..."
                psql "$DB_URL" -f "$SCRIPT_DIR/sample_data.sql" -v ON_ERROR_STOP=1
                print_success "Sample data created successfully"
            else
                print_warning "Sample data script not found"
            fi
        fi
    fi
}

verify_setup() {
    print_header "Verifying Database Setup"
    
    if command -v psql &> /dev/null; then
        print_info "Checking database connection..."
        if psql "$DB_URL" -c "SELECT 1;" > /dev/null 2>&1; then
            print_success "Database connection successful"
        else
            print_error "Database connection failed"
            exit 1
        fi
        
        print_info "Checking table creation..."
        TABLES=$(psql "$DB_URL" -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='public' AND table_type='BASE TABLE';")
        print_success "Found $TABLES tables in the database"
        
        print_info "Checking pgvector extension..."
        EXTENSION=$(psql "$DB_URL" -t -c "SELECT COUNT(*) FROM pg_extension WHERE extname='vector';")
        if [ "$EXTENSION" -eq 1 ]; then
            print_success "pgvector extension is installed"
        else
            print_error "pgvector extension is not installed"
        fi
        
        print_info "Checking default admin user..."
        ADMIN_COUNT=$(psql "$DB_URL" -t -c "SELECT COUNT(*) FROM users WHERE username='admin' AND is_admin=true;")
        if [ "$ADMIN_COUNT" -eq 1 ]; then
            print_success "Default admin user exists"
        else
            print_warning "Default admin user not found"
        fi
    fi
}

show_connection_info() {
    print_header "Database Connection Information"
    echo -e "${BLUE}Database URL:${NC} $DB_URL"
    echo -e "${BLUE}Host:${NC} $DB_HOST"
    echo -e "${BLUE}Port:${NC} $DB_PORT"
    echo -e "${BLUE}Database:${NC} $DB_NAME"
    echo -e "${BLUE}Username:${NC} $DB_USER"
    echo -e "${BLUE}Password:${NC} $DB_PASSWORD"
    echo ""
    echo -e "${YELLOW}Default Admin Credentials:${NC}"
    echo -e "${BLUE}Username:${NC} admin"
    echo -e "${BLUE}Password:${NC} password"
    echo -e "${RED}⚠ CHANGE THE DEFAULT PASSWORD IN PRODUCTION!${NC}"
}

main() {
    print_header "AI-Rust-API Database Setup"
    echo -e "${BLUE}This script will set up the complete database environment for the RAG system.${NC}"
    echo ""
    
    check_dependencies
    start_database
    sleep 3  # Give PostgreSQL a moment to fully initialize
    initialize_database
    create_sample_data
    verify_setup
    show_connection_info
    
    print_header "Setup Complete!"
    print_success "Database is ready for use"
    print_info "You can now start the RAG system services"
    print_info "Use './start.sh' from the project root to start all services"
}

# Handle command line arguments
case "${1:-}" in
    "check")
        check_dependencies
        ;;
    "start")
        start_database
        ;;
    "init")
        initialize_database
        ;;
    "verify")
        verify_setup
        ;;
    "info")
        show_connection_info
        ;;
    "help"|"-h"|"--help")
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  (no command)  Run full setup process"
        echo "  check         Check dependencies only"
        echo "  start         Start database only"
        echo "  init          Initialize database schema only"
        echo "  verify        Verify database setup"
        echo "  info          Show connection information"
        echo "  help          Show this help message"
        echo ""
        echo "Environment variables:"
        echo "  DB_HOST      Database host (default: localhost)"
        echo "  DB_PORT      Database port (default: 5434)"
        echo "  DB_NAME      Database name (default: ragdb)"
        echo "  DB_USER      Database user (default: raguser)"
        echo "  DB_PASSWORD  Database password (default: password)"
        ;;
    *)
        main
        ;;
esac