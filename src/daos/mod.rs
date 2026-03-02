pub mod impldepartamento;
pub mod implfornecedor;
pub mod implfuncionario;
pub mod implproduto;

pub trait Verbos<T> {
    async fn inserir(&self, novo: T) -> Result<T, sqlx::Error>;
    async fn consultar(&self) -> Result<Vec<T>, sqlx::Error>;
    async fn editar(&self, editado: T) -> Result<T, sqlx::Error>;
    async fn excluir(&self, id: i32) -> Result<T, sqlx::Error>;
}