// Citadel: Bitcoin, LN & RGB wallet runtime
// Written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@mycitadel.io>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the AGPL License
// along with this software.
// If not, see <https://www.gnu.org/licenses/agpl-3.0-standalone.html>.

use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::{fs, io};

use lnpbp::strict_encoding::{StrictDecode, StrictEncode};
use microservices::FileFormat;

use super::{Cache, ContractCache};
use crate::cache::Error;
use crate::model::ContractId;

const CACHE_FILENAME: &'static str = "cache";

#[derive(
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Serialize,
    Deserialize,
    StrictEncode,
    StrictDecode,
)]
pub struct FileConfig {
    pub location: String,
    pub format: FileFormat,
}

impl FileConfig {
    pub fn filename(&self) -> PathBuf {
        let mut filename = PathBuf::from(self.location.clone());
        filename.push(CACHE_FILENAME);
        filename.set_extension(self.format.extension());
        filename
    }
}

#[derive(Debug)]
pub struct FileDriver {
    fd: fs::File,
    config: FileConfig,
    pub(super) cache: Cache,
}

impl FileDriver {
    pub fn with(config: FileConfig) -> Result<Self, Error> {
        info!("Initializing file driver for cache in {}", &config.location);
        fs::create_dir_all(&config.location)?;

        let filename = config.filename();
        let exists = filename.exists();
        let fd = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(!exists)
            .open(&filename)?;
        let mut me = Self {
            fd,
            config: config.clone(),
            cache: none!(),
        };
        if !exists {
            warn!(
                "Cache file `{:?}` does not exist: initializing empty citadel cache",
                filename
            );
            me.store()?;
        } else {
            me.load()?;
        }
        Ok(me)
    }

    pub(super) fn load(&mut self) -> Result<(), Error> {
        debug!("Loading cache from `{:?}`", self.config.filename());
        self.fd.seek(io::SeekFrom::Start(0))?;
        trace!("Parsing cache (expected format {})", self.config.format);
        self.cache = match self.config.format {
            FileFormat::StrictEncode => Cache::strict_decode(&mut self.fd)?,
            FileFormat::Yaml => serde_yaml::from_reader(&mut self.fd)?,
            FileFormat::Toml => {
                let mut data: Vec<u8> = vec![];
                self.fd.read_to_end(&mut data)?;
                toml::from_slice(&data)?
            }
            FileFormat::Json => serde_json::from_reader(&mut self.fd)?,
            _ => unimplemented!(),
        };
        trace!("Cache loaded from storage");
        Ok(())
    }

    pub(super) fn store(&mut self) -> Result<(), Error> {
        debug!(
            "Storing cache to the file `{:?}` in {} format",
            self.config.filename(),
            self.config.format
        );
        self.fd.seek(io::SeekFrom::Start(0))?;
        self.fd.set_len(0)?;
        match self.config.format {
            FileFormat::StrictEncode => {
                self.cache.strict_encode(&mut self.fd)?;
            }
            FileFormat::Yaml => {
                serde_yaml::to_writer(&mut self.fd, &self.cache)?;
            }
            FileFormat::Toml => {
                let data = toml::to_vec(&self.cache)?;
                self.fd.write_all(&data)?;
            }
            FileFormat::Json => {
                serde_json::to_writer(&mut self.fd, &self.cache)?;
            }
            _ => unimplemented!(),
        };
        trace!("Cache stored");
        Ok(())
    }

    pub(super) fn map_contract_or_default<R>(
        &self,
        contract_id: ContractId,
        predicate: impl FnOnce(&ContractCache) -> R,
    ) -> Result<R, Error>
    where
        R: Sized + Default,
    {
        Ok(self
            .cache
            .descriptors
            .get(&contract_id)
            .map(predicate)
            .unwrap_or_default())
    }

    pub(super) fn with_contract<R>(
        &mut self,
        contract_id: ContractId,
        predicate: impl FnOnce(&mut ContractCache) -> Result<R, Error>,
    ) -> Result<R, Error>
    where
        R: Sized,
    {
        predicate(
            self.cache
                .descriptors
                .entry(contract_id)
                .or_insert(default!()),
        )
        .and_then(|r| {
            self.store()?;
            Ok(r)
        })
    }
}
