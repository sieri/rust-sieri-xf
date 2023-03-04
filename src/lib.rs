mod events;
mod xf;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, parse::Parser, parse_macro_input, ItemStruct};

/// Add a field to a structure
///
///
/// # Examples
///
/// ```
///     use sieri_xf::add_field;
///
///     #[add_field]
///     struct Foo {}
///
///     let bar = Foo { a: "lorem ipsum".to_string()};
/// ```
#[proc_macro_attribute]
pub fn add_field(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { pub a: String })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[cfg(test)]
mod tests {
    use std::sync::Once;

    static INIT: Once = Once::new();

    pub fn setup_logger() -> Result<(), fern::InitError> {
        use fern::colors::{Color, ColoredLevelConfig};
        let level = if cfg!(debug_assertions) {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        };
        let colors = ColoredLevelConfig::new()
            .trace(Color::Cyan)
            .debug(Color::Magenta)
            .info(Color::Green)
            .warn(Color::BrightYellow)
            .error(Color::Red);

        fern::Dispatch::new()
            .level(level)
            .chain(
                fern::Dispatch::new()
                    .format(move |out, message, record| {
                        let module_path: Vec<&str> =
                            record.module_path().unwrap_or("").split("::").collect();
                        let len = module_path.len();
                        let module_path = if len > 2 && module_path[0] == "factory_management_utils"
                        {
                            format!("..{}::{}", module_path[len - 2], module_path[len - 1])
                        } else {
                            record.module_path().unwrap_or("").to_string()
                        };
                        out.finish(format_args!(
                            "[{}][{}] {}",
                            module_path,
                            colors.color(record.level()),
                            message
                        ))
                    })
                    .chain(std::io::stdout()),
            )
            .apply()?;

        Ok(())
    }

    pub fn setup() {
        INIT.call_once(|| {
            setup_logger().expect("Logger couldn't be initialized");
        });
    }
}
