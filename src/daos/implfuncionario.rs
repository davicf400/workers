use crate::models::Funcionario;
use sqlx::PgPool;

pub struct FuncionarioConnect {
    pub pool: PgPool,
}

use crate:: daos:: Verbos;

impl Verbos<Funcionario> for FuncionarioConnect {
    
    async fn inserir(&self, new_fun: Funcionario) -> Result<Funcionario, sqlx::Error> {
        let funcionario_inserido = sqlx::query_as!(
            Funcionario,
            "INSERT INTO Funcionario (nome, cpf, cargo, departamento, salario)
            VALUES($1, $2, $3, $4, $5)
            RETURNING *",
            new_fun.nome,
            new_fun.cpf,
            new_fun.cargo,
            new_fun.departamento,
            new_fun.salario
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(funcionario_inserido)
    }

    async fn consultar(&self) -> Result<Vec<Funcionario>, sqlx::Error> {
        let consulta = sqlx::query_as!(
            Funcionario,
            "SELECT * FROM Funcionario"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(consulta)
    }      

    async fn editar(&self, fun_editado: Funcionario) -> Result<Funcionario, sqlx::Error> {
        let editado = sqlx::query_as!(
            Funcionario,
            "UPDATE Funcionario SET nome = $1, cpf = $2, cargo = $3, salario = $4, departamento = $5 WHERE id = $6
            RETURNING *",
            fun_editado.nome,
            fun_editado.cpf,
            fun_editado.cargo,
            fun_editado.salario,
            fun_editado.departamento,
            fun_editado.id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(editado)
    }

    async fn excluir(&self, id: i32) -> Result<Funcionario, sqlx::Error> {
        let excluido = sqlx::query_as!( 
            Funcionario,
            "DELETE FROM Funcionario WHERE id = $1
            RETURNING *",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(excluido)
    }
}