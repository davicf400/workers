mod models;
mod daos;
mod handlers;
use tower_http::cors::CorsLayer;
use axum::response::Html;

use axum::{
    routing::{get, delete},
    Router,
};
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
}
async fn tela_principal() -> Html<&'static str> {
    
    Html(include_str!("../index.html")) 
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:admin123@localhost/workers")
        .await
        .expect(" Erro fatal: Não consegui conectar no banco de dados!");

    let estado = AppState { pool };

    let app = Router::new()
    .layer(CorsLayer::permissive())
    .route("/", get(tela_principal))
        .route(
            "/funcionarios", 
            get(handlers::funcionario_handler::consultar_funcionario)
            .post(handlers::funcionario_handler::criar_funcionario)
            
        )
        .route(
            "/funcionarios/{id}", 
            delete(handlers::funcionario_handler::excluir_funcionario)
            .put(handlers::funcionario_handler::editar_funcionario)
        )
        .route(
            "/produtos", 
            get(handlers::produto_handler::consultar_produtos)
            .post(handlers::produto_handler::criar_produto)
           
        )
        .route(
            "/produtos/{id}", 
            delete(handlers::produto_handler::excluir_produto)
             .put(handlers::produto_handler::editar_produto)
        )
        .route(
            "/departamentos", 
            get(handlers::departamento_handler::consultar_departamentos)
            .post(handlers::departamento_handler::criar_departamento)
           
        )
        .route(
            "/departamentos/{id}", 
            delete(handlers::departamento_handler::excluir_departamento)
             .put(handlers::departamento_handler::editar_departamento)
        )
        .route(
            "/fornecedores", 
            get(handlers::fornecedor_handler::consultar_fornecedores)
            .post(handlers::fornecedor_handler::criar_fornecedor)
            
        )
        .route(
            "/fornecedores/{id}", 
            delete(handlers::fornecedor_handler::excluir_fornecedor)
            .put(handlers::fornecedor_handler::editar_fornecedor)
        )
        .with_state(estado);

    println!("🚀 Sistema no ar! Servidor rodando em http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}