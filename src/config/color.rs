/// A generator for a color generator, using a given config.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ColorGeneratorConfig {
    /// A color generator that shifts the hue of the output color by a defined rate each sample.
    // TODO: allow using per frame instead of per sample
    HueShift(HueShiftConfig),
}

impl ColorGeneratorConfig {
    /// Builds a color generator from the config.
    pub fn build_generator(&self) -> Box<dyn ColorGenerator> {
        match self {
            Self::HueShift(cfg) => Box::new(HueShiftGenerator::new(*cfg)),
        }
    }

    /// Parses a color generator config from a generator type and a string of attributes. The generator type is optional and defaults to "hue_shift"
    pub fn parse(generator_type: &str, attributes: &str) -> anyhow::Result<Self> {
        let attributes = ConfigSet::from_str(attributes)
            .context("Failed to parse color generator attributes")?;
        match generator_type {
            "hue_shift" => Ok(Self::HueShift(HueShiftConfig::from_config(
                &mut attributes.clone(),
            )?)),
            _ => anyhow::bail!("Unknown color generator type: {}", generator_type),
        }
    }
}

impl Default for ColorGeneratorConfig {
    fn default() -> Self {
        Self::HueShift(HueShiftConfig::default())
    }
}
