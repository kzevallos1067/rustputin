// En src/lib.rs
pub mod engine;
pub mod domain;
pub mod stats;

// Re-exportaciones para un acceso más limpio del usuario:
pub use engine::simulador::Simulador;
pub use domain::ids::{EntityId, ResourceId};
pub use domain::entidad::Entidad;
pub use domain::recurso::Recurso;
pub use stats::monitor::MonitorEstadistico;