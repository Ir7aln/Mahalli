use super::dto::*;
use sea_orm::{
    sea_query::{
        Alias, Cond, Expr, Func, Query, SimpleExpr, SqliteQueryBuilder, SubQueryStatement,
    },
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    inventory_transactions::{self, Entity as InventoryTransactions},
    order_items::{self, Entity as OrderItems},
    orders::{self, Entity as Orders},
    products::{self, ActiveModel as ProductActiveModel, Entity as Products},
};

fn requested_order(direction: Option<&str>) -> Order {
    if matches!(direction, Some("asc")) {
        Order::Asc
    } else {
        Order::Desc
    }
}

fn product_inventory_expr() -> SimpleExpr {
    SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .from(InventoryTransactions)
                .expr(Func::coalesce([
                    Expr::expr(Func::sum(Expr::col(
                        inventory_transactions::Column::Quantity,
                    ))),
                    Expr::val(0.0f64),
                ]))
                .cond_where(
                    Cond::all()
                        .add(inventory_transactions::Column::TransactionType.eq(String::from("IN")))
                        .add(
                            Expr::col((
                                InventoryTransactions,
                                inventory_transactions::Column::ProductId,
                            ))
                            .equals((Products, products::Column::Id)),
                        ),
                )
                .to_owned(),
        )),
    )
    .sub(SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .from(InventoryTransactions)
                .expr(Func::coalesce([
                    Expr::expr(Func::sum(Expr::col(
                        inventory_transactions::Column::Quantity,
                    ))),
                    Expr::val(0.0f64),
                ]))
                .join(
                    JoinType::Join,
                    OrderItems,
                    Expr::col((OrderItems, order_items::Column::InventoryId)).equals((
                        InventoryTransactions,
                        inventory_transactions::Column::Id,
                    )),
                )
                .join(
                    JoinType::Join,
                    Orders,
                    Expr::col((Orders, orders::Column::Id))
                        .equals((OrderItems, order_items::Column::OrderId)),
                )
                .cond_where(
                    Cond::all()
                        .add(
                            Expr::col((
                                InventoryTransactions,
                                inventory_transactions::Column::ProductId,
                            ))
                            .equals((Products, products::Column::Id)),
                        )
                        .add(orders::Column::Status.is_in(["DELIVERED", "SHIPPED"]))
                        .add(orders::Column::IsDeleted.eq(false)),
                )
                .to_owned(),
        )),
    ))
}

fn product_search_condition(search: &str) -> Cond {
    let pattern = format!("%{}%", search);
    Cond::any()
        .add(Expr::col((Products, products::Column::Name)).like(pattern.clone()))
        .add(Expr::col((Products, products::Column::Description)).like(pattern))
}

pub struct ProductsService;

impl ProductsService {
    pub async fn list_products(
        db: &DbConn,
        args: ListProductsArgs,
    ) -> Result<ProductsResponse, DbErr> {
        let count = Products::find()
            .filter(
                Cond::all()
                    .add(Expr::col((Products, products::Column::IsArchived)).eq(false))
                    .add(Expr::col((Products, products::Column::IsDeleted)).eq(false))
                    .add(product_search_condition(&args.search)),
            )
            .apply_if(args.selling_price_min, |query, value| {
                query.filter(Expr::col((Products, products::Column::SellingPrice)).gte(value))
            })
            .apply_if(args.selling_price_max, |query, value| {
                query.filter(Expr::col((Products, products::Column::SellingPrice)).lte(value))
            })
            .apply_if(args.stock_status.clone(), |query, stock_status| match stock_status.as_str() {
                "out" => query.filter(product_inventory_expr().lte(0.0)),
                "low" => query.filter(
                    product_inventory_expr()
                        .gt(0.0)
                        .and(product_inventory_expr().lte(Expr::col((Products, products::Column::MinQuantity)))),
                ),
                "healthy" => query.filter(
                    product_inventory_expr().gt(Expr::col((Products, products::Column::MinQuantity))),
                ),
                _ => query,
            })
            .count(db)
            .await?;

        let mut query = Query::select();
        query
            .from(Products)
            .exprs([
                Expr::col((Products, products::Column::Id)),
                Expr::col((Products, products::Column::Name)),
                Expr::col((Products, products::Column::Description)),
                Expr::col((Products, products::Column::Image)),
                Expr::col((Products, products::Column::CreatedAt)),
                Expr::col((Products, products::Column::SellingPrice)),
                Expr::col((Products, products::Column::PurchasePrice)),
                Expr::col((Products, products::Column::MinQuantity)),
            ])
            .expr_as(product_inventory_expr(), Alias::new("inventory"))
            .cond_where(
                Cond::all()
                    .add(Expr::col((Products, products::Column::IsArchived)).eq(false))
                    .add(Expr::col((Products, products::Column::IsDeleted)).eq(false))
                    .add(product_search_condition(&args.search)),
            )
            .conditions(
                args.selling_price_min.is_some(),
                |query| {
                    query.and_where(
                        Expr::col((Products, products::Column::SellingPrice))
                            .gte(args.selling_price_min),
                    );
                },
                |_| {},
            )
            .conditions(
                args.selling_price_max.is_some(),
                |query| {
                    query.and_where(
                        Expr::col((Products, products::Column::SellingPrice))
                            .lte(args.selling_price_max),
                    );
                },
                |_| {},
            )
            .conditions(
                args.stock_status.as_deref() == Some("out"),
                |query| {
                    query.and_where(product_inventory_expr().lte(0.0));
                },
                |_| {},
            )
            .conditions(
                args.stock_status.as_deref() == Some("low"),
                |query| {
                    query.and_where(
                        product_inventory_expr().gt(0.0).and(
                            product_inventory_expr()
                                .lte(Expr::col((Products, products::Column::MinQuantity))),
                        ),
                    );
                },
                |_| {},
            )
            .conditions(
                args.stock_status.as_deref() == Some("healthy"),
                |query| {
                    query.and_where(
                        product_inventory_expr()
                            .gt(Expr::col((Products, products::Column::MinQuantity))),
                    );
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit);
        match args.sort.as_deref() {
            Some("name") => {
                query.order_by(
                    products::Column::Name,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("inventory") => {
                query.order_by_expr(
                    Expr::cust("inventory"),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("min_quantity") => {
                query.order_by(
                    products::Column::MinQuantity,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("purchase_price") => {
                query.order_by(
                    products::Column::PurchasePrice,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("selling_price") => {
                query.order_by(
                    products::Column::SellingPrice,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("created_at") => {
                query.order_by(
                    products::Column::CreatedAt,
                    requested_order(args.direction.as_deref()),
                );
            }
            _ => {
                query.order_by(products::Column::CreatedAt, Order::Desc);
            }
        }
        let (sql, values) = query.to_owned().build(SqliteQueryBuilder);

        let products = SelectProducts::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(ProductsResponse { count, products })
    }

    pub async fn search_products(db: &DbConn, search: String) -> Result<Vec<ProductSearch>, DbErr> {
        let products = Products::find()
            .select_only()
            .expr_as(Expr::col(products::Column::Name), "label")
            .expr_as(Expr::col(products::Column::Id), "value")
            .expr_as(Expr::col(products::Column::SellingPrice), "price")
            .filter(products::Column::IsDeleted.eq(false))
            .filter(products::Column::Name.like(format!("%{}%", search)))
            .into_model::<ProductSearch>()
            .all(db)
            .await?;
        Ok(products)
    }

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
        product_active.description = ActiveValue::Set(product.description);
        product_active.min_quantity = ActiveValue::Set(product.min_quantity);
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
}
