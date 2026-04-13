mod models;
mod mutations;
mod queries;
mod transactions;

mod entities {
    pub use entity::clients::{self, ActiveModel as ClientActiveModel, Entity as Clients};
    pub use entity::inventory_transactions::{
        self, ActiveModel as InventoryActiveModel, Entity as InventoryTransactions,
    };
    pub use entity::invoice_items::{
        self, ActiveModel as InvoiceItemActiveModel, Entity as InvoiceItems,
    };
    pub use entity::invoices::{self, ActiveModel as InvoiceActiveModel, Entity as Invoices};
    pub use entity::order_items::{
        self, ActiveModel as OrderItemActiveModel, Entity as OrderItems,
    };
    pub use entity::orders::{self, ActiveModel as OrderActiveModel, Entity as Orders};
    pub use entity::products::{self, ActiveModel as ProductActiveModel, Entity as Products};
    pub use entity::quote_items::{
        self, ActiveModel as QuoteItemActiveModel, Entity as QuoteItems,
    };
    pub use entity::quotes::{self, ActiveModel as QuoteActiveModel, Entity as Quotes};
    pub use entity::suppliers::{self, ActiveModel as SupplierActiveModel, Entity as Suppliers};
    pub use entity::templates::ActiveModel as TemplateActiveModel;
}

pub use models::*;
pub use mutations::*;
pub use queries::*;
pub use transactions::*;

pub use sea_orm;
