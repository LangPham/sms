pub(crate) mod conn;
pub(crate) mod models;
pub(crate) mod schema;
pub(crate) mod services;

pub mod prelude {
    pub use crate::{
        models::*,
        services::{svc_brand, svc_campus, svc_people},
    };
}
