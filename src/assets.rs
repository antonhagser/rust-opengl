use crossbeam_channel::Receiver;
use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use notify::{DebouncedEvent, RecursiveMode, Watcher};
use std::{sync::Arc, thread};

pub use asset::{Asset, AssetKind};

mod asset;

#[derive(Clone)]
pub struct AssetManager {
    assets: Arc<DashMap<String, Asset>>,
    channel: Option<Receiver<(String, AssetKind)>>,
}

impl AssetManager {
    pub fn new() -> Self {
        info!("Initializing asset manager");

        Self {
            channel: None,
            assets: Arc::new(DashMap::new()),
        }
    }

    /// Trigger thread to allow hot-reload. <br>
    /// Only works in debug
    pub fn awake_hotreload(&mut self, path: String) {
        #[cfg(debug_assertions)]
        {
            info!("Initializing hot-reload feature");
            let (s, r) = crossbeam_channel::unbounded();
            self.channel = Some(r);

            let assets = self.assets.clone();
            thread::spawn(move || {
                info!("Created hot-reload thread");
                let (tx, rx) = std::sync::mpsc::channel();
                let mut watcher = notify::watcher(tx, std::time::Duration::from_secs(1)).unwrap();
                watcher
                    .watch(path.clone(), RecursiveMode::Recursive)
                    .unwrap();

                loop {
                    match rx.recv() {
                        Ok(event) => match event {
                            DebouncedEvent::Write(p) => {
                                info!("Received a write event on path");
                                let p = std::path::Path::new(&p);
                                let id = p.file_name().unwrap().to_string_lossy().to_string();

                                let asset = assets.get_mut(&id);
                                let mut asset = match asset {
                                    Some(asset) => asset,
                                    None => {
                                        dbg!("here {}", id);
                                        return;
                                    }
                                };

                                // Check if item should be reloaded
                                if !asset.should_reload {
                                    dbg!("here");
                                    continue;
                                }

                                trace!("Preparing reload");
                                let res = asset.reload();
                                match res {
                                    Ok(_) => {
                                        trace!("Successfully reloaded asset data");
                                        drop(asset);
                                        drop(res);

                                        // Fetch kind for assets
                                        let kind = assets.get(&id).unwrap().kind();

                                        // Send information to renderer
                                        s.send((id, kind)).unwrap();
                                        trace!("Successfully reloaded internal structures");
                                    }
                                    Err(e) => {
                                        warn!("An error occured while reloading asset: {}", e);
                                    }
                                }
                            }
                            _ => {}
                        },
                        Err(e) => println!("watch error: {:?}", e),
                    }
                }
            });
        }
    }

    pub fn create_asset(&mut self, asset: Asset) {
        info!("Creating asset {:?}", asset.path.to_str().unwrap());
        let id = asset
            .path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        self.assets.insert(id, asset);
    }

    pub fn register_for_hotreload(&mut self, path: std::path::PathBuf) -> Option<()> {
        #[cfg(debug_assertions)]
        {
            let id = std::path::Path::new(&path)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();
            trace!("Registering file for hot-reload {:?}", id.as_str());
            let mut asset = self.asset_mut(&id)?;
            asset.should_reload = true;
        }
        Some(())
    }

    pub fn asset_mut(&mut self, identifier: &str) -> Option<RefMut<'_, String, Asset>> {
        self.assets.get_mut(&identifier.to_string())
    }

    pub fn asset(&mut self, identifier: &str) -> Option<Ref<'_, String, Asset>> {
        self.assets.get(&identifier.to_string())
    }

    /// Get a reference to the asset manager's channel.
    pub fn channel(&self) -> &Option<Receiver<(String, AssetKind)>> {
        &self.channel
    }
}
