use camino::Utf8PathBuf;
use cargo_metadata::Metadata;

use crate::ext::{PathBufExt as _, PathExt as _};

use super::ProjectConfig;
use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub struct StylanceConfig {
    pub package_dir: Utf8PathBuf,
    pub tmp_file: Utf8PathBuf,
}

impl StylanceConfig {
    pub fn new(conf: &ProjectConfig, metadata: &Metadata) -> Result<Option<Self>> {
        let Some(ref stylance_package) = conf.stylance_package else {
            return Ok(None);
        };

        let package_abs_dir = metadata
            .workspace_packages()
            .iter()
            .find_map(|p| {
                (&p.name == stylance_package).then(|| p.manifest_path.clone().without_last())
            })
            .ok_or_else(|| {
                anyhow!(r#"Could not find the project stylance-package "{stylance_package}""#,)
            })?;
        let package_dir = package_abs_dir.unbase(&metadata.workspace_root)?;

        let tmp_file = conf.tmp_dir.join("stylance.css");

        Ok(Some(Self {
            package_dir,
            tmp_file,
        }))
    }
}
