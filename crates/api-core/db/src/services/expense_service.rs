use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::models::{Expense, Tag, User};

pub struct ExpenseService {
    db: PgPool,
}

impl ExpenseService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn get_all(&self, user_id: i32) -> Result<Vec<Expense>, sqlx::Error> {
        let expenses = sqlx::query!(
            "SELECT expense_id, user_id, amount, description, expense_time FROM expenses WHERE user_id = $1",
            user_id 
        ) 
        .fetch_all(&self.db) 
        .await?;

        let mut tags_vec: Vec<Vec<Tag>> = Vec::from(Vec::new());

        for expense in &expenses {
            let tags = sqlx::query!(
                "SELECT tag_id, tagname FROM tags WHERE tag_id IN (SELECT tag_id FROM expenses_tags WHERE expense_id = $1)",
                expense.expense_id
            ).fetch_all(&self.db).await?.iter().map(|e| {
                Tag {
                    tag_id: e.tag_id,
                    tagname: e.tagname.clone(),
                }
            }).collect::<Vec<Tag>>();
            tags_vec.push(tags)
        }

        let user = sqlx::query!(
            "SELECT user_id, display_name, email, creation_time FROM users WHERE user_id = $1",
            user_id
        )
        .fetch_one(&self.db)
        .await?;

        let user = User {
            user_id: user.user_id,
            display_name: user.display_name,
            email: user.email,
            password: None,
            creation_time: user.creation_time,
        };

        let expenses: Vec<Expense> = expenses
            .iter()
            .enumerate()
            .map(move |(i, e)| Expense {
                expense_id: e.expense_id,
                user: user.clone(),
                amount: e.amount,
                description: e.description.clone(),
                expense_time: e.expense_time,
                tags: tags_vec[i].clone(),
            })
            .collect();

        Ok(expenses)
    }

    pub async fn get_by_id(&self, expense_id: i32) -> Result<Expense, sqlx::Error> {
        let expense = sqlx::query!(
            "SELECT expense_id, user_id, amount, description, expense_time FROM expenses WHERE expense_id = $1",
            expense_id
        ).fetch_one(&self.db).await?;

        let tags = sqlx::query!(
            "SELECT tag_id, tagname FROM tags WHERE tag_id IN (SELECT tag_id FROM expenses_tags WHERE expense_id = $1)",
            expense_id
        ).fetch_all(&self.db).await?.iter().map(|e| {
            Tag {
                tag_id: e.tag_id,
                tagname: e.tagname.clone(),
            }
        }).collect::<Vec<Tag>>();

        let user = sqlx::query!(
            "SELECT user_id, display_name, email, creation_time FROM users WHERE user_id = $1",
            expense.user_id
        )
        .fetch_one(&self.db)
        .await?;

        let user = User {
            user_id: user.user_id,
            display_name: user.display_name,
            email: user.email,
            password: None,
            creation_time: user.creation_time,
        };

        let expense = Expense {
            expense_id: expense.expense_id,
            user: user,
            amount: expense.amount,
            description: expense.description,
            expense_time: expense.expense_time,
            tags: tags,
        };

        Ok(expense)
    }

    /// Same as get_by_id, but checks if expense belongs to user
    pub async fn get_by_id_checked(
        &self,
        expense_id: i32,
        user_id: i32,
    ) -> Result<Option<Expense>, sqlx::Error> {
        // check if expense belongs to user
        let expense = sqlx::query!(
            "SELECT expense_id, user_id FROM expenses WHERE expense_id = $1 AND user_id = $2",
            expense_id,
            user_id
        )
        .fetch_optional(&self.db)
        .await?;

        if expense.is_none() {
            return Ok(None);
        }

        self.get_by_id(expense_id).await.map(|e| Some(e))
    }

    pub async fn delete_by_id_checked(
        &self,
        expense_id: i32,
        user_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM expenses WHERE expense_id = $1 AND user_id = $2",
            expense_id,
            user_id
        )
        .execute(&self.db)
        .await?;
        Ok(())
    }

    // TODO: Create a create and update function
    pub async fn create(&self, expense: CreateExpense) -> Result<i32, sqlx::Error> {
        // Create expense
        let new_expense = sqlx::query!(
            "INSERT INTO expenses (user_id, amount, description) VALUES ($1, $2, $3) RETURNING expense_id",
            expense.user_id,
            expense.amount,
            expense.description
        ).fetch_one(&self.db).await?;

        // Create tags
        for tag_id in expense.tags {
            sqlx::query!(
                "INSERT INTO expenses_tags (expense_id, tag_id) VALUES ($1, $2)",
                new_expense.expense_id,
                tag_id
            )
            .execute(&self.db)
            .await?;
        }

        Ok(new_expense.expense_id)
    }

    pub async fn update_by_id_checked(
        &self,
        expense: CreateExpense,
        expense_id: i32,
        user_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE expenses SET amount = $1, description = $2 WHERE expense_id = $3 AND user_id = $4",
            expense.amount,
            expense.description,
            expense_id,
            user_id
        ).execute(&self.db).await?;

        // Delete all tags
        sqlx::query!(
            "DELETE FROM expenses_tags WHERE expense_id = $1",
            expense_id
        )
        .execute(&self.db)
        .await?;

        // Create tags
        for tag_id in expense.tags {
            sqlx::query!(
                "INSERT INTO expenses_tags (expense_id, tag_id) VALUES ($1, $2)",
                expense_id,
                tag_id
            )
            .execute(&self.db)
            .await?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExpense {
    pub user_id: i32,
    pub amount: Decimal,
    pub description: String,
    pub tags: Vec<i32>,
}
