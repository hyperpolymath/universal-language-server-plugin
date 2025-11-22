// Universal Language Connector - Web UI Application

const API_BASE = 'http://localhost:8080/api';
const WS_URL = 'ws://localhost:8081';

let ws = null;
let wsConnected = false;

// DOM Elements
const inputFormat = document.getElementById('input-format');
const outputFormat = document.getElementById('output-format');
const inputContent = document.getElementById('input-content');
const outputContent = document.getElementById('output-content');
const convertBtn = document.getElementById('convert-btn');
const clearBtn = document.getElementById('clear-btn');
const conversionWarnings = document.getElementById('conversion-warnings');
const refreshDocsBtn = document.getElementById('refresh-docs-btn');
const documentsTbody = document.getElementById('documents-tbody');
const enableWebSocket = document.getElementById('enable-websocket');
const liveUpdates = document.getElementById('live-updates');
const connectionStatus = document.getElementById('connection-status');

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    loadDocuments();
    loadStats();
    setupEventListeners();
    checkServerHealth();
});

function setupEventListeners() {
    convertBtn.addEventListener('click', convertDocument);
    clearBtn.addEventListener('click', clearConverter);
    refreshDocsBtn.addEventListener('click', loadDocuments);
    enableWebSocket.addEventListener('change', toggleWebSocket);

    // Auto-convert on input change (debounced)
    let debounceTimer;
    inputContent.addEventListener('input', () => {
        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(convertDocument, 500);
    });

    inputFormat.addEventListener('change', convertDocument);
    outputFormat.addEventListener('change', convertDocument);
}

// Convert Document
async function convertDocument() {
    const content = inputContent.value;
    if (!content.trim()) {
        outputContent.value = '';
        return;
    }

    const from = inputFormat.value;
    const to = outputFormat.value;

    conversionWarnings.textContent = 'Converting...';
    convertBtn.disabled = true;

    try {
        const response = await fetch(`${API_BASE}/convert`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ content, from, to })
        });

        if (!response.ok) {
            throw new Error(`HTTP ${response.status}`);
        }

        const result = await response.json();
        outputContent.value = result.content;

        if (result.warnings && result.warnings.length > 0) {
            conversionWarnings.textContent = '⚠️ ' + result.warnings.join(', ');
        } else {
            conversionWarnings.textContent = '✅ Converted successfully';
        }
    } catch (error) {
        conversionWarnings.textContent = '❌ Conversion failed: ' + error.message;
        outputContent.value = '';
    } finally {
        convertBtn.disabled = false;
    }
}

// Clear Converter
function clearConverter() {
    inputContent.value = '';
    outputContent.value = '';
    conversionWarnings.textContent = '';
}

// Load Documents
async function loadDocuments() {
    try {
        const response = await fetch(`${API_BASE}/documents`);
        if (!response.ok) throw new Error('Failed to load documents');

        const data = await response.json();
        renderDocuments(data.documents);

        document.getElementById('total-docs').textContent = data.count;
    } catch (error) {
        console.error('Error loading documents:', error);
        documentsTbody.innerHTML = '<tr><td colspan="6" class="empty-state">Failed to load documents</td></tr>';
    }
}

// Render Documents Table
function renderDocuments(documents) {
    if (!documents || documents.length === 0) {
        documentsTbody.innerHTML = '<tr><td colspan="6" class="empty-state">No documents available</td></tr>';
        return;
    }

    documentsTbody.innerHTML = documents.map(doc => `
        <tr>
            <td>${doc.id.substring(0, 8)}...</td>
            <td>${truncate(doc.uri, 40)}</td>
            <td><span class="badge">${doc.language}</span></td>
            <td>${doc.version}</td>
            <td>${formatDate(doc.modified_at)}</td>
            <td>
                <button class="btn btn-danger" onclick="deleteDocument('${doc.id}')">Delete</button>
            </td>
        </tr>
    `).join('');
}

// Delete Document
async function deleteDocument(id) {
    if (!confirm('Are you sure you want to delete this document?')) return;

    try {
        const response = await fetch(`${API_BASE}/documents/${id}`, {
            method: 'DELETE'
        });

        if (response.ok) {
            loadDocuments();
        } else {
            alert('Failed to delete document');
        }
    } catch (error) {
        console.error('Error deleting document:', error);
        alert('Error deleting document: ' + error.message);
    }
}

// Load Server Statistics
async function loadStats() {
    try {
        const response = await fetch(`${API_BASE}/stats`);
        if (!response.ok) throw new Error('Failed to load stats');

        const stats = await response.json();
        document.getElementById('stat-version').textContent = stats.version;
        document.getElementById('stat-doc-count').textContent = stats.document_count;
        document.getElementById('stat-uptime').textContent = formatUptime(stats.uptime_seconds);
    } catch (error) {
        console.error('Error loading stats:', error);
    }
}

// Check Server Health
async function checkServerHealth() {
    try {
        const response = await fetch(`${API_BASE}/health`);
        if (response.ok) {
            const health = await response.json();
            document.getElementById('server-version').textContent = `v${health.version}`;
            updateConnectionStatus(true);
        } else {
            updateConnectionStatus(false);
        }
    } catch (error) {
        updateConnectionStatus(false);
    }
}

// WebSocket Management
function toggleWebSocket() {
    if (enableWebSocket.checked) {
        connectWebSocket();
    } else {
        disconnectWebSocket();
    }
}

function connectWebSocket() {
    if (ws) return;

    ws = new WebSocket(WS_URL);

    ws.onopen = () => {
        wsConnected = true;
        addUpdateEntry('✅ WebSocket connected');
        document.getElementById('active-connections').textContent = '1';
    };

    ws.onmessage = (event) => {
        try {
            const message = JSON.parse(event.data);
            handleWebSocketMessage(message);
        } catch (error) {
            console.error('Error parsing WebSocket message:', error);
        }
    };

    ws.onerror = (error) => {
        addUpdateEntry('❌ WebSocket error: ' + error.message);
    };

    ws.onclose = () => {
        wsConnected = false;
        ws = null;
        addUpdateEntry('🔴 WebSocket disconnected');
        document.getElementById('active-connections').textContent = '0';
    };
}

function disconnectWebSocket() {
    if (ws) {
        ws.close();
        ws = null;
        wsConnected = false;
    }
    liveUpdates.innerHTML = '<p class="empty-state">WebSocket disconnected</p>';
}

function handleWebSocketMessage(message) {
    switch (message.type) {
        case 'DocumentUpdated':
            addUpdateEntry(`📝 Document ${message.document_id.substring(0, 8)}... updated`);
            loadDocuments(); // Refresh document list
            break;
        case 'Pong':
            addUpdateEntry('🏓 Pong received');
            break;
        case 'Error':
            addUpdateEntry(`❌ Error: ${message.message}`);
            break;
        default:
            addUpdateEntry(`📨 ${message.type}`);
    }
}

function addUpdateEntry(text) {
    const entry = document.createElement('div');
    entry.className = 'update-entry';
    entry.innerHTML = `
        <div>${text}</div>
        <div class="update-timestamp">${new Date().toLocaleTimeString()}</div>
    `;

    if (liveUpdates.querySelector('.empty-state')) {
        liveUpdates.innerHTML = '';
    }

    liveUpdates.insertBefore(entry, liveUpdates.firstChild);

    // Keep only last 50 entries
    while (liveUpdates.children.length > 50) {
        liveUpdates.removeChild(liveUpdates.lastChild);
    }
}

// Utility Functions
function updateConnectionStatus(connected) {
    if (connected) {
        connectionStatus.textContent = 'Connected';
        connectionStatus.className = 'status-badge connected';
    } else {
        connectionStatus.textContent = 'Disconnected';
        connectionStatus.className = 'status-badge disconnected';
    }
}

function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleString();
}

function formatUptime(seconds) {
    if (seconds < 60) return `${seconds}s`;
    if (seconds < 3600) return `${Math.floor(seconds / 60)}m`;
    return `${Math.floor(seconds / 3600)}h`;
}

function truncate(str, maxLength) {
    if (str.length <= maxLength) return str;
    return str.substring(0, maxLength - 3) + '...';
}

// Auto-refresh stats and documents
setInterval(() => {
    loadStats();
    if (!enableWebSocket.checked) {
        loadDocuments();
    }
}, 30000); // Every 30 seconds

// Keep-alive ping for WebSocket
setInterval(() => {
    if (ws && wsConnected) {
        ws.send(JSON.stringify({ type: 'Ping' }));
    }
}, 30000);
