macro_rules! icon_table {
    ($($name:ident = $icon:literal;)*) => {$(
        pub const $name: &'static str = $icon;
    )*};
}

icon_table! {
    TEXT = "󰉿";
    METHOD = "󰊕";
    FUNCTION = "󰊕";
    CONSTRUCTOR = "󱌣";

    FIELD = "󰜢";
    VARIABLE = "󰀫";
    PROPERTY = "󰖷";

    CLASS = "󰠱";
    INTERFACE = "";
    STRUCT = "󰅩";
    MODULE = "";

    UNIT = "";
    VALUE = "";
    ENUM = "󱡠";
    ENUMMEMBER = "󰦨";

    KEYWORD = "󰌋";
    CONSTANT = "󰏿";

    SNIPPET = "";
    COLOR = "󰏘";
    FILE = "󰈔";
    REFERENCE = "󰬲";
    FOLDER = "󰉋";
    EVENT = "";
    OPERATOR = "󰆕";
    TYPEPARAMETER = "󰬁";

    GOOD = "";
    BAD = "";
    PENDING = "";

    ERROR = "";
    WARN = "";
    HINT = "󰌵";
    INFO = "";

    DOT = "";
}
