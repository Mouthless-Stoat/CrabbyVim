#![allow(dead_code, missing_docs)]

macro_rules! icon_table {
    ($($name:ident = $icon:literal;)*) => {$(
        pub const $name: &'static str = $icon;
    )*};
}

icon_table! {
    TEXT = "󰉿";
    METHOD = "󰊕";
    FUNCTION = "󰊕";
    CONSTRUCTOR = "";

    FIELD = "󰜢";
    VARIABLE = "󰀫";
    PROPERTY = "󰖷";

    CLASS = "󰠱";
    INTERFACE = "";
    STRUCT = "󰆼";
    MODULE = "";

    UNIT = "";
    VALUE = "";
    ENUM = "󱡠";
    ENUM_MEMBER = "󰦨";

    KEYWORD = "󰌋";
    CONSTANT = "󰏿";

    SNIPPET = "";
    COLOR = "󰏘";
    FILE = "󰈔";
    REFERENCE = "󰬲";
    FOLDER = "󰉋";
    EVENT = "";
    OPERATOR = "󰆕";
    TYPE_PARAMETER = "󰬁";

    GOOD = "";
    BAD = "";
    PENDING = "";

    INSTALLED = "󰋘";
    UNINSTALLED = "󰋙";

    ERROR = "";
    WARN = "";
    HINT = "󰌵";
    INFO = "";

    ADDED = "▐";
    CHANGED = "▐";
    DELETED = "";
    TOP_DELETED = "";
    CHANGE_DELETED = "▐";
    UNTRACKED = "▐";

    MAGNIFYING_GLASS = "";

    CODE_CWD = "";
    DESKTOP_CWD = "";
    HOME_CWD = "";
    NVIM_CWD = "";

    LSP = "";
    FORMATTER = "";
    GIT_BRANCH = "";
    GIT_DIFF = "";
}
