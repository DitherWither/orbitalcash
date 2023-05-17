use sqlx::PgPool;

use crate::models::Tag;

pub struct TagService {
    db: PgPool,
}

impl TagService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_all(&self, user_id: i32) -> Result<Vec<Tag>, sqlx::Error> {
        let tags = sqlx::query!(
            "SELECT tag_id, tagname FROM tags WHERE user_id = $1",
            user_id
        )
        .fetch_all(&self.db)
        .await?;

        Ok(tags
            .iter()
            .map(|e| Tag {
                tag_id: e.tag_id,
                tagname: e.tagname.clone(),
            })
            .collect::<Vec<Tag>>())
    }

    pub async fn get_by_id(&self, tag_id: i32) -> Result<Tag, sqlx::Error> {
        let tag = sqlx::query!("SELECT tag_id, tagname FROM tags WHERE tag_id = $1", tag_id)
            .fetch_one(&self.db)
            .await?;

        Ok(Tag {
            tag_id: tag.tag_id,
            tagname: tag.tagname.clone(),
        })
    }

    pub async fn get_by_id_checked(
        &self,
        tag_id: i32,
        user_id: i32,
    ) -> Result<Option<Tag>, sqlx::Error> {
        let tag = sqlx::query!(
            "SELECT tag_id, tagname FROM tags WHERE tag_id = $1 AND user_id = $2",
            tag_id,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        match tag {
            Some(tag) => Ok(Some(Tag {
                tag_id: tag.tag_id,
                tagname: tag.tagname.clone(),
            })),
            None => Ok(None),
        }
    }

    pub async fn create(&self, tagname: String, user_id: i32) -> Result<Tag, sqlx::Error> {
        let tag = sqlx::query!(
            "INSERT INTO tags (tagname, user_id) VALUES ($1, $2) RETURNING tag_id, tagname",
            tagname,
            user_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Tag {
            tag_id: tag.tag_id,
            tagname: tag.tagname.clone(),
        })
    }

    pub async fn update(
        &self,
        tag: Tag,
        user_id: i32,
    ) -> Result<Tag, sqlx::Error> {
        let tag = sqlx::query!(
            "UPDATE tags SET tagname = $1 WHERE tag_id = $2 AND user_id = $3 RETURNING tag_id, tagname",
            tag.tagname, 
            tag.tag_id,
            user_id
        )
        .fetch_one(&self.db)
        .await?;

        Ok(Tag {
            tag_id: tag.tag_id,
            tagname: tag.tagname.clone(),
        })
    }

    pub async fn delete_by_id_checked(&self, tag_id: i32, user_id: i32) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM tags WHERE tag_id = $1 AND user_id = $2",
            tag_id,
            user_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }
}
