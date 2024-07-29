use crate::biblioteca::portal_biblioteca_client::PortalBibliotecaClient;
use crate::biblioteca::{Criterio, Identificador, UsuarioLivro, Vazia};
use clap::{Parser, ValueEnum};
use tonic::Request;

pub mod biblioteca {
    tonic::include_proto!("biblioteca");
}

/// Portal Biblioteca Client
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server IP address
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    address: String,
    /// Server port
    #[arg(required = true, short, long, default_value_t = 8000)]
    port: u16,
    /// Operation
    #[arg(value_enum, short, long, default_value_t = Operation::Pesquisa)]
    op: Operation,
    /// Keys (multiple if needed)
    #[arg(short, long)]
    key: Vec<String>,
    /// Values (if needed)
    #[arg(short, long)]
    val: Vec<String>,
}

/// Allowed operations
#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Operation {
    Empresta,
    Devolve,
    Bloqueia,
    Libera,
    Pesquisa,
    ListaBloqueados,
    ListaEmprestados,
    ListaEmFalta,
}

fn args_to_usuario_livro(mut k: Vec<String>) -> Vec<UsuarioLivro> {
    assert!(
        k.len() >= 2,
        "Informe o identificador do usuario e o(s) identificador(es) do(s) livro(s)!"
    );
    let mut uls = Vec::new();
    let id_usuario = Identificador { id: k.remove(0) };
    while !k.is_empty() {
        let id_livro = Identificador { id: k.remove(0) };
        let ul = UsuarioLivro {
            usuario: Some(id_usuario.clone()),
            livro: Some(id_livro),
        };
        uls.push(ul);
    }
    uls
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("\nArguments = {:?}", args);
    let mut client =
        PortalBibliotecaClient::connect(format!("http://{}:{}", args.address, args.port)).await?;

    let k = args.key;
    let mut v = args.val;

    let reply: String = match args.op {
        Operation::Empresta => {
            format!(
                "{:?}",
                client
                    .realiza_emprestimo(Request::new(tokio_stream::iter(args_to_usuario_livro(k))))
                    .await
                    .unwrap()
                    .into_inner()
            )
        }
        Operation::Devolve => {
            format!(
                "{:?}",
                client
                    .realiza_devolucao(Request::new(tokio_stream::iter(args_to_usuario_livro(k))))
                    .await
                    .unwrap()
                    .into_inner()
            )
        }
        Operation::Bloqueia => {
            format!(
                "{:?}",
                client
                    .bloqueia_usuarios(Request::new(Vazia {}))
                    .await
                    .unwrap()
                    .into_inner()
            )
        }
        Operation::Libera => {
            format!(
                "{:?}",
                client
                    .libera_usuarios(Request::new(Vazia {}))
                    .await
                    .unwrap()
                    .into_inner()
            )
        }
        Operation::Pesquisa => {
            assert!(
                v.len() > 0,
                "Informe ao menos um criterio de pesquisa no argumento 'val'!"
            );
            let criterio = Criterio {
                criterio: v.remove(0),
            };
            let mut stream = client
                .pesquisa_livro(Request::new(criterio))
                .await?
                .into_inner();
            let mut s = String::new();
            while let Some(livro) = stream.message().await? {
                s = s + format!("{:?}\n", livro).as_str();
            }
            s
        }
        Operation::ListaBloqueados => {
            let mut stream = client
                .lista_usuarios_bloqueados(Request::new(Vazia {}))
                .await?
                .into_inner();
            let mut s = String::new();
            while let Some(livro) = stream.message().await? {
                s = s + format!("{:?}\n", livro).as_str();
            }
            s
        }
        Operation::ListaEmprestados => {
            let mut stream = client
                .lista_livros_emprestados(Request::new(Vazia {}))
                .await?
                .into_inner();
            let mut s = String::new();
            while let Some(livro) = stream.message().await? {
                s = s + format!("{:?}\n", livro).as_str();
            }
            s
        }
        Operation::ListaEmFalta => {
            let mut stream = client
                .lista_livros_em_falta(Request::new(Vazia {}))
                .await?
                .into_inner();
            let mut s = String::new();
            while let Some(livro) = stream.message().await? {
                s = s + format!("{:?}\n", livro).as_str();
            }
            s
        }
    };
    println!("Reply = {}", reply);
    Ok(())
}
