use axum::{extract::{State, Path}, Json, http::StatusCode};

use crate::models::Departamento;
use crate::daos::Verbos;
use crate::daos::impldepartamento::DepartamentoConnect;
use crate::AppState; 

pub async fn criar_departamento(
    State(estado): State<AppState>, 
    Json(novo): Json<Departamento>
) -> Result<(StatusCode, Json<Departamento>), StatusCode> { 
    let dao = DepartamentoConnect { pool: estado.pool };
    let criado = dao.inserir(novo)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::CREATED, Json(criado)))
}

pub async fn excluir_departamento(
    State(estado): State<AppState>, 
    Path(id): Path<i32> 
) -> Result<(StatusCode, Json<Departamento>), StatusCode> {
    let dao = DepartamentoConnect { pool: estado.pool };
    let excluido = dao.excluir(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(excluido)))
}

pub async fn editar_departamento(
    State(estado): State<AppState>,
    Json(editado): Json<Departamento>
) -> Result<(StatusCode, Json<Departamento>), StatusCode> {
    let dao = DepartamentoConnect { pool: estado.pool }; 
    let edited = dao.editar(editado)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(edited))) 
}

pub async fn consultar_departamentos(
    State(estado): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Departamento>>), StatusCode> { 
    let dao = DepartamentoConnect { pool: estado.pool }; 
    let consulta = dao.consultar()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(consulta))) 
}