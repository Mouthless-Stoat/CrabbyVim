use mlua::{Table, Function};
use nvim_oxi::mlua;
use serde::Serialize;

#[derive(Serialize)]
struct LspConfig {
    name: &'static str,
    cmd: &'static [&'static str],
    filetypes: &'static [&'static str],
    root_markers: &'static [&'static str],
    settings: Table
}

pub struct Lsp {
    configs: Vec<LspConfig>
}

impl Lsp {
    /// Create a new LSP structure to configure the LSP servers
    fn new() -> Self {
        Lsp {
            configs: vec![]
        }
    }

    /// Add a new LSP server config
    fn add_config(&mut self, config: LspConfig) {
        self.configs.push(config);
    }

    /// Configure the LSP server with all config.
    fn configure(self) -> nvim_oxi::Result<()> {
        let lua = mlua::lua();
        let vim_lsp = lua.globals().get::<Table>("vim")?.get::<Table>("lsp")?;
        let lsp_config = vim_lsp.get::<Table>("config")?;

        for config in self.configs {
            let tbl = lua.create_table()?;

            tbl.set("cmd", config.cmd)?;
            tbl.set("filetypes", config.filetypes)?;
            tbl.set("root_markers", config.root_markers)?;
            tbl.set("settings", config.settings)?;

            lsp_config.set(config.name, tbl)?;
            vim_lsp.get::<Function>("enable")?.call::<()>(config.name)?;
        }

        Ok(())
    }
}

pub fn setup_lsp() -> nvim_oxi::Result<()> {
    let lua = nvim_oxi::mlua::lua();
    let mut lsp =Lsp::new();

    lsp.add_config(LspConfig {
        name: "rust",
        cmd: &[ "rust-analyzer" ],
        filetypes: &[ "rust" ],
        root_markers: &[ "Cargo.toml"],
        settings: {
            let tbl = lua.create_table()?;
            
            let imports = lua.create_table()?;
            imports.set("granularity", lua.create_table_from([("group", "module")])?)?;
            imports.set("prefix", "self")?;

            let cargo = lua.create_table()?;
            cargo.set("buildScripts", lua.create_table_from([("enable", true)])?)?;

            let check = lua.create_table()?;
            check.set("command", "clippy")?;

            let inner_tbl = lua.create_table()?;

            inner_tbl.set("imports", imports)?;
            inner_tbl.set("cargo", cargo)?;
            inner_tbl.set("check", check)?;

            tbl.set("rust-analyzer", inner_tbl)?;

            tbl
        }
    });

    lsp.configure()?;
    Ok(())
}
