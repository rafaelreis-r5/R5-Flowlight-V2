// src-tauri/src/core/search_engine.rs - VERSÃO FINAL BASEADA NA ANÁLISE CORRETA

use anyhow::Result;
use log::{debug, info};
use std::sync::Arc;
use tantivy::{
    collector::TopDocs,
    directory::MmapDirectory,
    doc,
    query::QueryParser,
    schema::{Field, Schema, STORED, TEXT, STRING, OwnedValue},
    Index, IndexReader, IndexWriter, TantivyDocument,
};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone, serde::Serialize)]
pub struct SearchResult {
    pub path: String,
    pub title: String,
    pub score: Option<f32>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AppResult {
    pub name: String,
    pub path: String,
    pub icon: Option<Vec<u8>>,
    pub description: Option<String>,
}

#[derive(Clone)]
struct SearchIndexFields {
    title: Field,
    path: Field,
}

struct SearchIndexSchema {
    schema: Schema,
    fields: SearchIndexFields,
}

impl SearchIndexSchema {
    fn new() -> Self {
        let mut schema_builder = Schema::builder();
        let title = schema_builder.add_text_field("title", TEXT | STORED);
        let path = schema_builder.add_text_field("path", STRING | STORED);
        let schema = schema_builder.build();
        Self { schema, fields: SearchIndexFields { title, path } }
    }
}

pub struct SearchEngine {
    index: Index,
    index_reader: IndexReader,
    pub index_writer: Arc<AsyncMutex<IndexWriter>>,
    schema: SearchIndexSchema,
}

impl SearchEngine {
    pub async fn new() -> Result<Self> {
        let index_path = directories::ProjectDirs::from("com", "r5hub", "flowlight")
            .ok_or_else(|| anyhow::anyhow!("Could not find a valid home directory"))?
            .data_local_dir().join("search_index");

        let lock_file_path = index_path.join(".tantivy-meta.lock");
        if lock_file_path.exists() {
            let _ = tokio::fs::remove_file(lock_file_path).await;
        }

        tokio::fs::create_dir_all(&index_path).await?;
        let schema = SearchIndexSchema::new();
        let dir = MmapDirectory::open(&index_path)?;
        let index = Index::open_or_create(dir, schema.schema.clone())?;
        let index_writer = index.writer(50_000_000)?;
        let index_reader = index.reader()?;

        info!("SearchEngine initialized");
        Ok(Self { index, index_reader, index_writer: Arc::new(AsyncMutex::new(index_writer)), schema })
    }

    pub async fn add_document(&self, path_str: String, title: String) -> Result<()> {
        let mut writer = self.index_writer.lock().await;
        debug!("Adding document: {}", &path_str);
        writer.add_document(doc!(
            self.schema.fields.path => path_str,
            self.schema.fields.title => title
        ))?;
        Ok(())
    }

    pub async fn commit_changes(&self) -> Result<()> {
        self.index_writer.lock().await.commit()?;
        Ok(())
    }

    pub async fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let searcher = self.index_reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, vec![self.schema.fields.title, self.schema.fields.path]);
        let query = query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            // CORREÇÃO: Explicitamente pedimos um `TantivyDocument`, que é a struct concreta.
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

            // Agora, `doc_to_search_result` recebe um tipo concreto que TEM o método `.get_first`.
            if let Some(mut result) = self.doc_to_search_result(&retrieved_doc) {
                result.score = Some(score);
                results.push(result);
            }
        }
        Ok(results)
    }

    // CORREÇÃO: A assinatura da função agora espera a struct concreta `&TantivyDocument`.
    fn doc_to_search_result(&self, doc: &TantivyDocument) -> Option<SearchResult> {
        let path = match doc.get_first(self.schema.fields.path)? {
            OwnedValue::Str(text) => text.clone(),
            _ => return None,
        };
        
        let title = match doc.get_first(self.schema.fields.title)? {
            OwnedValue::Str(text) => text.clone(),
            _ => return None,
        };
        
        Some(SearchResult {
            path,
            title,
            score: None,
        })
    }
}
