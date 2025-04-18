vim.g.rustaceanvim = {
	-- Plugin configuration
	tools = {},
	-- LSP configuration
	server = {
		on_attach = function(client, bufnr)
			-- you can also put keymaps in here
		end,
		default_settings = {
			-- rust-analyzer language server configuration
			["rust-analyzer"] = {
				procMacro = {
					ignored = {
						leptos_macro = {
							-- optional: --
							-- "component",
							"server",
						},
					},
				},
				rustfmt = {
					overrideCommand = "leptosfmt -r",
				},
			},
		},
		-- DAP configuration
		dap = {},
	},
}
