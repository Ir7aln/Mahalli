pub mod models;
pub mod mutations;
pub mod queries;

mod entities {
    pub use system_entity::databases::{
        self, ActiveModel as DatabaseActiveModel, Entity as Databases,
    };
}

pub use models::*;
pub use mutations::*;
pub use queries::*;

pub use sea_orm;
