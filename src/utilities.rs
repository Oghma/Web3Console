//! Contains utilities function or macro for the other modules.

/// Format a vector printing each value with an optional identifier
#[macro_export]
macro_rules! format_vec {
    ($vec:expr) => {
        format!("{}", $vec.join("\n"))
    };
    ($vec:expr, $indent:literal) => {{
        let indent_str = " ".repeat($indent);
        let join_str = format!("\n{}", &indent_str);
        format!("{}{}", &indent_str, $vec.join(&join_str))
    }};
}
