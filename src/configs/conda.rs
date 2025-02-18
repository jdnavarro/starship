use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CondaConfig<'a> {
    pub truncation_length: usize,
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub ignore_base: bool,
    pub disabled: bool,
}

impl<'a> Default for CondaConfig<'a> {
    fn default() -> Self {
        CondaConfig {
            truncation_length: 1,
            format: "via [$symbol$environment]($style) ",
            symbol: "🅒 ",
            style: "green bold",
            ignore_base: true,
            disabled: false,
        }
    }
}
