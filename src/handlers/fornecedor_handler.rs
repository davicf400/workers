use axum::{extract::{State, Path}, Json, http::StatusCode};

use crate::models::Fornecedor;
use crate::daos::Verbos;
use crate::daos::implfornecedor::FornecedorConnect;
use crate::AppState; 

pub async fn criar_fornecedor(
    State(estado): State<AppState>, 
    Json(novo): Json<Fornecedor>
) -> Result<(StatusCode, Json<Fornecedor>), StatusCode> { 
    let dao:FornecedorConnect = FornecedorConnect { pool: estado.pool };
    let criado: Fornecedor = dao.inserir(novo)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::CREATED, Json(criado)))
}

pub async fn excluir_fornecedor(
    State(estado): State<AppState>, 
    Path(id): Path<i32> 
) -> Result<(StatusCode, Json<Fornecedor>), StatusCode> {
    let dao = FornecedorConnect { pool: estado.pool };
    let excluido = dao.excluir(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(excluido)))
}

pub async fn editar_fornecedor(
    State(estado): State<AppState>,
    Json(editado): Json<Fornecedor>
) -> Result<(StatusCode, Json<Fornecedor>), StatusCode> {
    let dao = FornecedorConnect { pool: estado.pool }; 
    let edited = dao.editar(editado)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(edited))) 
}

pub async fn consultar_fornecedores(
    State(estado): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Fornecedor>>), StatusCode> { 
    let dao = FornecedorConnect { pool: estado.pool }; 
    let consulta = dao.consultar()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(consulta))) 
}