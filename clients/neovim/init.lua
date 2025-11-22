-- Universal Language Connector - Neovim Client
-- ~65 LOC - All logic delegated to LSP server

local M = {}

-- Start the LSP client
function M.setup(opts)
  opts = opts or {}

  -- Server command configuration
  local server_cmd = opts.cmd or { 'universal-connector-server' }

  -- LSP client configuration
  local client_id = vim.lsp.start({
    name = 'universal-connector',
    cmd = server_cmd,
    filetypes = { 'markdown', 'html', 'json' },
    root_dir = vim.fn.getcwd(),
    settings = {},
    on_attach = function(client, bufnr)
      -- Enable completion
      vim.bo[bufnr].omnifunc = 'v:lua.vim.lsp.omnifunc'

      -- Key mappings
      local opts_map = { noremap = true, silent = true, buffer = bufnr }

      vim.keymap.set('n', 'K', vim.lsp.buf.hover, opts_map)
      vim.keymap.set('n', 'gd', vim.lsp.buf.definition, opts_map)
      vim.keymap.set('n', '<leader>ca', vim.lsp.buf.code_action, opts_map)

      -- Conversion commands
      vim.keymap.set('n', '<leader>ch', function()
        vim.lsp.buf.execute_command({
          command = 'convert.toHtml',
          arguments = { vim.uri_from_bufnr(bufnr) }
        })
      end, opts_map)

      vim.keymap.set('n', '<leader>cm', function()
        vim.lsp.buf.execute_command({
          command = 'convert.toMarkdown',
          arguments = { vim.uri_from_bufnr(bufnr) }
        })
      end, opts_map)

      vim.keymap.set('n', '<leader>cj', function()
        vim.lsp.buf.execute_command({
          command = 'convert.toJson',
          arguments = { vim.uri_from_bufnr(bufnr) }
        })
      end, opts_map)
    end,
  })

  return client_id
end

-- Auto-start for supported filetypes
vim.api.nvim_create_autocmd('FileType', {
  pattern = { 'markdown', 'html', 'json' },
  callback = function()
    M.setup()
  end,
})

return M
