use axum::{extract::{State, Path}, Json, http::StatusCode};


use crate::models::Funcionario;
use crate::daos::Verbos;
use crate::daos::implfuncionario::FuncionarioConnect;
use crate::AppState; 

pub async fn criar_funcionario(
    State(estado): State<AppState>, 
    Json(new_func): Json<Funcionario>
) -> Result<(StatusCode, Json<Funcionario>), StatusCode> { 
    let dao = FuncionarioConnect { pool: estado.pool };
    let criado = dao.inserir(new_func)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::CREATED, Json(criado)))
}

pub async fn excluir_funcionario(
    State(estado): State<AppState>, 
    Path(id): Path<i32> 
) -> Result<(StatusCode, Json<Funcionario>), StatusCode> {
    let dao = FuncionarioConnect { pool: estado.pool };
    let excluido = dao.excluir(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(excluido)))
}

pub async fn editar_funcionario(
    State(estado): State<AppState>,
    Json(editado): Json<Funcionario>
) -> Result<(StatusCode, Json<Funcionario>), StatusCode> {
    let dao = FuncionarioConnect { pool: estado.pool }; 
    let edited = dao.editar(editado)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(edited))) 
}


pub async fn consultar_funcionario(
    State(estado): State<AppState>,

) -> Result<(StatusCode, Json<Vec<Funcionario>>), StatusCode> { 
    let dao = FuncionarioConnect { pool: estado.pool }; 
    let consulta = dao.consultar()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(consulta))) 
}