// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//! Performance benchmarks for critical LSP operations
//!
//! Run with: cargo bench --manifest-path server/Cargo.toml

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use universal_connector_server::core::{ConversionCore, ConversionRequest, Format};
use universal_connector_server::document_store::{Document, DocumentStore};

fn benchmark_toml_parse_throughput(c: &mut Criterion) {
    let toml_content = black_box(
        "[server]\nhost = \"localhost\"\nport = 8080\n[database]\nurl = \"postgres://localhost\"",
    );

    c.bench_function("toml_parse", |b| {
        b.iter(|| {
            let request = ConversionRequest {
                content: toml_content.to_string(),
                from: Format::Toml,
                to: Format::Json,
            };
            let _ = ConversionCore::convert(request);
        })
    });
}

fn benchmark_yaml_parse_throughput(c: &mut Criterion) {
    let yaml_content = black_box(
        "server:\n  host: localhost\n  port: 8080\ndatabase:\n  url: postgres://localhost",
    );

    c.bench_function("yaml_parse", |b| {
        b.iter(|| {
            let request = ConversionRequest {
                content: yaml_content.to_string(),
                from: Format::Yaml,
                to: Format::Json,
            };
            let _ = ConversionCore::convert(request);
        })
    });
}

fn benchmark_markdown_to_html(c: &mut Criterion) {
    let md_content = black_box(
        "# Title\n\nThis is a paragraph with **bold** and *italic* text.\n\n## Section\n\nMore content here."
    );

    c.bench_function("markdown_to_html", |b| {
        b.iter(|| {
            let request = ConversionRequest {
                content: md_content.to_string(),
                from: Format::Markdown,
                to: Format::Html,
            };
            let _ = ConversionCore::convert(request);
        })
    });
}

fn benchmark_json_to_yaml(c: &mut Criterion) {
    let json_content = black_box(
        r#"{"server": {"host": "localhost", "port": 8080}, "database": {"url": "postgres://localhost"}}"#
    );

    c.bench_function("json_to_yaml", |b| {
        b.iter(|| {
            let request = ConversionRequest {
                content: json_content.to_string(),
                from: Format::Json,
                to: Format::Yaml,
            };
            let _ = ConversionCore::convert(request);
        })
    });
}

fn benchmark_document_store_insert(c: &mut Criterion) {
    c.bench_function("document_store_insert", |b| {
        let store = DocumentStore::new();
        let mut counter = 0;
        b.iter(|| {
            let uri = black_box(format!("file:///doc{}", counter));
            store.upsert(uri, "test content".to_string(), "markdown".to_string());
            counter += 1;
        })
    });
}

fn benchmark_document_store_get(c: &mut Criterion) {
    let store = DocumentStore::new();
    let uri = "file:///test".to_string();
    store.upsert(uri.clone(), "test content".to_string(), "markdown".to_string());

    c.bench_function("document_store_get", |b| {
        b.iter(|| {
            let _ = store.get(black_box(&uri));
        })
    });
}

fn benchmark_document_store_contains(c: &mut Criterion) {
    let store = DocumentStore::new();
    let uri = "file:///test".to_string();
    store.upsert(uri.clone(), "test content".to_string(), "markdown".to_string());

    c.bench_function("document_store_contains", |b| {
        b.iter(|| {
            let _ = store.contains(black_box(&uri));
        })
    });
}

fn benchmark_document_create(c: &mut Criterion) {
    c.bench_function("document_create", |b| {
        let uri = black_box("file:///test".to_string());
        let content = black_box("# Test Document\n\nWith content.".to_string());
        let lang = black_box("markdown".to_string());

        b.iter(|| {
            let _ = Document::new(uri.clone(), content.clone(), lang.clone());
        })
    });
}

fn benchmark_document_stats(c: &mut Criterion) {
    let doc = Document::new(
        "file:///test".to_string(),
        "# Title\n\nThis is content.\n\nWith multiple lines.".to_string(),
        "markdown".to_string(),
    );

    c.bench_function("document_stats", |b| {
        b.iter(|| {
            let _ = doc.stats();
        })
    });
}

fn benchmark_document_update(c: &mut Criterion) {
    c.bench_function("document_update", |b| {
        let mut doc = Document::new(
            "file:///test".to_string(),
            "initial".to_string(),
            "markdown".to_string(),
        );
        let new_content = black_box("updated content".to_string());

        b.iter(|| {
            doc.update_content(new_content.clone());
        })
    });
}

fn benchmark_large_markdown_conversion(c: &mut Criterion) {
    let large_content = black_box(
        "# Section\n\nParagraph with content. ".repeat(100)
    );

    c.bench_function("large_markdown_to_html", |b| {
        b.iter(|| {
            let request = ConversionRequest {
                content: large_content.clone(),
                from: Format::Markdown,
                to: Format::Html,
            };
            let _ = ConversionCore::convert(request);
        })
    });
}

fn benchmark_xml_parse(c: &mut Criterion) {
    let xml_content = black_box(
        "<root><item>value1</item><item>value2</item></root>"
    );

    c.bench_function("xml_parse", |b| {
        b.iter(|| {
            let request = ConversionRequest {
                content: xml_content.to_string(),
                from: Format::Xml,
                to: Format::Json,
            };
            let _ = ConversionCore::convert(request);
        })
    });
}

fn benchmark_html_to_markdown(c: &mut Criterion) {
    let html_content = black_box(
        "<h1>Title</h1><p>This is a paragraph with <strong>bold</strong> text.</p>"
    );

    c.bench_function("html_to_markdown", |b| {
        b.iter(|| {
            let request = ConversionRequest {
                content: html_content.to_string(),
                from: Format::Html,
                to: Format::Markdown,
            };
            let _ = ConversionCore::convert(request);
        })
    });
}

fn benchmark_json_validation(c: &mut Criterion) {
    let json_content = black_box(
        r#"{"key": "value", "nested": {"inner": "data"}}"#
    );

    c.bench_function("json_validation", |b| {
        b.iter(|| {
            let _ = ConversionCore::validate(json_content, Format::Json);
        })
    });
}

fn benchmark_document_store_bulk_operations(c: &mut Criterion) {
    c.bench_function("document_store_bulk_100", |b| {
        let store = DocumentStore::new();
        let mut counter = 0;

        b.iter(|| {
            for i in 0..100 {
                let uri = format!("file:///doc{}", counter + i);
                store.upsert(uri, format!("content{}", i), "text".to_string());
            }
            counter += 100;
        })
    });
}

fn benchmark_format_roundtrip_md_json_md(c: &mut Criterion) {
    let content = black_box("# Title\n\nContent with **bold**.".to_string());

    c.bench_function("format_roundtrip_md_json_md", |b| {
        b.iter(|| {
            // MD → JSON
            let req1 = ConversionRequest {
                content: content.clone(),
                from: Format::Markdown,
                to: Format::Json,
            };
            if let Ok(resp1) = ConversionCore::convert(req1) {
                // JSON → MD
                let req2 = ConversionRequest {
                    content: resp1.content,
                    from: Format::Json,
                    to: Format::Markdown,
                };
                let _ = ConversionCore::convert(req2);
            }
        })
    });
}

fn benchmark_same_format_noop(c: &mut Criterion) {
    let content = black_box("# Title\n\nContent.".to_string());

    c.bench_function("same_format_noop", |b| {
        b.iter(|| {
            let request = ConversionRequest {
                content: content.clone(),
                from: Format::Markdown,
                to: Format::Markdown,
            };
            let _ = ConversionCore::convert(request);
        })
    });
}

criterion_group!(
    benches,
    benchmark_toml_parse_throughput,
    benchmark_yaml_parse_throughput,
    benchmark_markdown_to_html,
    benchmark_json_to_yaml,
    benchmark_document_store_insert,
    benchmark_document_store_get,
    benchmark_document_store_contains,
    benchmark_document_create,
    benchmark_document_stats,
    benchmark_document_update,
    benchmark_large_markdown_conversion,
    benchmark_xml_parse,
    benchmark_html_to_markdown,
    benchmark_json_validation,
    benchmark_document_store_bulk_operations,
    benchmark_format_roundtrip_md_json_md,
    benchmark_same_format_noop,
);

criterion_main!(benches);
