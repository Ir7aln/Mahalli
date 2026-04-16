mod models;
mod mutations;
mod queries;
mod transactions;

mod entities {
    pub use tenant_entity::clients::{self, ActiveModel as ClientActiveModel, Entity as Clients};
    pub use tenant_entity::inventory_transactions::{
        self, ActiveModel as InventoryActiveModel, Entity as InventoryTransactions,
    };
    pub use tenant_entity::invoice_items::{
        self, ActiveModel as InvoiceItemActiveModel, Entity as InvoiceItems,
    };
    pub use tenant_entity::invoices::{
        self, ActiveModel as InvoiceActiveModel, Entity as Invoices,
    };
    pub use tenant_entity::order_items::{
        self, ActiveModel as OrderItemActiveModel, Entity as OrderItems,
    };
    pub use tenant_entity::orders::{self, ActiveModel as OrderActiveModel, Entity as Orders};
    pub use tenant_entity::products::{
        self, ActiveModel as ProductActiveModel, Entity as Products,
    };
    pub use tenant_entity::quote_items::{
        self, ActiveModel as QuoteItemActiveModel, Entity as QuoteItems,
    };
    pub use tenant_entity::quotes::{self, ActiveModel as QuoteActiveModel, Entity as Quotes};
    pub use tenant_entity::suppliers::{
        self, ActiveModel as SupplierActiveModel, Entity as Suppliers,
    };
    pub use tenant_entity::templates::ActiveModel as TemplateActiveModel;
}

pub use models::*;
pub use mutations::*;
pub use queries::*;
pub use transactions::*;

pub use sea_orm;
