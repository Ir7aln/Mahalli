use super::dto::*;
use sea_orm::{
    sea_query::{Alias, Cond, Expr, Func, Query, SqliteQueryBuilder},
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    clients::{self, Entity as Clients},
    inventory_transactions,
    products::{self, Entity as Products},
    quote_items::{self, ActiveModel as QuoteItemActiveModel, Entity as QuoteItems},
    quotes::{self, ActiveModel as QuoteActiveModel, Entity as Quotes},
};

fn requested_order(direction: Option<&str>) -> Order {
    if matches!(direction, Some("asc")) {
        Order::Asc
    } else {
        Order::Desc
    }
}

fn quote_search_condition(search: &str) -> Cond {
    let pattern = format!("%{}%", search);
    Cond::any()
        .add(Expr::col((Clients, clients::Column::FullName)).like(pattern.clone()))
        .add(Expr::col((Quotes, quotes::Column::Identifier)).like(pattern))
}

pub struct QuotesService;

impl QuotesService {
    pub async fn list_quotes(db: &DbConn, args: ListQuotesArgs) -> Result<QuotesResponse, DbErr> {
        let count = Quotes::find()
            .join(JoinType::Join, quotes::Relation::Clients.def())
            .filter(
                Cond::all()

                    .add(Expr::col((Quotes, quotes::Column::IsDeleted)).eq(false))
                    .add(quote_search_condition(&args.search)),
            )
            .apply_if(args.created_from.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', quotes.created_at) >= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .apply_if(args.created_to.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', quotes.created_at) <= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .count(db)
            .await?;

        let mut query = Query::select();
        query
            .from(Quotes)
            .exprs([
                Expr::col((Quotes, quotes::Column::Id)),
                Expr::col((Quotes, quotes::Column::CreatedAt)),
                Expr::col((Quotes, quotes::Column::ClientId)),
                Expr::col((Quotes, quotes::Column::Identifier)),
                Expr::col((Clients, clients::Column::FullName)),
            ])
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::count(Expr::col(
                        inventory_transactions::Column::Quantity,
                    ))),
                    Expr::val(0i64),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((QuoteItems, quote_items::Column::Quantity))
                            .mul(Expr::col((QuoteItems, quote_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ]),
                Alias::new("total"),
            )
            .left_join(
                QuoteItems,
                Expr::col((QuoteItems, quote_items::Column::QuoteId))
                    .equals((Quotes, quotes::Column::Id)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Quotes, quotes::Column::ClientId)),
            )
            .cond_where(
                Cond::all()

                    .add(Expr::col((Quotes, quotes::Column::IsDeleted)).eq(false))
                    .add(quote_search_condition(&args.search)),
            )
            .conditions(
                args.created_from.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', quotes.created_at) >= strftime('%Y-%m-%d', ?)",
                        args.created_from,
                    ));
                },
                |_| {},
            )
            .conditions(
                args.created_to.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', quotes.created_at) <= strftime('%Y-%m-%d', ?)",
                        args.created_to,
                    ));
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit);
        match args.sort.as_deref() {
            Some("identifier") => {
                query.order_by(
                    (Quotes, quotes::Column::Identifier),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("full_name") => {
                query.order_by(
                    (Clients, clients::Column::FullName),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("products") => {
                query.order_by_expr(
                    Expr::cust("products"),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("total") => {
                query.order_by_expr(
                    Expr::cust("total"),
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("created_at") => {
                query.order_by(
                    (Quotes, quotes::Column::CreatedAt),
                    requested_order(args.direction.as_deref()),
                );
            }
            _ => {
                query.order_by((Quotes, quotes::Column::CreatedAt), Order::Desc);
            }
        }
        query.group_by_col((Quotes, quotes::Column::Id));
        let (sql, values) = query.to_owned().build(SqliteQueryBuilder);

        let result = SelectQuotes::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(QuotesResponse {
            count,
            quotes: result,
        })
    }

    pub async fn get_quote(db: &DbConn, id: String) -> Result<QuoteWithClient, DbErr> {
        let quote = Quotes::find_by_id(id.clone())
            .find_also_related(Clients)
            .one(db)
            .await?;

        match quote {
            Some(quote) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((QuoteItems, quote_items::Column::Id)),
                        Expr::col((QuoteItems, quote_items::Column::Price)),
                        Expr::col((QuoteItems, quote_items::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .expr_as(
                        Expr::col((Products, products::Column::Id)),
                        Alias::new("product_id"),
                    )
                    .from(QuoteItems)
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((QuoteItems, quote_items::Column::ProductId)),
                    )
                    .cond_where(Expr::col((QuoteItems, quote_items::Column::QuoteId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectQuotesItemsForUpdate::find_by_statement(
                    Statement::from_sql_and_values(DbBackend::Sqlite, sql, values),
                )
                .all(db)
                .await?;

                let client = quote.1.unwrap();
                Ok(QuoteWithClient {
                    id: quote.0.id,
                    client_id: quote.0.client_id,
                    created_at: quote.0.created_at.to_string(),
                    identifier: quote.0.identifier,
                    full_name: client.full_name,
                    items,
                })
            }
            None => Err(DbErr::RecordNotFound(String::from("no quote"))),
        }
    }

    pub async fn list_quote_products(
        db: &DbConn,
        id: String,
    ) -> Result<Vec<QuoteProductItem>, DbErr> {
        let quote_products = QuoteItems::find()
            .select_only()
            .columns([quote_items::Column::Price])
            .exprs([
                Expr::col((Products, products::Column::Name)),
                Expr::col((QuoteItems, quote_items::Column::Quantity)),
            ])
            .join(JoinType::Join, quote_items::Relation::Products.def())
            .filter(Expr::col((QuoteItems, quote_items::Column::QuoteId)).eq(id))
            .into_model::<QuoteProductItem>()
            .all(db)
            .await?;

        Ok(quote_products)
    }

    pub async fn get_quote_details(db: &DbConn, id: String) -> Result<QuoteDetailsResponse, DbErr> {
        let (sql, values) = Query::select()
            .from(Quotes)
            .exprs([
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Email)),
                Expr::col((Quotes, quotes::Column::Id)),
                Expr::col((Quotes, quotes::Column::Identifier)),
                Expr::col((Quotes, quotes::Column::CreatedAt)),
            ])
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((QuoteItems, quote_items::Column::Quantity))
                            .mul(Expr::col((QuoteItems, quote_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ]),
                Alias::new("total"),
            )
            .left_join(
                QuoteItems,
                Expr::col((QuoteItems, quote_items::Column::QuoteId))
                    .equals((Quotes, quotes::Column::Id)),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((Quotes, quotes::Column::ClientId)),
            )
            .cond_where(Expr::col((Quotes, quotes::Column::Id)).eq(id.clone()))
            .to_owned()
            .build(SqliteQueryBuilder);

        let quote = SelectQuoteDetails::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .one(db)
        .await?;

        match quote {
            Some(quote) => {
                let (sql, values) = Query::select()
                    .exprs([
                        Expr::col((QuoteItems, quote_items::Column::Price)),
                        Expr::col((QuoteItems, quote_items::Column::Quantity)),
                        Expr::col((Products, products::Column::Name)),
                    ])
                    .from(QuoteItems)
                    .join(
                        JoinType::Join,
                        Products,
                        Expr::col((Products, products::Column::Id))
                            .equals((QuoteItems, quote_items::Column::ProductId)),
                    )
                    .cond_where(Expr::col((QuoteItems, quote_items::Column::QuoteId)).eq(id))
                    .to_owned()
                    .build(SqliteQueryBuilder);

                let items = SelectQuotesItems::find_by_statement(Statement::from_sql_and_values(
                    DbBackend::Sqlite,
                    sql,
                    values,
                ))
                .all(db)
                .await?;

                Ok(QuoteDetailsResponse {
                    id: quote.id,
                    created_at: quote.created_at,
                    identifier: quote.identifier,
                    total: quote.total,
                    client: QuoteClientInfo {
                        full_name: quote.full_name,
                        email: quote.email,
                        address: quote.address,
                        phone_number: quote.phone_number,
                    },
                    items,
                })
            }
            None => Err(DbErr::RecordNotFound(String::from("no quote"))),
        }
    }

    pub async fn create_quote(
        db: &DbConn,
        quote: NewQuote,
    ) -> Result<String, TransactionError<DbErr>> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                let created_quote = QuoteActiveModel {
                    client_id: ActiveValue::Set(quote.client_id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let mut items = Vec::<QuoteItemActiveModel>::new();

                for item in quote.items {
                    items.push(QuoteItemActiveModel {
                        quote_id: ActiveValue::Set(created_quote.id.clone()),
                        price: ActiveValue::Set(item.price),
                        quantity: ActiveValue::Set(item.quantity),
                        product_id: ActiveValue::Set(item.product_id),
                        ..Default::default()
                    })
                }
                if !items.is_empty() {
                    QuoteItems::insert_many(items).exec(txn).await?;
                }

                Ok(created_quote.id)
            })
        })
        .await
    }

    pub async fn update_quote(
        db: &DbConn,
        quote: UpdateQuote,
    ) -> Result<(), TransactionError<DbErr>> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                let quote_model = Quotes::find_by_id(quote.id.clone()).one(txn).await?;
                let mut quote_active: QuoteActiveModel = quote_model.unwrap().into();
                quote_active.client_id = ActiveValue::Set(quote.client_id);
                quote_active.update(txn).await?;

                let mut items = Vec::<QuoteItemActiveModel>::new();

                for item in quote.items {
                    match item.id {
                        Some(id) => {
                            let quote_item_model = QuoteItems::find_by_id(&id).one(txn).await?;
                            let mut quote_item_active: QuoteItemActiveModel =
                                quote_item_model.unwrap().into();
                            quote_item_active.product_id = ActiveValue::Set(item.product_id);
                            quote_item_active.price = ActiveValue::Set(item.price);
                            quote_item_active.quantity = ActiveValue::Set(item.quantity);
                            quote_item_active.update(txn).await?;
                        }
                        None => items.push(QuoteItemActiveModel {
                            quote_id: ActiveValue::Set(quote.id.clone()),
                            price: ActiveValue::Set(item.price),
                            quantity: ActiveValue::Set(item.quantity),
                            product_id: ActiveValue::Set(item.product_id),
                            ..Default::default()
                        }),
                    }
                }
                if !items.is_empty() {
                    QuoteItems::insert_many(items).exec(txn).await?;
                }
                Ok(())
            })
        })
        .await
    }

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
}
