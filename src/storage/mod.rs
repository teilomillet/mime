mod traits;

pub use traits::StorageBackend;

#[cfg(not(target_arch = "wasm32"))]
mod desktop;
#[cfg(not(target_arch = "wasm32"))]
pub use desktop::DesktopStorage as PlatformStorage;

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::WebStorage as PlatformStorage;

pub fn create_storage() -> PlatformStorage {
    PlatformStorage::new()
}
