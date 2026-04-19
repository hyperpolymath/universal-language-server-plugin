//! Concurrent document storage using lock-free DashMap
//!
//! Provides thread-safe document management with minimal contention.

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Document metadata and content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Unique document identifier
    pub id: String,
    /// Document URI (file path or virtual identifier)
    pub uri: String,
    /// Document content
    pub content: String,
    /// Document format/language
    pub language: String,
    /// Document version (incremented on each change)
    pub version: i32,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last modification timestamp
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

impl Document {
    /// Create a new document
    pub fn new(uri: String, content: String, language: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            uri,
            content,
            language,
            version: 1,
            created_at: now,
            modified_at: now,
        }
    }

    /// Update document content (increments version)
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.version += 1;
        self.modified_at = chrono::Utc::now();
    }

    /// Get document statistics
    pub fn stats(&self) -> DocumentStats {
        DocumentStats {
            lines: self.content.lines().count(),
            characters: self.content.len(),
            words: self.content.split_whitespace().count(),
            version: self.version,
        }
    }
}

/// Document statistics for hover/info displays
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentStats {
    pub lines: usize,
    pub characters: usize,
    pub words: usize,
    pub version: i32,
}

/// Thread-safe document store using lock-free concurrent HashMap
pub struct DocumentStore {
    /// Documents indexed by URI
    documents: DashMap<String, Document>,
}

impl DocumentStore {
    /// Create a new empty document store
    pub fn new() -> Self {
        Self {
            documents: DashMap::new(),
        }
    }

    /// Insert or update a document
    pub fn upsert(&self, uri: String, content: String, language: String) -> Arc<Document> {
        self.documents
            .entry(uri.clone())
            .and_modify(|doc| doc.update_content(content.clone()))
            .or_insert_with(|| Document::new(uri, content, language))
            .clone()
            .into()
    }

    /// Get a document by URI
    pub fn get(&self, uri: &str) -> Option<Document> {
        self.documents.get(uri).map(|doc| doc.clone())
    }

    /// Get a document by ID
    pub fn get_by_id(&self, id: &str) -> Option<Document> {
        self.documents
            .iter()
            .find(|entry| entry.value().id == id)
            .map(|entry| entry.value().clone())
    }

    /// Remove a document by URI
    pub fn remove(&self, uri: &str) -> Option<Document> {
        self.documents.remove(uri).map(|(_, doc)| doc)
    }

    /// List all documents
    pub fn list(&self) -> Vec<Document> {
        self.documents
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// Get document count
    pub fn count(&self) -> usize {
        self.documents.len()
    }

    /// Clear all documents
    pub fn clear(&self) {
        self.documents.clear();
    }

    /// Check if a document exists
    pub fn contains(&self, uri: &str) -> bool {
        self.documents.contains_key(uri)
    }
}

impl Default for DocumentStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new(
            "file:///test.md".to_string(),
            "# Hello".to_string(),
            "markdown".to_string(),
        );
        assert_eq!(doc.version, 1);
        assert_eq!(doc.content, "# Hello");
        assert_eq!(doc.language, "markdown");
    }

    #[test]
    fn test_document_update() {
        let mut doc = Document::new(
            "file:///test.md".to_string(),
            "# Hello".to_string(),
            "markdown".to_string(),
        );
        let v1 = doc.version;
        doc.update_content("# Hello World".to_string());
        assert_eq!(doc.version, v1 + 1);
        assert_eq!(doc.content, "# Hello World");
    }

    #[test]
    fn test_document_stats() {
        let doc = Document::new(
            "file:///test.md".to_string(),
            "# Hello World\nThis is a test.".to_string(),
            "markdown".to_string(),
        );
        let stats = doc.stats();
        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 7); // "# Hello World This is a test." = 7 words
    }

    #[test]
    fn test_store_operations() {
        let store = DocumentStore::new();

        // Insert
        store.upsert(
            "file:///test.md".to_string(),
            "# Hello".to_string(),
            "markdown".to_string(),
        );
        assert_eq!(store.count(), 1);

        // Get
        let doc = store.get("file:///test.md").unwrap();
        assert_eq!(doc.content, "# Hello");

        // Update
        store.upsert(
            "file:///test.md".to_string(),
            "# Hello World".to_string(),
            "markdown".to_string(),
        );
        let doc = store.get("file:///test.md").unwrap();
        assert_eq!(doc.content, "# Hello World");
        assert_eq!(doc.version, 2);

        // Remove
        store.remove("file:///test.md");
        assert_eq!(store.count(), 0);
    }

    #[test]
    fn test_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let store = Arc::new(DocumentStore::new());
        let mut handles = vec![];

        // Spawn 10 threads, each inserting 100 documents
        for i in 0..10 {
            let store_clone = Arc::clone(&store);
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    let uri = format!("file:///test_{i}_{j}.md");
                    store_clone.upsert(
                        uri,
                        format!("Content {i} {j}"),
                        "markdown".to_string(),
                    );
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(store.count(), 1000);
    }
}
