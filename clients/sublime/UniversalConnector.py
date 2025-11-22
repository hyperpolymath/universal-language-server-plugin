# Universal Language Connector - Sublime Text Plugin
# ~60 LOC - All logic delegated to LSP server

import sublime
import sublime_plugin
from LSP.plugin import Session
from LSP.plugin.core.protocol import Request


class UniversalConnectorPlugin(sublime_plugin.Plugin):
    """Universal Language Connector LSP client for Sublime Text."""

    @classmethod
    def name(cls) -> str:
        return "universal-connector"

    @classmethod
    def configuration(cls):
        """LSP server configuration."""
        return {
            "command": ["universal-connector-server"],
            "selector": "source.markdown | text.html | source.json",
            "enabled": True,
            "languages": [
                {
                    "languageId": "markdown",
                    "scopes": ["source.markdown"],
                    "syntaxes": ["Packages/Markdown/Markdown.sublime-syntax"]
                },
                {
                    "languageId": "html",
                    "scopes": ["text.html"],
                    "syntaxes": ["Packages/HTML/HTML.sublime-syntax"]
                },
                {
                    "languageId": "json",
                    "scopes": ["source.json"],
                    "syntaxes": ["Packages/JSON/JSON.sublime-syntax"]
                }
            ]
        }


class ConvertToHtmlCommand(sublime_plugin.TextCommand):
    """Convert document to HTML."""

    def run(self, edit):
        session = Session.for_view(self.view, "universal-connector")
        if session:
            uri = self.view.file_name() or ""
            session.send_request(
                Request("workspace/executeCommand", {
                    "command": "convert.toHtml",
                    "arguments": [uri]
                }),
                lambda result: None
            )


class ConvertToMarkdownCommand(sublime_plugin.TextCommand):
    """Convert document to Markdown."""

    def run(self, edit):
        session = Session.for_view(self.view, "universal-connector")
        if session:
            uri = self.view.file_name() or ""
            session.send_request(
                Request("workspace/executeCommand", {
                    "command": "convert.toMarkdown",
                    "arguments": [uri]
                }),
                lambda result: None
            )
