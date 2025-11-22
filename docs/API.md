# Universal Language Connector - API Documentation

## Overview

The Universal Language Connector provides three main APIs:

1. **LSP (Language Server Protocol)** - For editor integration via stdio
2. **HTTP REST API** - For web and programmatic access
3. **WebSocket API** - For real-time document updates

## LSP API

### Server Capabilities

The server implements LSP 3.17 with the following capabilities:

```typescript
{
  textDocumentSync: "incremental",
  completionProvider: {
    triggerCharacters: ["#", "@", "["]
  },
  hoverProvider: true,
  definitionProvider: true,
  diagnosticProvider: true,
  executeCommandProvider: {
    commands: [
      "convert.toMarkdown",
      "convert.toHtml",
      "convert.toJson"
    ]
  }
}
```

### LSP Methods

#### textDocument/didOpen

Notifies the server that a document has been opened.

**Parameters:**
```json
{
  "textDocument": {
    "uri": "file:///path/to/document.md",
    "languageId": "markdown",
    "version": 1,
    "text": "# Document content"
  }
}
```

#### textDocument/didChange

Notifies the server of document changes.

**Parameters:**
```json
{
  "textDocument": {
    "uri": "file:///path/to/document.md",
    "version": 2
  },
  "contentChanges": [
    {
      "text": "# Updated content"
    }
  ]
}
```

#### textDocument/completion

Requests completion items at a given position.

**Parameters:**
```json
{
  "textDocument": {
    "uri": "file:///path/to/document.md"
  },
  "position": {
    "line": 0,
    "character": 5
  }
}
```

**Response:**
```json
[
  {
    "label": "Convert to HTML",
    "kind": 1,
    "detail": "Convert current document to HTML",
    "command": {
      "title": "Convert to HTML",
      "command": "convert.toHtml",
      "arguments": ["file:///path/to/document.md"]
    }
  }
]
```

#### textDocument/hover

Provides hover information (document statistics).

**Parameters:**
```json
{
  "textDocument": {
    "uri": "file:///path/to/document.md"
  },
  "position": {
    "line": 0,
    "character": 5
  }
}
```

**Response:**
```json
{
  "contents": {
    "kind": "markdown",
    "value": "**Document Statistics**\n\n- Lines: 10\n- Words: 50\n- Characters: 300\n- Version: 2\n- Format: markdown"
  }
}
```

#### workspace/executeCommand

Executes a conversion command.

**Parameters:**
```json
{
  "command": "convert.toHtml",
  "arguments": ["file:///path/to/document.md"]
}
```

**Response:**
```json
{
  "content": "<h1>Converted HTML</h1>",
  "format": "html",
  "warnings": []
}
```

## HTTP REST API

Base URL: `http://localhost:8080/api`

### Endpoints

#### POST /api/convert

Convert document between formats.

**Request:**
```json
{
  "content": "# Hello World",
  "from": "markdown",
  "to": "html"
}
```

**Response:**
```json
{
  "content": "<h1>Hello World</h1>",
  "from": "markdown",
  "to": "html",
  "warnings": []
}
```

**Status Codes:**
- `200 OK` - Conversion successful
- `400 Bad Request` - Invalid format or content
- `500 Internal Server Error` - Conversion failed

#### GET /api/documents

List all documents in the server.

**Response:**
```json
{
  "documents": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "uri": "file:///path/to/document.md",
      "content": "# Document content",
      "language": "markdown",
      "version": 1,
      "created_at": "2025-11-22T12:00:00Z",
      "modified_at": "2025-11-22T12:05:00Z"
    }
  ],
  "count": 1
}
```

#### GET /api/documents/:id

Get a specific document by ID.

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "uri": "file:///path/to/document.md",
  "content": "# Document content",
  "language": "markdown",
  "version": 1,
  "created_at": "2025-11-22T12:00:00Z",
  "modified_at": "2025-11-22T12:05:00Z"
}
```

**Status Codes:**
- `200 OK` - Document found
- `404 Not Found` - Document not found

#### DELETE /api/documents/:id

Delete a document by ID.

**Status Codes:**
- `204 No Content` - Document deleted
- `404 Not Found` - Document not found

#### POST /api/validate

Validate document format.

**Request:**
```json
{
  "content": "# Valid Markdown",
  "format": "markdown"
}
```

**Response:**
```json
{
  "valid": true,
  "diagnostics": []
}
```

For invalid content:
```json
{
  "valid": false,
  "diagnostics": [
    "Invalid JSON: unexpected token at line 1"
  ]
}
```

#### GET /api/stats

Get server statistics.

**Response:**
```json
{
  "document_count": 5,
  "uptime_seconds": 3600,
  "version": "0.1.0"
}
```

#### GET /api/health

Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0"
}
```

### Error Responses

All errors return a standard error object:

```json
{
  "error": "Error message description"
}
```

## WebSocket API

WebSocket URL: `ws://localhost:8081`

### Message Types

#### Subscribe

Subscribe to document updates.

**Client → Server:**
```json
{
  "type": "Subscribe",
  "document_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

#### Unsubscribe

Unsubscribe from document updates.

**Client → Server:**
```json
{
  "type": "Unsubscribe",
  "document_id": "550e8400-e29b-41d4-a716-446655440000"
}
```

#### DocumentUpdated

Document update notification.

**Server → Client:**
```json
{
  "type": "DocumentUpdated",
  "document_id": "550e8400-e29b-41d4-a716-446655440000",
  "content": "# Updated content",
  "timestamp": "2025-11-22T12:10:00Z"
}
```

#### Ping/Pong

Keep-alive messages.

**Client → Server:**
```json
{
  "type": "Ping"
}
```

**Server → Client:**
```json
{
  "type": "Pong"
}
```

#### Error

Error notification.

**Server → Client:**
```json
{
  "type": "Error",
  "message": "Error description"
}
```

## Supported Formats

### Format Identifiers

- `markdown` or `md` - Markdown
- `html` or `htm` - HTML
- `json` - JSON

### Conversion Matrix

| From     | To       | Status | Notes                          |
|----------|----------|--------|--------------------------------|
| Markdown | HTML     | ✅     | Full support via pulldown-cmark |
| Markdown | JSON     | ✅     | Structured representation       |
| HTML     | Markdown | ⚠️     | Lossy conversion               |
| HTML     | JSON     | ✅     | DOM structure extraction        |
| JSON     | Markdown | ✅     | Key-value representation        |
| JSON     | HTML     | ✅     | Via Markdown intermediary       |

## Authentication & Security

Currently, the server does not implement authentication. For production use:

- Deploy behind a reverse proxy (nginx, Apache)
- Use TLS/SSL for encrypted connections
- Implement authentication at the proxy level
- Restrict access via firewall rules

## Rate Limiting

No rate limiting is currently implemented. Consider adding:

- Request rate limiting at proxy level
- Connection limits for WebSocket
- Resource usage monitoring

## CORS

CORS is enabled for all origins in development. For production:

- Configure specific allowed origins
- Restrict allowed methods and headers
- Implement proper preflight handling

## Examples

### cURL Examples

**Convert Markdown to HTML:**
```bash
curl -X POST http://localhost:8080/api/convert \
  -H "Content-Type: application/json" \
  -d '{
    "content": "# Hello World",
    "from": "markdown",
    "to": "html"
  }'
```

**List documents:**
```bash
curl http://localhost:8080/api/documents
```

**Health check:**
```bash
curl http://localhost:8080/api/health
```

### JavaScript/Fetch Examples

**Convert document:**
```javascript
const response = await fetch('http://localhost:8080/api/convert', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    content: '# Hello World',
    from: 'markdown',
    to: 'html'
  })
});

const result = await response.json();
console.log(result.content);
```

**WebSocket connection:**
```javascript
const ws = new WebSocket('ws://localhost:8081');

ws.onopen = () => {
  ws.send(JSON.stringify({
    type: 'Subscribe',
    document_id: 'doc-123'
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

## Performance Considerations

- **Response Time Target:** <100ms for all operations
- **Memory Usage:** <50MB steady state
- **Startup Time:** <500ms
- **Concurrent Connections:** Supports 100+ simultaneous clients

## Versioning

API version is included in all responses via the `version` field.

Current version: `0.1.0`

Future versions will maintain backward compatibility within major versions following Semantic Versioning (SemVer).
