use crossbeam_channel::Receiver;
use notify::{DebouncedEvent, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    ffi::CString,
    io::Read,
    sync::{Arc, Mutex, RwLock},
    thread,
};

pub struct AssetManager {
    assets: Arc<Mutex<HashMap<String, Arc<RwLock<Asset>>>>>,
    channel: Option<Receiver<Arc<RwLock<Asset>>>>,
    has_hotreload: bool,
}

impl AssetManager {
    pub fn new() -> Self {
        info!("Initializing asset manager");

        Self {
            has_hotreload: false,
            channel: None,
            assets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Trigger thread to allow hot-reload. <br>
    /// Only works in debug
    pub fn awake_hotreload(&mut self, path: String) {
        #[cfg(debug_assertions)]
        {
            info!("Initializing hot-reload feature");
            self.has_hotreload = true;
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

                                let mut assets_lock = assets.lock().unwrap();
                                let asset = assets_lock.get_mut(&id);
                                let asset = match asset {
                                    Some(asset) => asset,
                                    None => {
                                        return;
                                    }
                                };

                                trace!("Locked asset, preparing reload");
                                let mut write_asset = asset.write().unwrap();
                                let res = write_asset.reload();
                                match res {
                                    Ok(_) => {
                                        info!("Successfully reloaded asset data");
                                        drop(write_asset);
                                        drop(res);

                                        // Send information to renderer
                                        let asset = assets_lock.get(&id).unwrap();
                                        s.send(asset.clone()).unwrap();
                                        info!("Successfully reloaded internal structures");
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
        let mut assets = self.assets.lock().unwrap();
        assets.insert(id, Arc::new(RwLock::new(asset)));
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
            let mut asset = self.assets.lock().unwrap();
            let asset = asset.get_mut(&id)?;

            let mut asset = asset.write().unwrap();
            asset.should_reload = true;
        }
        Some(())
    }

    /// Get a reference to the asset manager's channel.
    pub fn channel(&self) -> &Option<Receiver<Arc<RwLock<Asset>>>> {
        &self.channel
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AssetKind {
    Shader,
    Texture,
    Video,
}

pub struct Asset {
    name: String,
    path: std::path::PathBuf,
    raw: Vec<u8>,
    kind: AssetKind,
    should_reload: bool,
    identifier: String,
    kind_identifier: u8,
}

impl Asset {
    pub fn new(
        name: String,
        path: std::path::PathBuf,
        kind: AssetKind,
        identifier: &str,
        kind_identifier: u8,
    ) -> Result<Self, String> {
        let mut a = Asset {
            name,
            path,
            raw: Vec::new(),
            kind,
            should_reload: false,
            identifier: identifier.to_string(),
            kind_identifier,
        };
        #[cfg(debug_assertions)]
        a.reload()?;

        Ok(a)
    }

    pub fn reload(&mut self) -> Result<(), String> {
        info!("Reloading asset data");
        let f = std::fs::File::open(self.path.clone());
        let mut f = match f {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let mut buf = Vec::new();
        let b = f.read_to_end(&mut buf).expect("Failed reading content of file");

        if b == 0 {
            warn!("Buffer from reload file was empty and could therefore not be read");
        }

        self.raw = buf;

        Ok(())
    }

    pub fn raw_to_cstr(&self) -> std::ffi::CString {
        let s = std::str::from_utf8(&self.raw).expect("Failed conversion to string");
        CString::new(s).unwrap()
    }

    /// Get a reference to the asset's kind.
    pub fn kind(&self) -> AssetKind {
        self.kind
    }

    /// Get a reference to the asset's identifier.
    pub fn identifier(&self) -> &String {
        &self.identifier
    }

    /// Get a reference to the asset's kind identifier.
    pub fn kind_identifier(&self) -> &u8 {
        &self.kind_identifier
    }
}
