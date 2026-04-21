use super::dto::*;
use sea_orm::{
    sea_query::{
        Alias, Cond, Expr, Func, Query, SimpleExpr, SqliteQueryBuilder, SubQueryStatement,
    },
    DatabaseConnection as DbConn, *,
};
use tenant_entity::{
    clients::{self, ActiveModel as ClientActiveModel, Entity as Clients},
    invoice_items::{self, Entity as InvoiceItems},
    invoices::{self, Entity as Invoices},
};

fn requested_order(direction: Option<&str>) -> Order {
    if matches!(direction, Some("asc")) {
        Order::Asc
    } else {
        Order::Desc
    }
}

fn client_credit_expr() -> SimpleExpr {
    SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .from(Invoices)
                .expr(Expr::expr(Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ])))
                .left_join(
                    InvoiceItems,
                    Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                        .equals((Invoices, invoices::Column::Id)),
                )
                .cond_where(
                    Cond::all()
                        .add(Expr::col((Invoices, invoices::Column::Status)).is_not_in([
                            "CANCELLED",
                            "DRAFT",
                            "PAID",
                        ]))
                        .add(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                        .add(
                            Expr::col((Invoices, invoices::Column::ClientId))
                                .equals((Clients, clients::Column::Id)),
                        ),
                )
                .to_owned(),
        )),
    )
    .sub(SimpleExpr::SubQuery(
        None,
        Box::new(SubQueryStatement::SelectStatement(
            Query::select()
                .from(Invoices)
                .expr(Expr::expr(Func::coalesce([
                    Expr::expr(Func::sum(Expr::col((
                        Invoices,
                        invoices::Column::PaidAmount,
                    )))),
                    Expr::val(0.0f64),
                ])))
                .cond_where(
                    Cond::all()
                        .add(Expr::col((Invoices, invoices::Column::Status)).is_not_in([
                            "CANCELLED",
                            "DRAFT",
                            "PAID",
                        ]))
                        .add(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                        .add(
                            Expr::col((Invoices, invoices::Column::ClientId))
                                .equals((Clients, clients::Column::Id)),
                        ),
                )
                .to_owned(),
        )),
    ))
}

pub struct ClientsService;

impl ClientsService {
    pub async fn list_clients(
        db: &DbConn,
        args: ListClientsArgs,
    ) -> Result<ClientsResponse, DbErr> {
        let count = Clients::find()
            .filter(
                Cond::all()
                    .add(Expr::col((Clients, clients::Column::IsArchived)).eq(false))
                    .add(Expr::col((Clients, clients::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .count(db)
            .await?;

        let mut query = Query::select();
        query
            .from(Clients)
            .exprs([
                Expr::col((Clients, clients::Column::Id)),
                Expr::col((Clients, clients::Column::FullName)),
                Expr::col((Clients, clients::Column::Address)),
                Expr::col((Clients, clients::Column::PhoneNumber)),
                Expr::col((Clients, clients::Column::Image)),
                Expr::col((Clients, clients::Column::Email)),
            ])
            .expr_as(client_credit_expr(), Alias::new("credit"))
            .cond_where(
                Cond::all()
                    .add(Expr::col((Clients, clients::Column::IsArchived)).eq(false))
                    .add(Expr::col((Clients, clients::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Clients, clients::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit);
        match args.sort.as_deref() {
            Some("full_name") => {
                query.order_by(
                    clients::Column::FullName,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("email") => {
                query.order_by(
                    clients::Column::Email,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("phone_number") => {
                query.order_by(
                    clients::Column::PhoneNumber,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("address") => {
                query.order_by(
                    clients::Column::Address,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("credit") => {
                query.order_by_expr(
                    Expr::cust("credit"),
                    requested_order(args.direction.as_deref()),
                );
            }
            _ => {
                query.order_by(clients::Column::CreatedAt, Order::Desc);
            }
        }
        let (sql, values) = query.to_owned().build(SqliteQueryBuilder);

        let result = SelectClients::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(ClientsResponse {
            count,
            clients: result,
        })
    }

    pub async fn list_client_invoice_debts(
        db: &DbConn,
        client_id: String,
    ) -> Result<Vec<ClientInvoiceDebtItem>, DbErr> {
        let (sql, values) = Query::select()
            .from(Invoices)
            .exprs([
                Expr::col((Invoices, invoices::Column::Id)),
                Expr::col((Invoices, invoices::Column::Identifier)),
                Expr::col((Invoices, invoices::Column::PaidAmount)),
            ])
            .expr_as(
                Func::coalesce([
                    Expr::expr(Func::sum(
                        Expr::col((InvoiceItems, invoice_items::Column::Quantity))
                            .mul(Expr::col((InvoiceItems, invoice_items::Column::Price))),
                    )),
                    Expr::val(0.0f64),
                ]),
                Alias::new("total"),
            )
            .left_join(
                InvoiceItems,
                Expr::col((InvoiceItems, invoice_items::Column::InvoiceId))
                    .equals((Invoices, invoices::Column::Id)),
            )
            .cond_where(
                Cond::all()
                    .add(Expr::col((Invoices, invoices::Column::ClientId)).eq(client_id))
                    .add(Expr::col((Invoices, invoices::Column::IsDeleted)).eq(false))
                    .add(Expr::col((Invoices, invoices::Column::Status)).is_not_in([
                        "CANCELLED",
                        "DRAFT",
                        "PAID",
                    ])),
            )
            .group_by_col((Invoices, invoices::Column::Id))
            .order_by((Invoices, invoices::Column::CreatedAt), Order::Desc)
            .to_owned()
            .build(SqliteQueryBuilder);

        let debts = ClientInvoiceDebtItem::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(debts)
    }

    pub async fn search_clients(db: &DbConn, search: String) -> Result<Vec<ClientSearch>, DbErr> {
        let clients = Clients::find()
            .select_only()
            .expr_as(Expr::col(clients::Column::FullName), "label")
            .expr_as(Expr::col(clients::Column::Id), "value")
            .filter(clients::Column::IsDeleted.eq(false))
            .filter(clients::Column::FullName.like(format!("{}%", search)))
            .into_model::<ClientSearch>()
            .all(db)
            .await?;
        Ok(clients)
    }

    pub async fn get_client(db: &DbConn, id: String) -> Result<ClientDetails, DbErr> {
        let client = Clients::find_by_id(id).one(db).await?;

        match client {
            Some(client) => Ok(ClientDetails {
                id: client.id,
                full_name: client.full_name,
                email: client.email,
                phone_number: client.phone_number,
                address: client.address,
                image: client.image,
            }),
            None => Err(DbErr::RecordNotFound(String::from("no client"))),
        }
    }

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
}
