-- Rebuilt rust config on startup
local WIP = true

local config_path = vim.fn.stdpath("config")
local rust_config = vim.fs.joinpath(config_path, "/lua", vim.fn.has("win32") == 1 and "config.dll" or "libconfig.so")

if WIP then
    vim.fs.rm(rust_config, { force = true })
end

-- compile rust config if file not found
if not vim.uv.fs_stat(rust_config) then
    local cwd = vim.fn.getcwd()
    vim.cmd.cd(config_path)
    vim.fn.system("just build")
    vim.cmd.cd(cwd)
end
require(vim.fn.has("win32") and "config" or "libconfig")
