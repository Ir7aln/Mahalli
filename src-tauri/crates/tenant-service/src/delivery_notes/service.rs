use super::dto::*;
use crate::OrderStatus;
use sea_orm::{
    sea_query::{Alias, Cond, Expr, Func, Query, SqliteQueryBuilder},
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    clients,
    delivery_note_items::{self, ActiveModel as DeliveryNoteItemActiveModel},
    delivery_notes::{self, ActiveModel as DeliveryNoteActiveModel},
    order_items,
    orders::{self, ActiveModel as OrderActiveModel},
    prelude::*,
    products,
};

fn requested_order(direction: Option<&str>) -> Order {
    if matches!(direction, Some("asc")) {
        Order::Asc
    } else {
        Order::Desc
    }
}

fn delivery_note_search_condition(search: &str) -> Cond {
    let pattern = format!("%{}%", search);
    Cond::any()
        .add(Expr::col((Clients, clients::Column::FullName)).like(pattern.clone()))
        .add(Expr::col((DeliveryNotes, delivery_notes::Column::Identifier)).like(pattern.clone()))
        .add(Expr::col((Orders, orders::Column::Identifier)).like(pattern))
}

pub struct DeliveryNotesService;

impl DeliveryNotesService {
    pub async fn list_delivery_notes(
        db: &DbConn,
        args: ListDeliveryNotesArgs,
    ) -> Result<DeliveryNotesResponse, DbErr> {
        let count = DeliveryNotes::find()
            .join(JoinType::Join, delivery_notes::Relation::Clients.def())
            .join(JoinType::Join, delivery_notes::Relation::Orders.def())
            .filter(
                Cond::all()
                    .add(Expr::col((DeliveryNotes, delivery_notes::Column::IsDeleted)).eq(false))
                    .add(delivery_note_search_condition(&args.search)),
            )
            .apply_if(args.status.clone(), |query, v| {
                query.filter(Expr::col((DeliveryNotes, delivery_notes::Column::Status)).eq(v))
            })
            .apply_if(args.created_from.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', delivery_notes.created_at) >= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .apply_if(args.created_to.clone(), |query, v| {
                query.filter(Expr::cust_with_values(
                    "strftime('%Y-%m-%d', delivery_notes.created_at) <= strftime('%Y-%m-%d', ?)",
                    [v],
                ))
            })
            .count(db)
            .await?;

        let mut query = Query::select();
        query
            .from(DeliveryNotes)
            .exprs([
                Expr::col((DeliveryNotes, delivery_notes::Column::Id)),
                Expr::col((DeliveryNotes, delivery_notes::Column::Identifier)),
                Expr::col((DeliveryNotes, delivery_notes::Column::CreatedAt)),
                Expr::col((DeliveryNotes, delivery_notes::Column::ClientId)),
                Expr::col((DeliveryNotes, delivery_notes::Column::OrderId)),
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Email)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::Ice)),
                Expr::col((Clients, clients::Column::IfNumber)),
                Expr::col((Clients, clients::Column::Rc)),
                Expr::col((Clients, clients::Column::Patente)),
                Expr::col((DeliveryNotes, delivery_notes::Column::Status)),
            ])
            .expr_as(
                Expr::col((Orders, orders::Column::Identifier)),
                Alias::new("order_identifier"),
            )
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::count(Expr::col((
                        DeliveryNoteItems,
                        delivery_note_items::Column::Id,
                    )))),
                    Expr::val(0i64),
                ]),
                Alias::new("products"),
            )
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((DeliveryNoteItems, delivery_note_items::Column::Quantity)).mul(
                            Expr::col((DeliveryNoteItems, delivery_note_items::Column::Price)),
                        ),
                    )),
                    Expr::val(0.0f64),
                ]),
                Alias::new("total"),
            )
            .join(
                JoinType::Join,
                Clients,
                Expr::col((Clients, clients::Column::Id))
                    .equals((DeliveryNotes, delivery_notes::Column::ClientId)),
            )
            .join(
                JoinType::Join,
                Orders,
                Expr::col((Orders, orders::Column::Id))
                    .equals((DeliveryNotes, delivery_notes::Column::OrderId)),
            )
            .left_join(
                DeliveryNoteItems,
                Expr::col((
                    DeliveryNoteItems,
                    delivery_note_items::Column::DeliveryNoteId,
                ))
                .equals((DeliveryNotes, delivery_notes::Column::Id)),
            )
            .cond_where(
                Cond::all()

                    .add(Expr::col((DeliveryNotes, delivery_notes::Column::IsDeleted)).eq(false))
                    .add(delivery_note_search_condition(&args.search)),
            )
            .conditions(
                args.status.clone().is_some(),
                |x| {
                    x.and_where(Expr::col((DeliveryNotes, delivery_notes::Column::Status)).eq(args.status.clone()));
                },
                |_| {},
            )
            .conditions(
                args.created_from.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', delivery_notes.created_at) >= strftime('%Y-%m-%d', ?)",
                        args.created_from,
                    ));
                },
                |_| {},
            )
            .conditions(
                args.created_to.clone().is_some(),
                |x| {
                    x.and_where(Expr::cust_with_values(
                        "strftime('%Y-%m-%d', delivery_notes.created_at) <= strftime('%Y-%m-%d', ?)",
                        args.created_to,
                    ));
                },
                |_| {},
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit)
            .group_by_col((DeliveryNotes, delivery_notes::Column::Id));

        if args.total_min.is_some() {
            query.and_having(
                Func::sum(
                    Expr::col((DeliveryNoteItems, delivery_note_items::Column::Price)).mul(
                        Expr::col((DeliveryNoteItems, delivery_note_items::Column::Quantity)),
                    ),
                )
                .gte(args.total_min.unwrap_or(0.0)),
            );
        }
        if args.total_max.is_some() {
            query.and_having(
                Func::sum(
                    Expr::col((DeliveryNoteItems, delivery_note_items::Column::Price)).mul(
                        Expr::col((DeliveryNoteItems, delivery_note_items::Column::Quantity)),
                    ),
                )
                .lte(args.total_max.unwrap_or(f64::MAX)),
            );
        }

        match args.sort.as_deref() {
            Some("identifier") => query.order_by(
                (DeliveryNotes, delivery_notes::Column::Identifier),
                requested_order(args.direction.as_deref()),
            ),
            Some("full_name") => query.order_by(
                (Clients, clients::Column::FullName),
                requested_order(args.direction.as_deref()),
            ),
            Some("status") => query.order_by(
                (DeliveryNotes, delivery_notes::Column::Status),
                requested_order(args.direction.as_deref()),
            ),
            Some("order_identifier") => query.order_by(
                (Orders, orders::Column::Identifier),
                requested_order(args.direction.as_deref()),
            ),
            Some("products") => query.order_by_expr(
                Expr::cust("products"),
                requested_order(args.direction.as_deref()),
            ),
            Some("total") => query.order_by_expr(
                Expr::cust("total"),
                requested_order(args.direction.as_deref()),
            ),
            Some("created_at") => query.order_by(
                (DeliveryNotes, delivery_notes::Column::CreatedAt),
                requested_order(args.direction.as_deref()),
            ),
            _ => query.order_by(
                (DeliveryNotes, delivery_notes::Column::CreatedAt),
                Order::Desc,
            ),
        };

        let (sql, values) = query.to_owned().build(SqliteQueryBuilder);
        let delivery_notes = SelectDeliveryNotes::find_by_statement(
            Statement::from_sql_and_values(DbBackend::Sqlite, sql, values),
        )
        .all(db)
        .await?;

        Ok(DeliveryNotesResponse {
            count,
            delivery_notes,
        })
    }

    pub async fn list_delivery_note_products(
        db: &DbConn,
        id: String,
    ) -> Result<Vec<DeliveryNoteProductItem>, DbErr> {
        let delivery_note = DeliveryNotes::find_by_id(&id).one(db).await?;
        if delivery_note.is_none() {
            return Err(DbErr::RecordNotFound("delivery note not found".to_string()));
        }

        DeliveryNoteItems::find()
            .select_only()
            .columns([
                delivery_note_items::Column::Price,
                delivery_note_items::Column::Quantity,
            ])
            .expr(Expr::col((Products, products::Column::Name)))
            .join(
                JoinType::Join,
                delivery_note_items::Relation::Products.def(),
            )
            .filter(delivery_note_items::Column::DeliveryNoteId.eq(id))
            .into_model::<DeliveryNoteProductItem>()
            .all(db)
            .await
    }

    pub async fn get_delivery_note(
        db: &DbConn,
        id: String,
    ) -> Result<DeliveryNoteDetailsResponse, DbErr> {
        let delivery_note = DeliveryNotes::find_by_id(&id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound("delivery note not found".to_string()))?;

        if delivery_note.is_deleted {
            return Err(DbErr::RecordNotFound("delivery note not found".to_string()));
        }

        let client = Clients::find_by_id(&delivery_note.client_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                DbErr::RecordNotFound("client for delivery note not found".to_string())
            })?;
        let order = Orders::find_by_id(&delivery_note.order_id)
            .one(db)
            .await?
            .ok_or_else(|| {
                DbErr::RecordNotFound("order for delivery note not found".to_string())
            })?;
        let items = DeliveryNoteItems::find()
            .filter(delivery_note_items::Column::DeliveryNoteId.eq(delivery_note.id.clone()))
            .all(db)
            .await?;
        let mut response_items = Vec::new();
        let mut total = 0.0f32;

        for item in items {
            let product = Products::find_by_id(&item.product_id)
                .one(db)
                .await?
                .ok_or_else(|| {
                    DbErr::RecordNotFound("product for delivery note not found".to_string())
                })?;
            total += item.quantity * item.price;
            response_items.push(DeliveryNoteProductDetailItem {
                product_id: item.product_id,
                name: product.name,
                price: item.price,
                quantity: item.quantity,
            });
        }

        Ok(DeliveryNoteDetailsResponse {
            id: delivery_note.id,
            created_at: delivery_note.created_at.to_string(),
            client_id: delivery_note.client_id,
            identifier: delivery_note.identifier,
            order_id: delivery_note.order_id,
            order_identifier: order.identifier,
            total,
            client: DeliveryNoteClientInfo {
                full_name: client.full_name,
                email: client.email,
                phone_number: client.phone_number,
                address: client.address,
                ice: client.ice,
                if_number: client.if_number,
                rc: client.rc,
                patente: client.patente,
            },
            items: response_items,
        })
    }

    pub async fn create_delivery_note_from_order(
        db: &DbConn,
        id: String,
    ) -> Result<String, TransactionError<DbErr>> {
        db.transaction::<_, String, DbErr>(|txn| {
            Box::pin(async move {
                if let Some(delivery_note) = DeliveryNotes::find()
                    .filter(delivery_notes::Column::OrderId.eq(&id))
                    .filter(delivery_notes::Column::IsDeleted.eq(false))
                    .one(txn)
                    .await?
                {
                    return Ok(delivery_note.id);
                }

                let order = Orders::find_by_id(&id).one(txn).await?.ok_or_else(|| {
                    DbErr::RecordNotFound("delivery note source order not found".to_string())
                })?;

                let delivery_note = DeliveryNoteActiveModel {
                    order_id: ActiveValue::Set(order.id.clone()),
                    client_id: ActiveValue::Set(order.client_id.clone()),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let order_items = OrderItems::find()
                    .join(
                        JoinType::Join,
                        order_items::Relation::InventoryTransactions.def(),
                    )
                    .filter(order_items::Column::OrderId.eq(order.id.clone()))
                    .all(txn)
                    .await?;

                let mut items = Vec::<DeliveryNoteItemActiveModel>::new();
                for item in order_items {
                    let inventory = InventoryTransactions::find_by_id(item.inventory_id)
                        .one(txn)
                        .await?
                        .ok_or_else(|| {
                            DbErr::RecordNotFound(
                                "delivery note inventory transaction missing".to_string(),
                            )
                        })?;

                    items.push(DeliveryNoteItemActiveModel {
                        delivery_note_id: ActiveValue::Set(delivery_note.id.clone()),
                        product_id: ActiveValue::Set(inventory.product_id),
                        price: ActiveValue::Set(item.price),
                        quantity: ActiveValue::Set(inventory.quantity),
                        inventory_id: ActiveValue::Set(Some(inventory.id)),
                        ..Default::default()
                    });
                }

                if !items.is_empty() {
                    DeliveryNoteItems::insert_many(items).exec(txn).await?;
                }

                let mut order_active: OrderActiveModel = order.into();
                order_active.status = ActiveValue::Set(OrderStatus::Completed.as_str().to_string());
                order_active.update(txn).await?;

                Ok(delivery_note.id)
            })
        })
        .await
    }
}
