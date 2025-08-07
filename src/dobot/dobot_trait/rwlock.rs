#[cfg(not(feature = "std"))]
extern crate spin;

#[cfg(not(feature = "std"))]
pub type RwLock<T> = spin::RwLock<T>;

#[cfg(feature = "std")]
pub type RwLock<T> = std::sync::RwLock<T>;

