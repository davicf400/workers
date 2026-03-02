use crate::models::Departamento;
use sqlx::PgPool;
use crate::daos::Verbos; 

pub struct DepartamentoConnect {
    pub pool: PgPool
}

impl Verbos<Departamento> for DepartamentoConnect {
    
    async fn inserir(&self, novo: Departamento) -> Result<Departamento, sqlx::Error> {
        let newdp = sqlx::query_as!(
            Departamento,
            
            "INSERT INTO Departamento (nome) VALUES ($1) RETURNING *", 
            novo.nome
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(newdp)
        
    }

    async fn consultar(&self) -> Result<Vec<Departamento>, sqlx::Error> {
        let consulta = sqlx::query_as!(
            Departamento,
            "SELECT * FROM Departamento"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(consulta) 
    }

    async fn editar(&self, editado: Departamento) -> Result<Departamento, sqlx::Error> {
        let edited = sqlx::query_as!(
            Departamento,
            
            "UPDATE Departamento SET nome = $1 WHERE id = $2 RETURNING *", 
            editado.nome,
            editado.id 
        )   
        .fetch_one(&self.pool)
        .await?;
        
        Ok(edited) 
    }

    async fn excluir(&self, id: i32) -> Result<Departamento, sqlx::Error> {
        let excluido = sqlx::query_as!(
            Departamento,
            "DELETE FROM Departamento WHERE id = $1 RETURNING *", 
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(excluido) 
    }
} 