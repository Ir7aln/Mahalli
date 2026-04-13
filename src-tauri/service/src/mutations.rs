use sea_orm::*;

use crate::models::*;

use crate::entities::*;

pub struct MutationsService;

impl MutationsService {
    pub async fn create_product(db: &DbConn, product: NewProduct) -> Result<String, DbErr> {
        let product = ProductActiveModel {
            name: ActiveValue::Set(product.name),
            selling_price: ActiveValue::Set(product.selling_price),
            purchase_price: ActiveValue::Set(product.purchase_price),
            image: ActiveValue::Set(product.image),
            description: ActiveValue::Set(product.description),
            min_quantity: ActiveValue::Set(product.min_quantity),
            ..Default::default()
        };
        match product.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_product(db: &DbConn, product: Product) -> Result<(), DbErr> {
        let product_model = Products::find_by_id(product.id).one(db).await?;
        let mut product_active: ProductActiveModel = product_model.unwrap().into();
        product_active.name = ActiveValue::Set(product.name);
        product_active.selling_price = ActiveValue::Set(product.selling_price);
        product_active.purchase_price = ActiveValue::Set(product.purchase_price);
        // product_active.image = ActiveValue::Set(product.image);
        product_active.description = ActiveValue::Set(product.description);
        product_active.min_quantity = ActiveValue::Set(product.min_quantity);
        match product_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn partial_update_product(db: &DbConn, product: UpdateProduct) -> Result<(), DbErr> {
        let product_model = Products::find_by_id(product.id).one(db).await?;
        let mut product_active: ProductActiveModel = product_model.unwrap().into();
        if let Some(name) = product.name {
            product_active.name = ActiveValue::Set(name);
        }
        if let Some(selling_price) = product.selling_price {
            product_active.selling_price = ActiveValue::Set(selling_price);
        }
        if let Some(purchase_price) = product.purchase_price {
            product_active.purchase_price = ActiveValue::Set(purchase_price);
        }
        if let Some(description) = product.description {
            product_active.description = ActiveValue::Set(Some(description));
        }
        if let Some(min_quantity) = product.min_quantity {
            product_active.min_quantity = ActiveValue::Set(min_quantity);
        }
        if let Some(image) = product.image {
            product_active.image = ActiveValue::Set(Some(image));
        }
        match product_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_product(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let product_model = Products::find_by_id(id).one(db).await?;
        let mut product_active: ProductActiveModel = product_model.unwrap().into();
        product_active.is_deleted = ActiveValue::Set(true);
        match product_active.update(db).await {
            Ok(_) => Ok(1),
            Err(err) => Err(err),
        }
    }
    //
    pub async fn create_client(db: &DbConn, client: NewClient) -> Result<String, DbErr> {
        let client = ClientActiveModel {
            full_name: ActiveValue::Set(client.full_name),
            email: ActiveValue::Set(client.email),
            phone_number: ActiveValue::Set(client.phone_number),
            address: ActiveValue::Set(client.address),
            image: ActiveValue::Set(client.image),
            ..Default::default()
        };
        match client.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_client(db: &DbConn, client: Client) -> Result<(), DbErr> {
        let client_model = Clients::find_by_id(client.id).one(db).await?;
        let mut client_active: ClientActiveModel = client_model.unwrap().into();
        client_active.full_name = ActiveValue::Set(client.full_name);
        client_active.email = ActiveValue::Set(client.email);
        client_active.phone_number = ActiveValue::Set(client.phone_number);
        client_active.address = ActiveValue::Set(client.address);
        // client_active.image = ActiveValue::Set(client.image);
        match client_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn partial_update_client(db: &DbConn, client: UpdateClient) -> Result<(), DbErr> {
        let client_model = Clients::find_by_id(client.id).one(db).await?;
        let mut client_active: ClientActiveModel = client_model.unwrap().into();
        if let Some(full_name) = client.full_name {
            client_active.full_name = ActiveValue::Set(full_name);
        }
        if let Some(email) = client.email {
            client_active.email = ActiveValue::Set(Some(email));
        }
        if let Some(phone_number) = client.phone_number {
            client_active.phone_number = ActiveValue::Set(Some(phone_number));
        }
        if let Some(address) = client.address {
            client_active.address = ActiveValue::Set(Some(address));
        }
        if let Some(image) = client.image {
            client_active.image = ActiveValue::Set(Some(image));
        }
        match client_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_client(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let client_model = Clients::find_by_id(id).one(db).await?;
        let mut client_active: ClientActiveModel = client_model.unwrap().into();
        client_active.is_deleted = ActiveValue::Set(true);
        match client_active.update(db).await {
            Ok(_) => Ok(1),
            Err(err) => Err(err),
        }
    }
    //
    pub async fn create_supplier(db: &DbConn, supplier: NewSupplier) -> Result<String, DbErr> {
        let supplier = SupplierActiveModel {
            full_name: ActiveValue::Set(supplier.full_name),
            email: ActiveValue::Set(supplier.email),
            phone_number: ActiveValue::Set(supplier.phone_number),
            address: ActiveValue::Set(supplier.address),
            image: ActiveValue::Set(supplier.image),
            ..Default::default()
        };
        match supplier.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_supplier(db: &DbConn, supplier: Supplier) -> Result<(), DbErr> {
        let supplier_model = Suppliers::find_by_id(supplier.id).one(db).await?;
        let mut supplier_active: SupplierActiveModel = supplier_model.unwrap().into();
        supplier_active.full_name = ActiveValue::Set(supplier.full_name);
        supplier_active.email = ActiveValue::Set(supplier.email);
        supplier_active.phone_number = ActiveValue::Set(supplier.phone_number);
        supplier_active.address = ActiveValue::Set(supplier.address);
        // supplier_active.image = ActiveValue::Set(supplier.image);
        match supplier_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn partial_update_supplier(
        db: &DbConn,
        supplier: UpdateSupplier,
    ) -> Result<(), DbErr> {
        let supplier_model = Suppliers::find_by_id(supplier.id).one(db).await?;
        let mut supplier_active: SupplierActiveModel = supplier_model.unwrap().into();
        if let Some(full_name) = supplier.full_name {
            supplier_active.full_name = ActiveValue::Set(full_name);
        }
        if let Some(email) = supplier.email {
            supplier_active.email = ActiveValue::Set(Some(email));
        }
        if let Some(phone_number) = supplier.phone_number {
            supplier_active.phone_number = ActiveValue::Set(Some(phone_number));
        }
        if let Some(address) = supplier.address {
            supplier_active.address = ActiveValue::Set(Some(address));
        }
        if let Some(image) = supplier.image {
            supplier_active.image = ActiveValue::Set(Some(image));
        }
        match supplier_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_supplier(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let supplier_model = Suppliers::find_by_id(id).one(db).await?;
        let mut supplier_active: SupplierActiveModel = supplier_model.unwrap().into();
        supplier_active.is_deleted = ActiveValue::Set(true);
        match supplier_active.update(db).await {
            Ok(_) => Ok(1),
            Err(err) => Err(err),
        }
    }
    //
    pub async fn create_inventory(db: &DbConn, transaction: NewInventory) -> Result<String, DbErr> {
        let in_transaction = InventoryActiveModel {
            transaction_type: ActiveValue::Set(transaction.transaction_type),
            quantity: ActiveValue::Set(transaction.quantity),
            product_id: ActiveValue::Set(transaction.product_id),
            ..Default::default()
        };
        match in_transaction.insert(db).await {
            Ok(im) => Ok(im.id),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_inventory(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let transaction_model = InventoryTransactions::find_by_id(id).one(db).await?;
        match transaction_model {
            Some(transaction_model) => {
                let transaction = transaction_model.delete(db).await?;
                Ok(transaction.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn update_order_status(db: &DbConn, data: UpdateStatus) -> Result<(), DbErr> {
        let order_model = Orders::find_by_id(data.id).one(db).await?;
        let mut order_active: OrderActiveModel = order_model.unwrap().into();
        order_active.status = ActiveValue::Set(data.status);
        match order_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_order(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let order_model = Orders::find_by_id(id).one(db).await?;
        let mut order_active: OrderActiveModel = order_model.unwrap().into();
        order_active.is_deleted = ActiveValue::Set(true);
        match order_active.update(db).await {
            Ok(_) => Ok(1),
            Err(err) => Err(err),
        }
    }
    //
    pub async fn update_invoice_status(db: &DbConn, data: UpdateStatus) -> Result<(), DbErr> {
        let invoice_model = Invoices::find_by_id(data.id).one(db).await?;
        let mut invoice_active: InvoiceActiveModel = invoice_model.unwrap().into();
        invoice_active.status = ActiveValue::Set(data.status);
        match invoice_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_invoice(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let invoice_model = Invoices::find_by_id(id).one(db).await?;
        let mut invoice_active: InvoiceActiveModel = invoice_model.unwrap().into();
        invoice_active.is_deleted = ActiveValue::Set(true);
        match invoice_active.update(db).await {
            Ok(_) => Ok(1),
            Err(err) => Err(err),
        }
    }
    //
    pub async fn delete_quote(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let quote_model = Quotes::find_by_id(id).one(db).await?;
        let mut quote_active: QuoteActiveModel = quote_model.unwrap().into();
        quote_active.is_deleted = ActiveValue::Set(true);
        match quote_active.update(db).await {
            Ok(_) => Ok(1),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_quote_item(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let quote_item_model = QuoteItems::find_by_id(id).one(db).await?;
        match quote_item_model {
            Some(quote_item_model) => {
                let quote_item = quote_item_model.delete(db).await?;
                Ok(quote_item.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn delete_order_item(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let item_model = OrderItems::find_by_id(id).one(db).await?;
        match item_model {
            Some(item_model) => {
                let res = item_model.delete(db).await?;
                Ok(res.rows_affected)
            }
            None => Ok(0),
        }
    }

    pub async fn delete_invoice_item(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let item_model = InvoiceItems::find_by_id(&id).one(db).await?;
        match item_model {
            Some(item_model) => {
                let res = item_model.delete(db).await?;
                Ok(res.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
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
