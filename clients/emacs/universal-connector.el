;;; universal-connector.el --- Universal Language Connector LSP client -*- lexical-binding: t -*-

;; Author: Universal Connector Contributors
;; Version: 0.1.0
;; Package-Requires: ((emacs "27.1") (lsp-mode "8.0"))
;; Keywords: languages, tools
;; URL: https://github.com/universal-connector

;;; Commentary:
;; Universal document conversion via LSP (~75 LOC)
;; All logic delegated to universal-connector-server

;;; Code:

(require 'lsp-mode)

(defgroup universal-connector nil
  "Universal Language Connector LSP client."
  :group 'lsp-mode
  :link '(url-link "https://github.com/universal-connector"))

(defcustom universal-connector-server-command "universal-connector-server"
  "Path to universal-connector-server executable."
  :type 'string
  :group 'universal-connector)

;; Register LSP client
(lsp-register-client
 (make-lsp-client
  :new-connection (lsp-stdio-connection
                   (lambda () (list universal-connector-server-command)))
  :activation-fn (lsp-activate-on "markdown" "html" "json")
  :server-id 'universal-connector
  :priority 0))

;; Conversion commands
(defun universal-connector-convert-to-html ()
  "Convert current document to HTML."
  (interactive)
  (lsp-execute-command "convert.toHtml" (vector (lsp--buffer-uri))))

(defun universal-connector-convert-to-markdown ()
  "Convert current document to Markdown."
  (interactive)
  (lsp-execute-command "convert.toMarkdown" (vector (lsp--buffer-uri))))

(defun universal-connector-convert-to-json ()
  "Convert current document to JSON."
  (interactive)
  (lsp-execute-command "convert.toJson" (vector (lsp--buffer-uri))))

;; Key bindings
(defun universal-connector-setup-keybindings ()
  "Setup key bindings for Universal Connector."
  (local-set-key (kbd "C-c u h") 'universal-connector-convert-to-html)
  (local-set-key (kbd "C-c u m") 'universal-connector-convert-to-markdown)
  (local-set-key (kbd "C-c u j") 'universal-connector-convert-to-json))

;; Auto-enable for supported modes
(add-hook 'markdown-mode-hook #'universal-connector-setup-keybindings)
(add-hook 'html-mode-hook #'universal-connector-setup-keybindings)
(add-hook 'json-mode-hook #'universal-connector-setup-keybindings)

;;;###autoload
(defun universal-connector-enable ()
  "Enable Universal Language Connector LSP client."
  (interactive)
  (add-hook 'markdown-mode-hook #'lsp)
  (add-hook 'html-mode-hook #'lsp)
  (add-hook 'json-mode-hook #'lsp))

(provide 'universal-connector)
;;; universal-connector.el ends here
