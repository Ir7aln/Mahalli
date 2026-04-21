use super::dto::*;
use sea_orm::{
    sea_query::{Cond, Expr, Query, SqliteQueryBuilder},
    DatabaseConnection as DbConn, *,
};
use tenant_entity::suppliers::{self, ActiveModel as SupplierActiveModel, Entity as Suppliers};

fn requested_order(direction: Option<&str>) -> Order {
    if matches!(direction, Some("asc")) {
        Order::Asc
    } else {
        Order::Desc
    }
}

pub struct SuppliersService;

impl SuppliersService {
    pub async fn list_suppliers(
        db: &DbConn,
        args: ListSuppliersArgs,
    ) -> Result<SuppliersResponse, DbErr> {
        let count = Suppliers::find()
            .filter(
                Cond::all()
                    .add(Expr::col((Suppliers, suppliers::Column::IsArchived)).eq(false))
                    .add(Expr::col((Suppliers, suppliers::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Suppliers, suppliers::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .count(db)
            .await?;

        let mut query = Query::select();
        query
            .from(Suppliers)
            .exprs([
                Expr::col((Suppliers, suppliers::Column::Id)),
                Expr::col((Suppliers, suppliers::Column::FullName)),
                Expr::col((Suppliers, suppliers::Column::Address)),
                Expr::col((Suppliers, suppliers::Column::PhoneNumber)),
                Expr::col((Suppliers, suppliers::Column::Image)),
                Expr::col((Suppliers, suppliers::Column::Email)),
            ])
            .cond_where(
                Cond::all()
                    .add(Expr::col((Suppliers, suppliers::Column::IsArchived)).eq(false))
                    .add(Expr::col((Suppliers, suppliers::Column::IsDeleted)).eq(false))
                    .add(
                        Expr::col((Suppliers, suppliers::Column::FullName))
                            .like(format!("{}%", args.search)),
                    ),
            )
            .limit(args.limit)
            .offset((args.page - 1) * args.limit);
        match args.sort.as_deref() {
            Some("full_name") => {
                query.order_by(
                    suppliers::Column::FullName,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("email") => {
                query.order_by(
                    suppliers::Column::Email,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("phone_number") => {
                query.order_by(
                    suppliers::Column::PhoneNumber,
                    requested_order(args.direction.as_deref()),
                );
            }
            Some("address") => {
                query.order_by(
                    suppliers::Column::Address,
                    requested_order(args.direction.as_deref()),
                );
            }
            _ => {
                query.order_by(suppliers::Column::CreatedAt, Order::Desc);
            }
        }
        let (sql, values) = query.to_owned().build(SqliteQueryBuilder);

        let result = SelectSuppliers::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            values,
        ))
        .all(db)
        .await?;

        Ok(SuppliersResponse {
            count,
            suppliers: result,
        })
    }

    pub async fn search_suppliers(
        db: &DbConn,
        search: String,
    ) -> Result<Vec<SupplierSearch>, DbErr> {
        let suppliers = Suppliers::find()
            .select_only()
            .expr_as(Expr::col(suppliers::Column::FullName), "label")
            .expr_as(Expr::col(suppliers::Column::Id), "value")
            .filter(suppliers::Column::IsDeleted.eq(false))
            .filter(suppliers::Column::FullName.like(format!("{}%", search)))
            .into_model::<SupplierSearch>()
            .all(db)
            .await?;
        Ok(suppliers)
    }

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
}
