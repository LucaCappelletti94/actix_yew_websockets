#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::*;
use diesel::prelude::*;

#[derive(Queryable, Debug)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub body: String,
}

impl From<commons::comments::Comment> for Comment {
    fn from(comment: commons::comments::Comment) -> Self {
        Self {
            id: comment.id,
            user_id: comment.user_id,
            body: comment.body,
        }
    }
}

impl Comment {
    pub fn delete(&self, conn: &mut PgConnection) -> QueryResult<usize> {
        use crate::schema::comments::dsl::*;
        diesel::delete(comments.filter(id.eq(self.id))).execute(conn)
    }
}

#[derive(Queryable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
}

impl Into<commons::users::User> for User {
    fn into(self) -> commons::users::User {
        commons::users::User {
            id: self.id,
            username: self.username,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub user_id: i32,
    pub body: String,
}

impl NewComment {
    pub fn new(user_id: i32, body: &str) -> Self {
        Self {
            user_id,
            body: body.to_string(),
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> QueryResult<Comment> {
        use crate::schema::comments::dsl::*;
        diesel::insert_into(comments)
            .values(self)
            .get_result::<Comment>(conn)
    }
}

impl Into<commons::comments::Comment> for Comment {
    fn into(self) -> commons::comments::Comment {
        commons::comments::Comment {
            id: self.id,
            user_id: self.user_id,
            body: self.body,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
}

impl NewUser {
    pub fn new(username: &str) -> Self {
        Self {
            username: username.to_string(),
        }
    }

    pub fn insert(&self, conn: &mut PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        diesel::insert_into(users)
            .values(self)
            .get_result::<User>(conn)
    }

    pub fn insert_or_get(&self, conn: &mut PgConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        match users.filter(username.eq(&self.username)).first::<User>(conn) {
            Ok(user) => Ok(user),
            Err(_) => self.insert(conn),
        }
    }
}
