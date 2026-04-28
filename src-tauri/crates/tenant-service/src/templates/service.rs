use super::dto::*;
use sea_orm::*;
use tenant_entity::{
    templates::{self, ActiveModel as TemplateActiveModel},
    prelude::*,
};

pub struct TemplatesService;

impl TemplatesService {
    pub async fn create_template(db: &DbConn, template: NewTemplate) -> Result<String, DbErr> {
        let template = TemplateActiveModel {
            values_json: ActiveValue::Set(template.values_json),
            ..Default::default()
        };
        match template.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
}
