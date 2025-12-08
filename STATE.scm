;;; STATE.scm - Universal Language Connector
;;; Checkpoint file for AI conversation continuity
;;; Format: Guile Scheme S-expressions
;;; Reference: https://github.com/hyperpolymath/state.scm

;;;============================================================================
;;; METADATA
;;;============================================================================

(define-module (state)
  #:export (project-state))

(define state-metadata
  '((format-version . "2.0")
    (schema-version . "1.0")
    (created . "2025-12-08")
    (last-updated . "2025-12-08")
    (generator . "claude-code")))

;;;============================================================================
;;; PROJECT CONTEXT
;;;============================================================================

(define project-context
  '((name . "Universal Language Connector")
    (repository . "universal-language-server-plugin")
    (description . "LSP-based universal plugin architecture - one server powers plugins across all major editors")
    (license . "MIT")
    (language . "Rust")
    (category . "developer-tools")))

;;;============================================================================
;;; CURRENT POSITION
;;;============================================================================

(define current-position
  '((phase . "post-implementation")
    (completion-percentage . 85)
    (status . "in-progress")

    (implemented
     ((server
       (lsp-handler . "complete")
       (http-api . "complete")
       (websocket . "complete")
       (conversion-core . "complete")
       (document-store . "complete")
       (monitoring . "complete")
       (auth . "complete"))

      (formats
       (markdown-html . "complete")
       (html-markdown . "complete")
       (html-json . "complete")
       (json-markdown . "complete")
       (yaml-json . "complete")
       (xml-json . "complete")
       (toml-json . "complete"))

      (clients
       (vscode . "complete")
       (neovim . "complete")
       (emacs . "complete")
       (jetbrains . "complete")
       (sublime . "complete")
       (zed . "complete")
       (helix . "complete"))

      (infrastructure
       (dockerfile . "complete")
       (docker-compose . "complete")
       (podman-compose . "complete")
       (makefile . "complete")
       (github-actions . "complete")
       (gitlab-ci . "complete"))))

    (architecture-validated
     ((lsp-compliance . "basic-tests-passing")
      (client-loc-constraint . "all-under-100-loc")
      (performance-targets . "untested")
      (real-editor-testing . "not-done")))))

;;;============================================================================
;;; ROUTE TO MVP V1
;;;============================================================================

(define mvp-v1-route
  '((target-milestone . "production-ready-v1.0")

    (critical-path
     ((step-1
       (task . "End-to-end testing with real editors")
       (description . "Test LSP integration in actual VS Code, Neovim, Emacs instances")
       (blockers . none)
       (effort . "medium")
       (priority . "P0"))

      (step-2
       (task . "Performance validation")
       (description . "Run benchmarks to verify <100ms response, <50MB memory, <500ms startup")
       (blockers . none)
       (effort . "small")
       (priority . "P0"))

      (step-3
       (task . "LSP 3.17 official compliance testing")
       (description . "Run official LSP test suite, fix any compliance gaps")
       (blockers . none)
       (effort . "medium")
       (priority . "P1"))

      (step-4
       (task . "Production hardening")
       (description . "Error handling, graceful shutdown, rate limiting, input sanitization")
       (blockers . none)
       (effort . "medium")
       (priority . "P1"))

      (step-5
       (task . "Documentation review")
       (description . "Verify all docs match actual implementation, update API.md")
       (blockers . none)
       (effort . "small")
       (priority . "P2"))))

    (definition-of-done
     ("All 7 editor clients work with real server"
      "Performance benchmarks meet targets"
      "LSP compliance tests pass"
      "No critical security issues"
      "Documentation accurate and complete"))))

;;;============================================================================
;;; KNOWN ISSUES
;;;============================================================================

(define known-issues
  '((technical
     ((issue-1
       (severity . "low")
       (component . "server/src/http.rs:181")
       (description . "Uptime tracking not implemented - returns 0")
       (fix . "Add startup timestamp to ServerState, calculate delta"))

      (issue-2
       (severity . "medium")
       (component . "server/src/websocket.rs:72-76")
       (description . "WebSocket subscription tracking incomplete - all clients receive all updates")
       (fix . "Implement per-client subscription set, filter broadcasts"))

      (issue-3
       (severity . "low")
       (component . "server/src/core.rs:222-280")
       (description . "HTML to Markdown conversion is lossy - may lose formatting")
       (fix . "Enhance scraper-based extraction, preserve more structure"))

      (issue-4
       (severity . "info")
       (component . "README.md:330")
       (description . "README says YAML/XML/TOML not supported, but Platinum RSR added them")
       (fix . "Update README roadmap to mark these complete"))))

    (documentation
     ((issue-1
       (description . "Mismatch between README roadmap and actual implementation")
       (details . "Extended formats (YAML, XML, TOML) are implemented but not documented"))

      (issue-2
       (description . "Client READMEs referenced but don't exist")
       (details . "README links to clients/vscode/README.md etc. which are missing"))))))

;;;============================================================================
;;; OPEN QUESTIONS
;;;============================================================================

(define open-questions
  '((architectural
     ((q1
       (question . "Should WebSocket use per-document subscriptions or broadcast all?")
       (context . "Current impl broadcasts everything, may not scale")
       (proposed-answer . "Implement subscription filtering for production"))

      (q2
       (question . "Is Platinum RSR compliance a formal requirement or internal label?")
       (context . "YAML/XML/TOML support marked as 'Platinum RSR' in comments")
       (proposed-answer . "Clarify if this is a compliance standard or internal milestone"))))

    (implementation
     ((q1
       (question . "How should conversion errors be surfaced to LSP clients?")
       (context . "Currently uses show_message, could use diagnostics instead")
       (proposed-answer . "Use diagnostics for persistent errors, messages for transient"))

      (q2
       (question . "Should the server serve web UI directly or via separate static server?")
       (context . "web/ folder exists but no route serves it")
       (proposed-answer . "Add static file serving to axum router for self-contained deployment"))))

    (testing
     ((q1
       (question . "What is the official LSP 3.17 compliance test suite?")
       (context . "CLAUDE.md mentions testing with 'official suite' but doesn't specify")
       (proposed-answer . "Microsoft's vscode-languageserver-node has test utilities"))))))

;;;============================================================================
;;; LONG-TERM ROADMAP
;;;============================================================================

(define long-term-roadmap
  '((v1-0
     (target . "MVP Production Release")
     (status . "in-progress")
     (items
      ("End-to-end editor testing"
       "Performance benchmarks"
       "LSP compliance validation"
       "Documentation updates")))

    (v1-1
     (target . "Enhanced Conversion")
     (status . "planned")
     (items
      ("Rich AST representation for all formats"
       "Bidirectional conversion with minimal loss"
       "Custom format plugin system"
       "Conversion pipeline visualization")))

    (v2-0
     (target . "WASM Module")
     (status . "future")
     (items
      ("WASM core module for sandboxed execution"
       "Browser-native conversion without server"
       "Edge deployment capability"
       "Portable conversion runtime")))

    (v2-1
     (target . "Advanced LSP Features")
     (status . "future")
     (items
      ("LSP 3.18 features when stabilized"
       "Semantic tokens for format-specific highlighting"
       "Code lens for inline conversion"
       "Rename support across linked documents")))

    (v3-0
     (target . "Enterprise Features")
     (status . "future")
     (items
      ("Multi-user collaboration"
       "Access control and audit logging"
       "Custom enterprise format support"
       "SLA-grade reliability")))))

;;;============================================================================
;;; CRITICAL NEXT ACTIONS
;;;============================================================================

(define critical-next-actions
  '((action-1
     (priority . "P0")
     (task . "Test VS Code extension with real server")
     (description . "Install extension in VS Code, start server, verify LSP methods work")
     (acceptance . "Can open markdown, see hover stats, execute convert commands"))

    (action-2
     (priority . "P0")
     (task . "Run cargo bench or criterion benchmarks")
     (description . "Verify performance meets <100ms response, <50MB memory targets")
     (acceptance . "Benchmark results documented, all targets met"))

    (action-3
     (priority . "P1")
     (task . "Fix WebSocket subscription filtering")
     (description . "Implement per-client document subscription tracking")
     (acceptance . "Clients only receive updates for subscribed documents"))

    (action-4
     (priority . "P1")
     (task . "Add static file serving for web UI")
     (description . "Serve web/ folder from HTTP server root")
     (acceptance . "http://localhost:8080/ serves web UI"))

    (action-5
     (priority . "P2")
     (task . "Update README roadmap")
     (description . "Mark YAML/XML/TOML as complete, update feature matrix")
     (acceptance . "README accurately reflects implementation status"))))

;;;============================================================================
;;; SESSION FILES
;;;============================================================================

(define session-files
  '((created . ())
    (modified . ())
    (analyzed
     ("server/src/core.rs"
      "server/src/lsp.rs"
      "server/src/http.rs"
      "server/src/websocket.rs"
      "server/Cargo.toml"
      "server/tests/lsp_compliance.rs"
      "clients/vscode/extension.ts"
      "web/index.html"
      "CLAUDE.md"
      "README.md"))))

;;;============================================================================
;;; HELPER FUNCTIONS
;;;============================================================================

(define (get-completion-percentage)
  "Return current project completion as percentage"
  (assoc-ref current-position 'completion-percentage))

(define (get-critical-blockers)
  "Return list of P0 blocking issues"
  (filter (lambda (action)
            (equal? (assoc-ref action 'priority) "P0"))
          (cdr critical-next-actions)))

(define (get-mvp-status)
  "Return MVP readiness assessment"
  '((core-implementation . "complete")
    (editor-clients . "complete")
    (testing . "incomplete")
    (documentation . "needs-update")
    (production-hardening . "incomplete")))

;;;============================================================================
;;; STATE EXPORT
;;;============================================================================

(define project-state
  `((metadata . ,state-metadata)
    (context . ,project-context)
    (position . ,current-position)
    (mvp-route . ,mvp-v1-route)
    (issues . ,known-issues)
    (questions . ,open-questions)
    (roadmap . ,long-term-roadmap)
    (next-actions . ,critical-next-actions)
    (session . ,session-files)))

;;; End of STATE.scm
