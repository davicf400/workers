use crate::models::Produto;
use sqlx::PgPool;
use crate::daos::Verbos;

pub struct ProdutoConnect {
    pub pool: PgPool
}

impl Verbos<Produto> for ProdutoConnect {
    
    async fn inserir(&self, novo: Produto) -> Result<Produto, sqlx::Error> {
        let inserido = sqlx::query_as!(
            Produto,
            
            "INSERT INTO Produto (nome, estoque) VALUES($1, $2) RETURNING *",
            novo.nome,
            novo.estoque
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(inserido)
    }

    async fn consultar(&self) -> Result<Vec<Produto>, sqlx::Error> {
        let consulta = sqlx::query_as!(
            Produto,
            "SELECT * FROM Produto"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(consulta)
    }      

    async fn editar(&self, edited: Produto) -> Result<Produto, sqlx::Error> {
        let editado = sqlx::query_as!(
            Produto,
            
            "UPDATE Produto SET nome = $1, estoque = $2 WHERE id = $3 RETURNING *",
            edited.nome,
            edited.estoque,
            edited.id 
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(editado)
    }

    async fn excluir(&self, id: i32) -> Result<Produto, sqlx::Error> {
        let excluido = sqlx::query_as!( 
            Produto,
            "DELETE FROM Produto WHERE id = $1 RETURNING *",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(excluido)
    }
}