use super::{Theme, ThemeError};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::{LazyLock, RwLock};

#[derive(Clone, Deserialize, PartialEq, Eq)]
struct ThemePack {
    schema_version: u32,
    pack: String,
    themes: Vec<Theme>,
}

#[derive(Default)]
struct Catalog {
    packs: BTreeMap<String, ThemePack>,
    themes: BTreeMap<String, &'static Theme>,
    host_initialized: bool,
}

static CATALOG: LazyLock<RwLock<Catalog>> = LazyLock::new(|| {
    let mut catalog = Catalog::default();
    let packs = lumis_themes::packs()
        .map(parse)
        .collect::<Result<_, _>>()
        .expect("invalid bundled theme pack");
    catalog
        .install(packs)
        .expect("conflicting bundled theme packs");
    RwLock::new(catalog)
});

fn invalid(message: impl Into<String>) -> ThemeError {
    ThemeError::InvalidJson(message.into())
}

fn parse(source: &str) -> Result<ThemePack, ThemeError> {
    let pack: ThemePack = serde_json::from_str(source)?;
    if pack.schema_version != 1 || !matches!(pack.pack.as_str(), "default" | "copyleft") {
        return Err(invalid("unsupported theme pack schema or identity"));
    }
    let mut names = std::collections::BTreeSet::new();
    for theme in &pack.themes {
        if theme.name.is_empty() || theme.highlights.is_empty() || !names.insert(&theme.name) {
            return Err(invalid("empty or duplicate theme in pack"));
        }
    }
    if pack.themes.is_empty() {
        return Err(invalid("empty theme pack"));
    }
    Ok(pack)
}

impl Catalog {
    fn initialize_host(&mut self, packs: Vec<ThemePack>) -> Result<(), ThemeError> {
        if packs.is_empty() {
            return Ok(());
        }
        if self.host_initialized {
            return self.install(packs);
        }
        let mut incoming = BTreeMap::new();
        for pack in packs {
            if incoming
                .get(&pack.pack)
                .is_some_and(|existing| existing != &pack)
            {
                return Err(invalid(format!("conflicting {} theme pack", pack.pack)));
            }
            incoming.insert(pack.pack.clone(), pack);
        }
        let mut merged = self.packs.clone();
        merged.extend(incoming);
        let mut catalog = Self::default();
        catalog.install(merged.into_values().collect())?;
        catalog.host_initialized = true;
        *self = catalog;
        Ok(())
    }

    fn install(&mut self, packs: Vec<ThemePack>) -> Result<(), ThemeError> {
        let mut pending = BTreeMap::new();
        let mut names = self
            .themes
            .keys()
            .cloned()
            .collect::<std::collections::BTreeSet<_>>();
        for pack in packs {
            if let Some(existing) = self
                .packs
                .get(&pack.pack)
                .or_else(|| pending.get(&pack.pack))
            {
                if existing != &pack {
                    return Err(invalid(format!("conflicting {} theme pack", pack.pack)));
                }
                continue;
            }
            for theme in &pack.themes {
                if !names.insert(theme.name.clone()) {
                    return Err(invalid(format!(
                        "duplicate theme {} across packs",
                        theme.name
                    )));
                }
            }
            pending.insert(pack.pack.clone(), pack);
        }
        for (name, pack) in pending {
            for theme in &pack.themes {
                self.themes
                    .insert(theme.name.clone(), Box::leak(Box::new(theme.clone())));
            }
            self.packs.insert(name, pack);
        }
        Ok(())
    }
}

/// Installs host-discovered theme packs into this runtime's catalog.
#[doc(hidden)]
pub fn install_packs(sources: &[&str]) -> Result<(), ThemeError> {
    let packs = sources
        .iter()
        .map(|source| parse(source))
        .collect::<Result<_, _>>()?;
    CATALOG
        .write()
        .expect("theme catalog lock poisoned")
        .initialize_host(packs)
}

/// Retrieves a theme by its name.
pub fn get(name: &str) -> Result<Theme, ThemeError> {
    CATALOG
        .read()
        .expect("theme catalog lock poisoned")
        .themes
        .get(name)
        .map(|theme| (*theme).clone())
        .ok_or_else(|| ThemeError::NotFound(name.to_owned()))
}

/// Returns an iterator over the themes available in this runtime.
pub fn available_themes() -> impl Iterator<Item = &'static Theme> {
    CATALOG
        .read()
        .expect("theme catalog lock poisoned")
        .themes
        .values()
        .copied()
        .collect::<Vec<_>>()
        .into_iter()
}
