use std::fmt::Debug;
use std::path::Path;

use glommio::io::{ImmutableFile, ImmutableFileBuilder};
// TODO move to more appropriate module

#[derive(Debug, Clone)]
pub(crate) struct DiskImageSource {
    path: Box<Path>,
}

impl DiskImageSource {
    pub fn new(path: Box<Path>) -> DiskImageSource {
        DiskImageSource { path }
    }

    pub async fn resolve(&self, ident: &str) -> glommio::Result<ImmutableFile, ()> {
        let file = self.path.join(ident);
        ImmutableFileBuilder::new(file).build_existing().await
    }
}
