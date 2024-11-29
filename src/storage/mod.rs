#[cfg(feature = "metamask")]
mod snap_persister;

#[cfg(feature = "metamask")]
pub use snap_persister::SnapPersister;
