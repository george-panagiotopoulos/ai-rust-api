#!/usr/bin/env python3
"""
Document Vectorization Script for RAGAPI

This script processes documents from the documents/ directory and sends them
to the RAGAPI for vectorization and storage.
"""

import os
import sys
import requests
import json
import hashlib
from pathlib import Path
from typing import List, Tuple, Optional

try:
    import pdfplumber
    HAS_PDF = True
except ImportError:
    HAS_PDF = False

try:
    from docx import Document
    HAS_DOCX = True
except ImportError:
    HAS_DOCX = False

# Configuration
RAG_API_BASE_URL = "http://localhost:9101"  # Default RAGAPI port
DOCUMENTS_DIR = "../documents"  # Relative to scripts directory

class DocumentProcessor:
    def __init__(self, api_base_url: str = RAG_API_BASE_URL):
        self.api_base_url = api_base_url.rstrip('/')
        self.processed_files = set()

    def get_file_hash(self, filepath: str) -> str:
        """Calculate SHA256 hash of file content."""
        hash_sha256 = hashlib.sha256()
        with open(filepath, "rb") as f:
            for chunk in iter(lambda: f.read(4096), b""):
                hash_sha256.update(chunk)
        return hash_sha256.hexdigest()

    def extract_text_from_pdf(self, filepath: str) -> str:
        """Extract text from PDF file."""
        if not HAS_PDF:
            raise ImportError("pdfplumber not installed. Install with: pip install pdfplumber")

        text = ""
        with pdfplumber.open(filepath) as pdf:
            for page in pdf.pages:
                page_text = page.extract_text()
                if page_text:
                    text += page_text + "\n"
        return text.strip()

    def chunk_text(self, text: str, chunk_size: int = 1000, overlap: int = 200) -> List[str]:
        """Split text into chunks with overlap."""
        if len(text) <= chunk_size:
            return [text]

        chunks = []
        start = 0

        while start < len(text):
            end = start + chunk_size

            # Try to find a good breaking point (sentence end)
            if end < len(text):
                # Look for sentence endings within the last 200 characters
                search_start = max(start, end - 200)
                sentence_endings = ['. ', '! ', '? ', '\n\n']

                best_break = end
                for ending in sentence_endings:
                    last_ending = text.rfind(ending, search_start, end)
                    if last_ending != -1 and last_ending > best_break - 100:
                        best_break = last_ending + len(ending)
                        break

                end = best_break

            chunk = text[start:end].strip()
            if chunk:
                chunks.append(chunk)

            # Move start position with overlap
            start = max(start + 1, end - overlap)

        return chunks

    def extract_text_from_docx(self, filepath: str) -> str:
        """Extract text from DOCX file."""
        if not HAS_DOCX:
            raise ImportError("python-docx not installed. Install with: pip install python-docx")

        doc = Document(filepath)
        text = ""
        for paragraph in doc.paragraphs:
            text += paragraph.text + "\n"
        return text.strip()

    def extract_text_from_txt(self, filepath: str) -> str:
        """Extract text from plain text file."""
        with open(filepath, 'r', encoding='utf-8') as f:
            return f.read().strip()

    def extract_text_from_md(self, filepath: str) -> str:
        """Extract text from Markdown file."""
        return self.extract_text_from_txt(filepath)

    def extract_text(self, filepath: str) -> str:
        """Extract text from file based on extension."""
        file_ext = Path(filepath).suffix.lower()

        if file_ext == '.pdf':
            return self.extract_text_from_pdf(filepath)
        elif file_ext == '.docx':
            return self.extract_text_from_docx(filepath)
        elif file_ext in ['.txt', '.md']:
            return self.extract_text_from_txt(filepath)
        else:
            raise ValueError(f"Unsupported file type: {file_ext}")

    def send_to_rag_api(self, filename: str, content: str) -> dict:
        """Send document to RAGAPI for processing."""
        url = f"{self.api_base_url}/process-document"
        payload = {
            "filename": filename,
            "content": content
        }

        try:
            response = requests.post(url, json=payload, timeout=300)
            response.raise_for_status()
            return response.json()
        except requests.exceptions.RequestException as e:
            print(f"Error sending {filename} to API: {e}")
            raise

    def process_file(self, filepath: str) -> bool:
        """Process a single file."""
        try:
            filename = Path(filepath).name
            print(f"Processing: {filename}")

            # Check if file was already processed (basic check)
            file_hash = self.get_file_hash(filepath)
            if file_hash in self.processed_files:
                print(f"  Skipping (already processed): {filename}")
                return True

            # Extract text
            content = self.extract_text(filepath)
            if not content:
                print(f"  Warning: No text extracted from {filename}")
                return False

            print(f"  Extracted {len(content)} characters")

            # Split into chunks for better processing
            chunks = self.chunk_text(content)
            print(f"  Split into {len(chunks)} chunks")

            # Process each chunk
            successful_chunks = 0
            for i, chunk in enumerate(chunks):
                try:
                    # Create a unique filename for each chunk
                    chunk_filename = f"{filename} [Chunk {i+1}]"

                    print(f"    Processing chunk {i+1}/{len(chunks)} ({len(chunk)} chars)")

                    # Send chunk to RAG API
                    result = self.send_to_rag_api(chunk_filename, chunk)
                    print(f"      Success: {result.get('message', 'Processed successfully')}")
                    successful_chunks += 1

                except Exception as chunk_error:
                    print(f"      Error processing chunk {i+1}: {chunk_error}")
                    continue

            if successful_chunks > 0:
                print(f"  Successfully processed {successful_chunks}/{len(chunks)} chunks")
                self.processed_files.add(file_hash)
                return True
            else:
                print(f"  Failed to process any chunks")
                return False

        except Exception as e:
            print(f"  Error processing {filename}: {e}")
            return False

    def process_directory(self, directory: str) -> Tuple[int, int]:
        """Process all files in directory."""
        dir_path = Path(directory)

        if not dir_path.exists():
            raise FileNotFoundError(f"Directory not found: {directory}")

        # Supported file extensions
        supported_exts = ['.pdf', '.docx', '.txt', '.md']

        files = []
        for ext in supported_exts:
            files.extend(dir_path.glob(f"**/*{ext}"))

        if not files:
            print(f"No supported files found in {directory}")
            print(f"Supported formats: {', '.join(supported_exts)}")
            return 0, 0

        print(f"Found {len(files)} files to process")

        success_count = 0
        total_count = len(files)

        for filepath in files:
            if self.process_file(str(filepath)):
                success_count += 1

        return success_count, total_count

def check_api_health() -> bool:
    """Check if RAGAPI is running and healthy."""
    try:
        response = requests.get(f"{RAG_API_BASE_URL}/health", timeout=10)
        response.raise_for_status()
        data = response.json()
        print(f"API Health: {data.get('status', 'Unknown')}")
        return True
    except requests.exceptions.RequestException as e:
        print(f"API health check failed: {e}")
        print("Make sure the RAGAPI server is running on the correct port.")
        return False

def install_dependencies():
    """Install required Python packages."""
    packages = []
    if not HAS_PDF:
        packages.append("pdfplumber")
    if not HAS_DOCX:
        packages.append("python-docx")

    if packages:
        print(f"Installing missing dependencies: {', '.join(packages)}")
        os.system(f"pip install {' '.join(packages)}")
        print("Please restart the script after installation.")
        return False
    return True

def main():
    print("RAG Document Vectorization Script")
    print("=" * 40)

    # Check dependencies
    if not install_dependencies():
        return

    # Check API health
    if not check_api_health():
        print("\nTo start the RAGAPI server:")
        print("  cd services/RAGAPI")
        print("  cargo run")
        return

    # Initialize processor
    processor = DocumentProcessor()

    # Process documents
    try:
        success_count, total_count = processor.process_directory(DOCUMENTS_DIR)
        print(f"\nProcessing complete: {success_count}/{total_count} files successful")

        if success_count > 0:
            print("\nYou can now query your documents using the RAGAPI!")
            print("Example: curl -X POST http://localhost:9101/query -H 'Content-Type: application/json' -d '{\"query\":\"your question here\"}'")

    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
