use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name = app::schema::posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub published: &'a bool,
}
