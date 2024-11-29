//! Repository traits for domain entities
//! 
//! This module contains trait definitions for all repository interfaces.
//! These traits define the contract that concrete implementations must fulfill.

pub mod base;
mod organization;

pub use organization::OrganizationRepository;

pub mod user;

pub use user::UserRepository; 