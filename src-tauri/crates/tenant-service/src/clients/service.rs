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
    invoice_payments::{self, Entity as InvoicePayments},
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
                .from(InvoicePayments)
                .expr(Expr::expr(Func::coalesce([
                    Expr::expr(Func::sum(Expr::col((
                        InvoicePayments,
                        invoice_payments::Column::Amount,
                    )))),
                    Expr::val(0.0f64),
                ])))
                .join(
                    JoinType::Join,
                    Invoices,
                    Expr::col((InvoicePayments, invoice_payments::Column::InvoiceId))
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
    ))
}

fn client_search_condition(search: &str, search_field: Option<&str>) -> Cond {
    let pattern = format!("%{}%", search);
    match search_field {
        Some("email") => Cond::any().add(Expr::col((Clients, clients::Column::Email)).like(pattern)),
        Some("phone_number") => Cond::any().add(Expr::col((Clients, clients::Column::PhoneNumber)).like(pattern)),
        Some("address") => Cond::any().add(Expr::col((Clients, clients::Column::Address)).like(pattern)),
        Some("ice") => Cond::any().add(Expr::col((Clients, clients::Column::Ice)).like(pattern)),
        Some("if_number") => Cond::any().add(Expr::col((Clients, clients::Column::IfNumber)).like(pattern)),
        Some("rc") => Cond::any().add(Expr::col((Clients, clients::Column::Rc)).like(pattern)),
        Some("patente") => Cond::any().add(Expr::col((Clients, clients::Column::Patente)).like(pattern)),
        _ => {
            Cond::any()
                .add(Expr::col((Clients, clients::Column::FullName)).like(pattern))
        }
    }
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
                    .add(client_search_condition(&args.search, args.search_field.as_deref())),
            )
            .apply_if(args.credit_only, |query, credit_only| {
                query.filter(client_credit_expr().gt(if credit_only { 0.0 } else { -1.0 }))
            })
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
                Expr::col((Clients, clients::Column::Ice)),
                Expr::col((Clients, clients::Column::IfNumber)),
                Expr::col((Clients, clients::Column::Rc)),
                Expr::col((Clients, clients::Column::Patente)),
            ])
            .expr_as(client_credit_expr(), Alias::new("credit"))
            .cond_where(
                Cond::all()
                    .add(Expr::col((Clients, clients::Column::IsArchived)).eq(false))
                    .add(Expr::col((Clients, clients::Column::IsDeleted)).eq(false))
                    .add(client_search_condition(&args.search, args.search_field.as_deref())),
            )
            .conditions(
                args.credit_only == Some(true),
                |query| {
                    query.and_where(client_credit_expr().gt(0.0));
                },
                |_| {},
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
            Some("ice") => {
                query.order_by(
                    clients::Column::Ice,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("if_number") => {
                query.order_by(
                    clients::Column::IfNumber,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("rc") => {
                query.order_by(
                    clients::Column::Rc,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("patente") => {
                query.order_by(
                    clients::Column::Patente,
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
            .expr_as(
                SimpleExpr::SubQuery(
                    None,
                    Box::new(SubQueryStatement::SelectStatement(
                        Query::select()
                            .expr(Expr::expr(Func::coalesce([
                                Expr::expr(Func::sum(Expr::col((
                                    InvoicePayments,
                                    invoice_payments::Column::Amount,
                                )))),
                                Expr::val(0.0f64),
                            ])))
                            .from(InvoicePayments)
                            .cond_where(
                                Expr::col((InvoicePayments, invoice_payments::Column::InvoiceId))
                                    .equals((Invoices, invoices::Column::Id)),
                            )
                            .to_owned(),
                    )),
                ),
                Alias::new("paid_amount"),
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
            .filter(clients::Column::FullName.like(format!("%{}%", search)))
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
                ice: client.ice,
                if_number: client.if_number,
                rc: client.rc,
                patente: client.patente,
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
            ice: ActiveValue::Set(client.ice),
            if_number: ActiveValue::Set(client.if_number),
            rc: ActiveValue::Set(client.rc),
            patente: ActiveValue::Set(client.patente),
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
        client_active.ice = ActiveValue::Set(client.ice);
        client_active.if_number = ActiveValue::Set(client.if_number);
        client_active.rc = ActiveValue::Set(client.rc);
        client_active.patente = ActiveValue::Set(client.patente);
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
