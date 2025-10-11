#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250111_120000_add_admin_fields_to_users;
mod m20250111_120001_create_user_groups;
mod m20250111_120002_create_site_settings;
mod m20251006_163609_books;
mod m20251006_172232_chapters;
mod m20251006_172318_medias;
mod m20251008_120000_add_chapter_tree_fields;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20251006_163609_books::Migration),
            Box::new(m20251006_172232_chapters::Migration),
            Box::new(m20251006_172318_medias::Migration),
            Box::new(m20251008_120000_add_chapter_tree_fields::Migration),
            Box::new(m20250111_120000_add_admin_fields_to_users::Migration),
            Box::new(m20250111_120001_create_user_groups::Migration),
            Box::new(m20250111_120002_create_site_settings::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
