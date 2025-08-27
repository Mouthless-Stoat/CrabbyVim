-- Rebuilt rust config on startup
local WIP = true

config_path = vim.fn.stdpath("config")
binary_path = vim.fs.joinpath(
    config_path,
    "/target/release",
    vim.fn.has("win32") == 1 and "config.dll" or (vim.fn.has("mac") == 1 and "config.dylib" or "config.so")
)
rust_config = vim.fs.joinpath(config_path, "/lua", vim.fn.has("win32") == 1 and "config.dll" or "config.so")

if WIP then
    vim.fs.rm(rust_config, { force = true })
end

-- compile rust config if file not found
if not vim.uv.fs_stat(rust_config) then
    local cwd = vim.fn.getcwd()
    vim.cmd.cd(config_path)
    vim.fn.mkdir(vim.fs.joinpath(config_path, "/lua"), "p")
    vim.fn.system("cargo build --release")
    vim.uv.fs_rename(binary_path, rust_config)
    vim.cmd.cd(cwd)
end
require("config")
vim.cmd.helptags(vim.fs.joinpath(config_path, "/doc"))
