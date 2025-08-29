#!/bin/bash
# ============================================================================
# AI-Rust-API Database Backup Script
# ============================================================================
# This script creates backups of the RAG system database
# Supports full backups, schema-only backups, and data-only backups
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

# Script directory and backup directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BACKUP_DIR="${BACKUP_DIR:-$SCRIPT_DIR/backups}"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_PREFIX="ragdb_backup"

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

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
    if ! command -v pg_dump &> /dev/null; then
        print_error "pg_dump is not installed. Please install PostgreSQL client tools."
        print_info "To install on macOS: brew install postgresql"
        print_info "To install on Ubuntu: sudo apt-get install postgresql-client"
        exit 1
    fi
    
    if ! command -v psql &> /dev/null; then
        print_error "psql is not installed. Please install PostgreSQL client tools."
        exit 1
    fi
    
    print_success "PostgreSQL client tools are available"
}

test_connection() {
    print_info "Testing database connection..."
    if psql "$DB_URL" -c "SELECT 1;" > /dev/null 2>&1; then
        print_success "Database connection successful"
    else
        print_error "Cannot connect to database"
        print_error "URL: $DB_URL"
        exit 1
    fi
}

create_full_backup() {
    local backup_file="${BACKUP_DIR}/${BACKUP_PREFIX}_full_${TIMESTAMP}.sql"
    
    print_info "Creating full database backup..."
    print_info "Backup file: $backup_file"
    
    pg_dump "$DB_URL" \
        --verbose \
        --no-password \
        --format=plain \
        --no-privileges \
        --no-tablespaces \
        --file="$backup_file" \
        || { print_error "Full backup failed"; exit 1; }
    
    # Compress the backup
    if command -v gzip &> /dev/null; then
        print_info "Compressing backup..."
        gzip "$backup_file"
        backup_file="${backup_file}.gz"
    fi
    
    local size=$(du -h "$backup_file" | cut -f1)
    print_success "Full backup created successfully"
    print_info "File: $backup_file"
    print_info "Size: $size"
    
    echo "$backup_file"
}

create_schema_backup() {
    local backup_file="${BACKUP_DIR}/${BACKUP_PREFIX}_schema_${TIMESTAMP}.sql"
    
    print_info "Creating schema-only backup..."
    print_info "Backup file: $backup_file"
    
    pg_dump "$DB_URL" \
        --verbose \
        --no-password \
        --format=plain \
        --schema-only \
        --no-privileges \
        --no-tablespaces \
        --file="$backup_file" \
        || { print_error "Schema backup failed"; exit 1; }
    
    local size=$(du -h "$backup_file" | cut -f1)
    print_success "Schema backup created successfully"
    print_info "File: $backup_file"
    print_info "Size: $size"
    
    echo "$backup_file"
}

create_data_backup() {
    local backup_file="${BACKUP_DIR}/${BACKUP_PREFIX}_data_${TIMESTAMP}.sql"
    
    print_info "Creating data-only backup..."
    print_info "Backup file: $backup_file"
    
    pg_dump "$DB_URL" \
        --verbose \
        --no-password \
        --format=plain \
        --data-only \
        --no-privileges \
        --no-tablespaces \
        --file="$backup_file" \
        || { print_error "Data backup failed"; exit 1; }
    
    # Compress the backup
    if command -v gzip &> /dev/null; then
        print_info "Compressing backup..."
        gzip "$backup_file"
        backup_file="${backup_file}.gz"
    fi
    
    local size=$(du -h "$backup_file" | cut -f1)
    print_success "Data backup created successfully"
    print_info "File: $backup_file"
    print_info "Size: $size"
    
    echo "$backup_file"
}

create_custom_backup() {
    local backup_file="${BACKUP_DIR}/${BACKUP_PREFIX}_custom_${TIMESTAMP}.dump"
    
    print_info "Creating custom format backup..."
    print_info "Backup file: $backup_file"
    
    pg_dump "$DB_URL" \
        --verbose \
        --no-password \
        --format=custom \
        --compress=9 \
        --no-privileges \
        --no-tablespaces \
        --file="$backup_file" \
        || { print_error "Custom backup failed"; exit 1; }
    
    local size=$(du -h "$backup_file" | cut -f1)
    print_success "Custom backup created successfully"
    print_info "File: $backup_file"
    print_info "Size: $size"
    
    echo "$backup_file"
}

cleanup_old_backups() {
    local keep_days=${1:-7}
    
    print_info "Cleaning up backups older than $keep_days days..."
    
    local deleted_count=0
    while IFS= read -r -d '' file; do
        rm "$file"
        ((deleted_count++))
        print_info "Deleted: $(basename "$file")"
    done < <(find "$BACKUP_DIR" -name "${BACKUP_PREFIX}_*" -type f -mtime "+$keep_days" -print0)
    
    if [ $deleted_count -eq 0 ]; then
        print_info "No old backups to clean up"
    else
        print_success "Cleaned up $deleted_count old backup files"
    fi
}

list_backups() {
    print_header "Available Backups"
    
    if [ ! -d "$BACKUP_DIR" ]; then
        print_warning "Backup directory does not exist: $BACKUP_DIR"
        return
    fi
    
    local backup_files=("$BACKUP_DIR"/${BACKUP_PREFIX}_*)
    
    if [ ! -e "${backup_files[0]}" ]; then
        print_warning "No backups found in: $BACKUP_DIR"
        return
    fi
    
    echo -e "${BLUE}File Name${NC}\t\t\t\t${BLUE}Size${NC}\t${BLUE}Date${NC}"
    echo "--------------------------------------------------------------------------------"
    
    for file in "${backup_files[@]}"; do
        if [ -f "$file" ]; then
            local filename=$(basename "$file")
            local size=$(du -h "$file" | cut -f1)
            local date=$(date -r "$file" "+%Y-%m-%d %H:%M:%S")
            printf "%-40s\t%s\t%s\n" "$filename" "$size" "$date"
        fi
    done
}

restore_backup() {
    local backup_file="$1"
    
    if [ -z "$backup_file" ]; then
        print_error "Backup file not specified"
        exit 1
    fi
    
    if [ ! -f "$backup_file" ]; then
        print_error "Backup file not found: $backup_file"
        exit 1
    fi
    
    print_header "Restoring Database from Backup"
    print_warning "This will OVERWRITE the existing database!"
    read -p "Are you sure you want to continue? (yes/no): " -r
    
    if [[ ! $REPLY == "yes" ]]; then
        print_info "Restore cancelled"
        exit 0
    fi
    
    print_info "Restoring from: $backup_file"
    
    # Determine restore method based on file extension
    if [[ "$backup_file" == *.dump ]]; then
        # Custom format
        pg_restore "$backup_file" \
            --dbname="$DB_URL" \
            --verbose \
            --clean \
            --if-exists \
            --no-privileges \
            --no-tablespaces \
            || { print_error "Restore failed"; exit 1; }
    elif [[ "$backup_file" == *.gz ]]; then
        # Compressed SQL file
        gunzip -c "$backup_file" | psql "$DB_URL" -v ON_ERROR_STOP=1 \
            || { print_error "Restore failed"; exit 1; }
    else
        # Plain SQL file
        psql "$DB_URL" -f "$backup_file" -v ON_ERROR_STOP=1 \
            || { print_error "Restore failed"; exit 1; }
    fi
    
    print_success "Database restored successfully"
}

show_help() {
    echo "AI-Rust-API Database Backup Script"
    echo ""
    echo "Usage: $0 [command] [options]"
    echo ""
    echo "Commands:"
    echo "  full                Create full database backup (default)"
    echo "  schema              Create schema-only backup"
    echo "  data                Create data-only backup"
    echo "  custom              Create custom format backup"
    echo "  list                List available backups"
    echo "  cleanup [days]      Clean up backups older than N days (default: 7)"
    echo "  restore <file>      Restore database from backup file"
    echo "  help                Show this help message"
    echo ""
    echo "Environment variables:"
    echo "  DB_HOST             Database host (default: localhost)"
    echo "  DB_PORT             Database port (default: 5434)"
    echo "  DB_NAME             Database name (default: ragdb)"
    echo "  DB_USER             Database user (default: raguser)"
    echo "  DB_PASSWORD         Database password (default: password)"
    echo "  BACKUP_DIR          Backup directory (default: ./backups)"
    echo ""
    echo "Examples:"
    echo "  $0                  # Create full backup"
    echo "  $0 schema           # Create schema-only backup"
    echo "  $0 cleanup 30       # Clean up backups older than 30 days"
    echo "  $0 restore backup.sql # Restore from backup file"
}

main() {
    local command="${1:-full}"
    
    case "$command" in
        "full"|"")
            print_header "AI-Rust-API Database Backup - Full Backup"
            check_dependencies
            test_connection
            create_full_backup
            ;;
        "schema")
            print_header "AI-Rust-API Database Backup - Schema Only"
            check_dependencies
            test_connection
            create_schema_backup
            ;;
        "data")
            print_header "AI-Rust-API Database Backup - Data Only"
            check_dependencies
            test_connection
            create_data_backup
            ;;
        "custom")
            print_header "AI-Rust-API Database Backup - Custom Format"
            check_dependencies
            test_connection
            create_custom_backup
            ;;
        "list")
            list_backups
            ;;
        "cleanup")
            print_header "AI-Rust-API Database Backup - Cleanup"
            cleanup_old_backups "$2"
            ;;
        "restore")
            restore_backup "$2"
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            print_error "Unknown command: $command"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

main "$@"