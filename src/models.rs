use serde::{Deserialize, Serialize,};

#[derive(Serialize, Deserialize, Debug)]
pub struct Funcionario{
    #[serde(default)]
   pub id: i32,
 pub  nome: String,
pub cpf: String,
pub cargo: String,
 pub   salario: f32,
pub departamento: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Departamento{
  #[serde(default)]
pub id:i32,
pub nome: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Fornecedor{
     #[serde(default)]
   pub id: i32,
    pub razao_social: String,
    pub cnpj: String,
    pub telefone: String,
    pub email: String

}
#[derive(Serialize, Deserialize, Debug)]
pub struct Produto{
     #[serde(default)]
   pub  id:i32,
    pub nome: String,
    pub estoque: i32
}

