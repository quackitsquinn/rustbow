use std::{collections::HashMap, str::FromStr, sync::Arc};

/// A set of config key-value pairs, used for parsing color generator configs.
///
/// This is in the format of `key1:value1,key2:value2,...`. The keys and values are both strings, and the keys are case-sensitive.
/// There is no escaping mechanism, so probably only numeral values should be used
#[derive(Debug, Clone)]
pub struct ConfigSet(HashMap<Arc<str>, Arc<str>>);

impl ConfigSet {
    /// Creates an empty config set.
    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    /// Attempts to parse a value of type `P` from the given key, removing it from the config set if it exists.
    pub fn take<P>(&mut self, key: &str) -> Result<Option<P>, P::Err>
    where
        P: FromStr,
    {
        match self.0.remove(key) {
            Some(value) => Ok(Some(value.parse::<P>()?)),
            None => Ok(None),
        }
    }

    /// Attempts to parse a value of type `P` from any of the given keys, removing it from the config set if it exists.
    ///  If multiple keys are found, the first one is used and a warning is printed.
    pub fn take_any<P>(&mut self, keys: &[&str]) -> Result<Option<P>, P::Err>
    where
        P: FromStr,
    {
        let mut result = None;

        for &key in keys {
            if let Some(value) = self.0.remove(key) {
                if result.is_some() {
                    println!(
                        "Warning: Multiple keys found for the same config value: `{}` and `{}`. Using value from `{}`.",
                        key, keys.iter().find(|&&k| k != key && self.0.contains_key(k)).unwrap_or(&key), key
                    );
                }
                result = Some(value.parse::<P>().map(Some));
            }
        }

        if let Some(v) = result {
            return v;
        }
        Ok(None)
    }

    /// Attempts to parse a value of type `P` from the given key using the provided parser function, removing it from the config set if it exists.
    pub fn take_with_parser<P, E>(
        &mut self,
        key: &str,
        parser: impl FnOnce(&str) -> Result<P, E>,
    ) -> Result<Option<P>, E> {
        match self.0.remove(key) {
            Some(value) => Ok(Some(parser(&value)?)),
            None => Ok(None),
        }
    }

    /// Finishes processing the config set, returning an error if there are any unknown keys left.
    pub fn finish(self) -> anyhow::Result<()> {
        if self.0.is_empty() {
            Ok(())
        } else {
            let unknown_keys = self
                .0
                .keys()
                .map(|k| k.as_ref())
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!("Unknown config keys: {unknown_keys}")
        }
    }
}

impl FromStr for ConfigSet {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_kvp(kvp: &str) -> anyhow::Result<(Arc<str>, Arc<str>)> {
            let mut parts = kvp.splitn(2, ':').map(str::trim);
            let key = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Invalid config entry `{kvp}`: no key found"))?;
            let value = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Invalid config entry `{kvp}`: no value found"))?;
            Ok((Arc::from(key), Arc::from(value)))
        }

        let mut map = HashMap::new();

        for kvp in s.split(',').map(str::trim) {
            if kvp.is_empty() {
                println!("Warning: Ignoring empty config entry in `{s}`");
                continue;
            }
            let (key, value) = parse_kvp(kvp)?;

            if let Some(old) = map.insert(key.clone(), value.clone()) {
                println!(
                    "Warning: Duplicate config key `{}` found in `{s}`. Old value: `{}`, new value: `{}`. Using new value.",
                    old, key, value
                );
            }
        }
        Ok(Self(map))
    }
}

/// A trait for types that can be constructed from a config set.
pub trait FromConfig
where
    Self: Sized,
{
    /// The error type that can occur when parsing the config.
    type Err;

    /// Constructs the type from the given config set, consuming the config set in the process.
    ///  
    fn from_config(cfg: &mut ConfigSet) -> Result<Self, Self::Err>;
}
