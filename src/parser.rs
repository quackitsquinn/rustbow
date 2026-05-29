//! Utils for parsing CLI arguments.

use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::config::{ColorGeneratorConfig, ConfigSet};

/// The default color generator type to use if not specified in the config string.
pub const DEFAULT_COLOR_GEN_TYPE: &str = "hue_shift";

/// Parses a color generator config string into a `ColorGeneratorConfig`.
///
/// The format of this string is the following, where `generator_type` is optional and defaults to `hue_shift`:
///
/// `generator_type:attrib1:value1,attrib2:value2,...`
pub fn parse_color_str(cstr: &str) -> anyhow::Result<ColorGeneratorConfig> {
    let (gen_type, attrib_str) = leading_component(cstr).unwrap_or((DEFAULT_COLOR_GEN_TYPE, cstr));
    ColorGeneratorConfig::parse(gen_type, attrib_str)
}

fn leading_component(s: &str) -> Option<(&str, &str)> {
    let first_section = s.split_once(',').map_or(s, |(head, _)| head);

    match first_section.matches(':').count() {
        // bare generator: "hue_shift"
        0 => Some((first_section, "")),

        // generator with first attr: "hue_shift:rate:0.1"
        2.. => {
            let (gen, rest) = s.split_once(':')?;
            Some((gen, rest))
        }

        // probably "rate:0.1", so no generator
        1 => None,
    }
}

fn parse_attribute_map(map: &str) -> anyhow::Result<HashMap<Arc<str>, Arc<str>>> {
    let mut attributes = HashMap::new();

    for part in map.split(',').map(str::trim) {
        let mut kv = part.splitn(2, ':').map(str::trim);
        let key = kv
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid attribute map: {map}"))?;
        let value = kv
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid color config string: {map}"))?;

        attributes.insert(key.into(), value.into());
    }

    Ok(attributes)
}

#[cfg(test)]
mod tests {
    use color::OpaqueColor;

    use super::*;

    #[test]
    fn test_leading_component() {
        fn test(s: &str, expected: Option<(&str, &str)>) {
            assert_eq!(leading_component(s), expected);
        }

        test("hue_shift:rate:0.1", Some(("hue_shift", "rate:0.1")));
        test("hue_shift", Some(("hue_shift", "")));
        test("rate:0.1", None);
        test(
            "hue_shift:rate:0.1,something_else",
            Some(("hue_shift", "rate:0.1,something_else")),
        );
        test("hue_shift:", Some(("hue_shift", ""))); //
    }

    #[test]
    fn test_parse_color_str() {
        let cfg = parse_color_str("hue_shift:rate:0.1").unwrap();
        match cfg {
            ColorGeneratorConfig::HueShift(hue_cfg) => {
                assert_eq!(hue_cfg.rate, 0.1);
                assert_eq!(hue_cfg.initial_color, OpaqueColor::new([0., 1., 0.5]));
            }

            _ => panic!("Expected HueShift config"),
        }
    }
}
