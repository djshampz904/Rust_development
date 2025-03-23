-- Bootstrap Packer and list your plugins
require('packer').startup(function(use)
  use 'wbthomason/packer.nvim'
  use 'nvim-telescope/telescope.nvim'
  use 'nvim-lua/plenary.nvim'
  use "lunarvim/synthwave84.nvim"
  use {
    'nvim-treesitter/nvim-treesitter',
    run = ':TSUpdate'  -- Auto-install/update parsers
  }
end)

-- Setup Telescope and keybindings
require('telescope').setup {}
vim.keymap.set('n', '<leader>ff', '<cmd>Telescope find_files<cr>', {})
vim.keymap.set('n', '<leader>fg', '<cmd>Telescope live_grep<cr>', {})
vim.keymap.set('n', '<leader>fb', '<cmd>Telescope buffers<cr>', {})

-- Enable true color support and load the colorscheme
vim.opt.termguicolors = true
vim.cmd("colorscheme synthwave84")

-- Delay Treesitter configuration until VimEnter (ensuring the plugin is loaded)
vim.api.nvim_create_autocmd("VimEnter", {
  callback = function()
    require('nvim-treesitter.configs').setup {
      highlight = {
        enable = true,
        additional_vim_regex_highlighting = false,
      }
    }
  end,
})

vim.opt.background = "dark"

vim.cmd([[
  " If you want true transparency, set guibg=none for relevant groups
  hi Normal       guibg=none
  hi NonText      guibg=none
  hi SignColumn   guibg=none
  hi LineNr       guibg=none
  hi Folded       guibg=none
  hi EndOfBuffer  guibg=none
  " ... add more highlight overrides as needed
]])

require 'synthwave84'.setup({
  glow = {
    error_msg = true,
    type2 = true,
    func = true,
    keyword = true,
    operator = false,
    buffer_current_target = true,
    buffer_visible_target = true,
    buffer_inactive_target = true,
  }
})
