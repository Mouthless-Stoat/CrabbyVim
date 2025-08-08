macro_rules! icon_table {
    ($($name:ident = $icon:literal;)*) => {$(
        #[allow(unused)]
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

    ERROR = "";
    WARN = "";
    HINT = "󰌵";
    INFO = "";

    ADDED = "│";
    CHANGED = "│";
    DELETED = "";
    TOP_DELETED = "";
    CHANGE_DELETED = "│";
    UNTRACKED = "┊";

    MAGNIFYING_GLASS = "";
}
