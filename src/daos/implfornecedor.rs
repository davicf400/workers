use crate::models::Fornecedor;
use sqlx::PgPool;
use crate::daos::Verbos;



pub struct FornecedorConnect {
    pub pool: PgPool
}

impl Verbos <Fornecedor> for FornecedorConnect{
      async fn inserir(&self, novo: Fornecedor) -> Result<Fornecedor, sqlx::Error>{
        let inserido = sqlx::query_as!(
            Fornecedor,
            "INSERT INTO Fornecedor (razao_social, cnpj, telefone, email)
            VALUES ($1, $2, $3, $4)
            RETURNING* ",
            novo.razao_social,
            novo.cnpj,
            novo.telefone,
            novo.email
        )
        .fetch_one(&self.pool)
        .await?;
    Ok(inserido)

      }

    async fn consultar(&self) -> Result<Vec<Fornecedor>, sqlx::Error>{
        let consulta = sqlx::query_as!(
            Fornecedor,
            "SELECT *FROM Fornecedor"
        )
        .fetch_all(&self.pool)
        .await?;
    Ok(consulta)
        
    }   

       async fn editar(&self, editado: Fornecedor) -> Result<Fornecedor, sqlx::Error>{
        let edited = sqlx::query_as!(
            Fornecedor,
             "UPDATE Fornecedor SET razao_social = $1, cnpj = $2, telefone = $3, email = $4 WHERE id = $5
            RETURNING *",
            editado.razao_social,
            editado.cnpj,
            editado.telefone,
            editado.email,
            editado.id

        )
        .fetch_one(&self.pool)
        .await?;
        Ok(edited)

       }

 async fn excluir(&self, id: i32) -> Result<Fornecedor, sqlx::Error> {
        let excluido = sqlx::query_as!( 
            Fornecedor,
            "DELETE FROM Fornecedor WHERE id = $1
            RETURNING *",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(excluido)
    }

}