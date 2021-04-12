use notify::{DebouncedEvent, RecursiveMode, Watcher};
use std::{
    collections::HashMap,
    io::Read,
    sync::{Arc, Mutex, RwLock},
    thread,
};

pub struct AssetManager {
    assets: Arc<Mutex<HashMap<String, RwLock<Asset>>>>,
    channel: Option<std::sync::mpsc::Sender<i32>>,
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
            info!("Initializing hot-realod feature");
            let (sender, receiver) = std::sync::mpsc::channel();
            self.has_hotreload = true;
            self.channel = Some(sender);

            let assets = self.assets.clone();
            thread::spawn(move || {
                info!("Created hot-reload thread");
                let (tx, rx) = std::sync::mpsc::channel();
                let mut watcher = notify::watcher(tx, std::time::Duration::from_secs(1)).unwrap();
                watcher
                    .watch(path.clone(), RecursiveMode::Recursive)
                    .unwrap();
                let _ = receiver;

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
                                let asset = asset.get_mut().unwrap();
                                let res = asset.reload();
                                match res {
                                    Ok(_) => {
                                        info!("Successfully reloaded asset");
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
        assets.insert(id, RwLock::new(asset));
    }

    pub fn register_for_hotreload(&mut self, path: std::path::PathBuf) -> Option<()> {
        let id = std::path::Path::new(&path)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        trace!("Registering file for hot-reload {:?}", id.as_str());
        let mut asset = self.assets.lock().unwrap();
        let asset = asset.get_mut(&id)?;
        let asset = asset.get_mut();
        let asset = match asset {
            Ok(a) => a,
            Err(_) => return None,
        };

        asset.should_reload = true;
        Some(())
    }
}

pub enum AssetKind {
    Text,
    Texture,
    Video,
}

pub struct Asset {
    name: String,
    path: std::path::PathBuf,
    raw: Vec<u8>,
    kind: AssetKind,
    should_reload: bool,
}

impl Asset {
    pub fn new(path: std::path::PathBuf, kind: AssetKind) -> Result<Self, String> {
        let mut a = Asset {
            name: std::path::Path::new(&path)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            path,
            raw: Vec::new(),
            kind,
            should_reload: false,
        };
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
        f.read(&mut buf).expect("Failed reading content of file");

        self.raw = buf;

        // Todo: how do I trigger a function on something else? Well I know how, just not sure how to implement it, hmm.

        Ok(())
    }
}
