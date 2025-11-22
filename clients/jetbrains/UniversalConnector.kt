// Universal Language Connector - JetBrains Plugin
// ~55 LOC - All logic delegated to LSP server

package com.universalconnector

import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.LspServerDescriptor
import com.intellij.platform.lsp.api.LspServerManager
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor

class UniversalConnectorLspServerDescriptor(project: Project) :
    ProjectWideLspServerDescriptor(project, "Universal Connector") {

    override fun isSupportedFile(file: VirtualFile): Boolean {
        val extension = file.extension?.lowercase()
        return extension in listOf("md", "markdown", "html", "htm", "json")
    }

    override fun createCommandLine(): List<String> {
        return listOf("universal-connector-server")
    }
}

class UniversalConnectorLspServerSupportProvider : LspServerManager.LspServerSupportProvider {
    override fun fileOpened(
        project: Project,
        file: VirtualFile,
        serverStarter: LspServerManager.LspServerStarter
    ) {
        val extension = file.extension?.lowercase()
        if (extension in listOf("md", "markdown", "html", "htm", "json")) {
            serverStarter.ensureServerStarted(UniversalConnectorLspServerDescriptor(project))
        }
    }
}

// Action for converting to HTML
class ConvertToHtmlAction : AnAction("Convert to HTML") {
    override fun actionPerformed(e: AnActionEvent) {
        val project = e.project ?: return
        val editor = e.getData(CommonDataKeys.EDITOR) ?: return
        val file = e.getData(CommonDataKeys.VIRTUAL_FILE) ?: return

        LspServerManager.getInstance(project).getServersForProvider(
            UniversalConnectorLspServerSupportProvider::class.java
        ).forEach { server ->
            server.sendRequest("workspace/executeCommand") {
                put("command", "convert.toHtml")
                putArray("arguments").add(file.url)
            }
        }
    }
}
