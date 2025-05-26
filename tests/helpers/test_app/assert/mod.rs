use sea_orm::{EntityTrait, PrimaryKeyTrait, Select};

use crate::helpers::TestApp;

impl TestApp {
    pub async fn find_exactly_one<E: EntityTrait>(&self, select: Select<E>) -> E::Model
    where
        <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let rows = select.all(&self.db).await.unwrap();
        assert!(rows.len().eq(&1));
        rows.into_iter().next().unwrap()
    }

    pub async fn assert_found_none<E: EntityTrait>(&self, select: Select<E>) {
        let rows = select.all(&self.db).await.unwrap();
        assert!(rows.len().eq(&0));
    }
}
