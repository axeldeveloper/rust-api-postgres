use tokio_postgres::{Error, GenericClient, Row};



#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateUserSchema {
    pub login: String,
    //pub content: String,
   
}


#[derive(Debug, serde::Serialize)]
pub struct User {
    pub id: i32,
    pub login: String,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            login: row.get(1),
        }
    }
}

impl User {
    
    // Função para retornar registro.
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<User>, Error> {
        let stmt = client.prepare("SELECT id, login FROM users").await?;
        let rows = client.query(&stmt, &[]).await?;

        Ok(rows.into_iter().map(User::from).collect())
    }

    // Função para incluir um registro.
    pub async fn create<C: GenericClient>(client: &C, d: CreateUserSchema) -> Result<u64, Error> {
        let stmt = match client.prepare("INSERT INTO users (login) VALUES ($1) RETURNING *").await {
            Ok(stmt) => stmt,
            Err(e) => return Err(e), // Retorna o erro imediatamente se a preparação da consulta falhar.
        };
    
        let rows = match client.execute(&stmt, &[&d.login]).await {
            Ok(rows) => rows,
            Err(e) => return Err(e), // Retorna o erro imediatamente se a execução da consulta falhar.
        };
    
        Ok(rows)
    }

    // Função para excluir um registro.
    pub async fn delete<C: GenericClient> (client: &C, id: i32) -> Result<u64,Error> {
        let stmt = client.prepare("DELETE FROM users WHERE id = $1").await?;   
        let rows = match client.execute(&stmt, &[&id]).await {
            Ok(rows) => rows,
            Err(e) => return Err(e), // Retorna o erro imediatamente se a execução da consulta falhar.
        };
        
        Ok(rows)
    }

    // Função para atualizar um registro e retornar a estrutura User atualizada.
    pub async fn update<C: GenericClient>(client: &C, d: User) -> Result<User, Error> {     
        let stmt = client.prepare("UPDATE users SET login = $2 WHERE id = $1 RETURNING id, login ").await?;
        let row = client.query_one(&stmt, &[&d.id, &d.login]).await?;
        let user = User {
            id: row.get("id"),
            login: row.get("login"),
        };   
        Ok(user) 
    }


}
