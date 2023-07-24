use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "detail")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i32,
    pub content: String,
}

// #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    // #[sea_orm(
    //     belongs_to = "super::problem::Entity",
    //     from = "Column::Id",
    //     to = "super::problem::Column::QuestionId",
    //     on_update = "NoAction",
    //     on_delete = "NoAction"
    // )]
    Problem,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Problem => Entity::belongs_to(super::problem::Entity)
                .from(Column::Id)
                .to(super::problem::Column::QuestionId)
                .on_update(ForeignKeyAction::NoAction)
                .on_delete(ForeignKeyAction::NoAction)
                .into(),
        }
    }
}

impl Related<super::problem::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Problem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
