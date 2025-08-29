use std::path::{Path, PathBuf};
use tokio::fs;
use anyhow::{Result, anyhow};
use tracing::{warn, error};

pub struct FileUtils;

impl FileUtils {
    pub fn sanitize_filename(filename: &str) -> Option<String> {
        if filename.is_empty() 
            || filename.contains("..")
            || filename.contains("/")
            || filename.contains("\\")
            || filename.starts_with('.')
        {
            return None;
        }

        let sanitized = filename
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
            .collect::<String>();

        if sanitized.is_empty() || sanitized.len() > 255 {
            return None;
        }

        Some(sanitized)
    }

    pub fn is_allowed_extension(filename: &str, allowed_extensions: &[&str]) -> bool {
        if let Some(extension) = filename.split('.').last() {
            allowed_extensions.contains(&extension.to_lowercase().as_str())
        } else {
            false
        }
    }

    pub async fn ensure_directory_exists(path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir_all(path).await
                .map_err(|e| anyhow!("Failed to create directory {}: {}", path.display(), e))?;
        }
        Ok(())
    }

    pub fn get_directory_size(path: &Path) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<u64>> + Send + '_>> {
        Box::pin(async move {
            let mut total_size = 0u64;
            let mut entries = fs::read_dir(path).await
                .map_err(|e| anyhow!("Failed to read directory {}: {}", path.display(), e))?;

            while let Some(entry) = entries.next_entry().await
                .map_err(|e| anyhow!("Failed to read directory entry: {}", e))? 
            {
                let metadata = entry.metadata().await
                    .map_err(|e| anyhow!("Failed to get metadata: {}", e))?;

                if metadata.is_file() {
                    total_size += metadata.len();
                } else if metadata.is_dir() {
                    match Self::get_directory_size(&entry.path()).await {
                        Ok(size) => total_size += size,
                        Err(e) => warn!("Failed to get size for subdirectory {}: {}", entry.path().display(), e),
                    }
                }
            }

            Ok(total_size)
        })
    }

    pub async fn count_files_in_directory(path: &Path) -> Result<usize> {
        let mut count = 0;
        let mut entries = fs::read_dir(path).await
            .map_err(|e| anyhow!("Failed to read directory {}: {}", path.display(), e))?;

        while let Some(entry) = entries.next_entry().await
            .map_err(|e| anyhow!("Failed to read directory entry: {}", e))?
        {
            let metadata = entry.metadata().await
                .map_err(|e| anyhow!("Failed to get metadata: {}", e))?;

            if metadata.is_file() {
                count += 1;
            }
        }

        Ok(count)
    }

    pub async fn safe_delete_file(file_path: &Path) -> Result<()> {
        if !file_path.exists() {
            return Err(anyhow!("File does not exist: {}", file_path.display()));
        }

        if !file_path.is_file() {
            return Err(anyhow!("Path is not a file: {}", file_path.display()));
        }

        fs::remove_file(file_path).await
            .map_err(|e| anyhow!("Failed to delete file {}: {}", file_path.display(), e))
    }

    pub async fn safe_delete_directory(dir_path: &Path) -> Result<()> {
        if !dir_path.exists() {
            return Err(anyhow!("Directory does not exist: {}", dir_path.display()));
        }

        if !dir_path.is_dir() {
            return Err(anyhow!("Path is not a directory: {}", dir_path.display()));
        }

        fs::remove_dir_all(dir_path).await
            .map_err(|e| anyhow!("Failed to delete directory {}: {}", dir_path.display(), e))
    }

    pub fn get_file_extension(filename: &str) -> Option<String> {
        filename.split('.').last().map(|s| s.to_lowercase())
    }

    pub fn generate_unique_filename(base_name: &str, extension: &str) -> String {
        let timestamp = chrono::Utc::now().timestamp();
        let random: u32 = rand::random();
        format!("{}_{}_{}_{}.{}", base_name, timestamp, random, base_name.len(), extension)
    }

    pub async fn validate_file_content(file_path: &Path, max_size: usize) -> Result<()> {
        let metadata = fs::metadata(file_path).await
            .map_err(|e| anyhow!("Failed to get file metadata: {}", e))?;

        if metadata.len() > max_size as u64 {
            return Err(anyhow!("File too large: {} bytes (max: {} bytes)", metadata.len(), max_size));
        }

        Ok(())
    }

    pub async fn create_backup_path(original_path: &Path) -> Result<PathBuf> {
        let parent = original_path.parent()
            .ok_or_else(|| anyhow!("Cannot determine parent directory"))?;
        
        let file_stem = original_path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow!("Cannot determine file stem"))?;
        
        let extension = original_path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        
        let backup_filename = if extension.is_empty() {
            format!("{}_backup_{}", file_stem, timestamp)
        } else {
            format!("{}_backup_{}.{}", file_stem, timestamp, extension)
        };

        Ok(parent.join(backup_filename))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(FileUtils::sanitize_filename("test.txt"), Some("test.txt".to_string()));
        assert_eq!(FileUtils::sanitize_filename("test_file-2.pdf"), Some("test_file-2.pdf".to_string()));
        assert_eq!(FileUtils::sanitize_filename("../malicious.txt"), None);
        assert_eq!(FileUtils::sanitize_filename("file/with/path.txt"), None);
        assert_eq!(FileUtils::sanitize_filename(".hidden"), None);
        assert_eq!(FileUtils::sanitize_filename(""), None);
    }

    #[test]
    fn test_is_allowed_extension() {
        let allowed = &["txt", "pdf", "doc"];
        
        assert!(FileUtils::is_allowed_extension("test.txt", allowed));
        assert!(FileUtils::is_allowed_extension("document.PDF", allowed));
        assert!(!FileUtils::is_allowed_extension("script.exe", allowed));
        assert!(!FileUtils::is_allowed_extension("noextension", allowed));
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(FileUtils::get_file_extension("test.txt"), Some("txt".to_string()));
        assert_eq!(FileUtils::get_file_extension("file.PDF"), Some("pdf".to_string()));
        assert_eq!(FileUtils::get_file_extension("noextension"), None);
        assert_eq!(FileUtils::get_file_extension("multiple.ext.txt"), Some("txt".to_string()));
    }

    #[test]
    fn test_generate_unique_filename() {
        let filename1 = FileUtils::generate_unique_filename("test", "txt");
        let filename2 = FileUtils::generate_unique_filename("test", "txt");
        
        assert_ne!(filename1, filename2);
        assert!(filename1.starts_with("test_"));
        assert!(filename1.ends_with(".txt"));
    }
}