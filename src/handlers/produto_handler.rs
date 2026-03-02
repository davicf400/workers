use axum::{extract::{State, Path}, Json, http::StatusCode};

use crate::models::Produto;
use crate::daos::Verbos;
use crate::daos::implproduto::ProdutoConnect;
use crate::AppState; 

pub async fn criar_produto(
    State(estado): State<AppState>, 
    Json(novo): Json<Produto>
) -> Result<(StatusCode, Json<Produto>), StatusCode> { 
    let dao = ProdutoConnect { pool: estado.pool };
    let criado = dao.inserir(novo)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok((StatusCode::CREATED, Json(criado)))
}

pub async fn excluir_produto(
    State(estado): State<AppState>, 
    Path(id): Path<i32> 
) -> Result<(StatusCode, Json<Produto>), StatusCode> {
    let dao = ProdutoConnect { pool: estado.pool };
    let excluido = dao.excluir(id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(excluido)))
}

pub async fn editar_produto(
    State(estado): State<AppState>,
    Json(editado): Json<Produto>
) -> Result<(StatusCode, Json<Produto>), StatusCode> {
    let dao = ProdutoConnect { pool: estado.pool }; 
    let edited = dao.editar(editado)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(edited))) 
}

pub async fn consultar_produtos(
    State(estado): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Produto>>), StatusCode> { 
    let dao = ProdutoConnect { pool: estado.pool }; 
    let consulta = dao.consultar()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
    Ok((StatusCode::OK, Json(consulta))) 
}