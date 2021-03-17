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

use std::fs;
use std::path::PathBuf;

use internet2::zmqsocket::ZmqSocketAddr;
use lnpbp::Chain;
use microservices::FileFormat;

use crate::{cache, storage};

const STORAGE_FORMAT: FileFormat = FileFormat::Yaml;
const CACHE_FORMAT: FileFormat = FileFormat::Yaml;

/// Final configuration resulting from data contained in config file environment
/// variables and command-line options. For security reasons node key is kept
/// separately.
#[derive(Clone, PartialEq, Eq, Debug, Display)]
#[display(Debug)]
pub struct Config {
    /// Bitcoin blockchain to use (mainnet, testnet, signet, liquid etc)
    pub chain: Chain,

    /// ZMQ socket for RPC API
    pub rpc_endpoint: ZmqSocketAddr,

    /// RGB20 ZMQ RPC API endpoint
    pub rgb20_endpoint: ZmqSocketAddr,

    /// Whether to run embedded RGB node
    pub rgb_embedded: bool,

    /// Data location
    pub data_dir: PathBuf,

    /// Verbosity level
    pub verbose: u8,

    /// Electrum server connection string
    pub electrum_server: String,
}

impl Config {
    pub fn storage_conf(&self) -> storage::FileConfig {
        storage::FileConfig {
            location: self.data_dir.to_string_lossy().to_string(),
            format: STORAGE_FORMAT,
        }
    }

    pub fn cache_conf(&self) -> cache::FileConfig {
        cache::FileConfig {
            location: self.data_dir.to_string_lossy().to_string(),
            format: CACHE_FORMAT,
        }
    }
}

impl Config {
    pub fn process(&mut self) {
        self.data_dir = PathBuf::from(
            shellexpand::tilde(&self.data_dir.to_string_lossy().to_string())
                .to_string(),
        );

        let me = self.clone();
        let mut data_dir = self.data_dir.to_string_lossy().into_owned();
        self.process_dir(&mut data_dir);
        self.data_dir = PathBuf::from(data_dir);

        fs::create_dir_all(&self.data_dir)
            .expect("Unable to access data directory");

        for dir in vec![&mut self.rpc_endpoint, &mut self.rgb20_endpoint] {
            match dir {
                ZmqSocketAddr::Ipc(ref mut path) => {
                    me.process_dir(path);
                }
                _ => {}
            }
        }
    }

    pub fn process_dir(&self, path: &mut String) {
        *path = path.replace("{data_dir}", &self.data_dir.to_string_lossy());
        *path = path.replace("{network}", &self.chain.to_string());
        *path = path.replace("{id}", "default");
        *path = shellexpand::tilde(path).to_string();
    }
}
