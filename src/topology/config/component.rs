use inventory;
use serde::de::{Deserializer, Error};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use toml::Value;

/// Combines a type field and a nested plugin config to create a table.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigSwapOut {
    #[serde(rename = "type")]
    pub type_str: String,
    #[serde(flatten)]
    pub nested: Value,
}

/// Stores both a constructed plugin instance and the ConfigSwapOut used to
/// create it, this allows us to echo back the config of a component at runtime.
pub struct ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    swap_out: ConfigSwapOut,

    pub component: T,
}

impl<T> ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    /// Returns a sorted Vec of all plugins registered of a type.
    pub fn types() -> Vec<&'static str> {
        let mut types = Vec::new();
        for definition in inventory::iter::<ComponentBuilder<T>> {
            types.push(definition.name);
        }
        types.sort();
        types
    }
}

impl<T> Serialize for ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.swap_out.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for ComponentConfig<T>
where
    T: 'static + Sized,
    inventory::iter<ComponentBuilder<T>>:
        std::iter::IntoIterator<Item = &'static ComponentBuilder<T>>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let swap_out = ConfigSwapOut::deserialize(deserializer)?;
        match inventory::iter::<ComponentBuilder<T>>
            .into_iter()
            .find(|t| t.name == swap_out.type_str)
        {
            Some(b) => match (b.from_value)(swap_out.nested.clone()) {
                Ok(c) => Ok(Self {
                    swap_out: swap_out,
                    component: c,
                }),
                Err(e) => Err(Error::custom(format!(
                    "failed to parse type `{}`: {}",
                    swap_out.type_str, e,
                ))),
            },
            None => Err(Error::custom(format!(
                "unrecognized type '{}'",
                swap_out.type_str,
            ))),
        }
    }
}

pub struct ComponentBuilder<T: Sized> {
    pub name: &'static str,
    from_value: fn(Value) -> Result<T, String>,
}

impl<T: Sized> ComponentBuilder<T> {
    pub fn new<'de, B: Deserialize<'de> + Into<T>>(name: &'static str) -> Self {
        ComponentBuilder {
            name: name,
            from_value: |value| {
                value
                    .try_into::<B>()
                    .map(|c| c.into())
                    .map_err(|e| format!("{}", e))
            },
        }
    }
}
