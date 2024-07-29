use cadastro::portal_cadastro_client::PortalCadastroClient;
use cadastro::{Identificador, Livro, Usuario, Vazia};
use clap::{Parser, ValueEnum};
use tonic::Request;

pub mod cadastro {
    tonic::include_proto!("cadastro");
}

/// Portal Cadastro Client
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Server IP address
    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    address: String,
    /// Server port
    #[arg(required = true, short, long, default_value_t = 9000)]
    port: u16,
    /// Database
    #[arg(value_enum, short, long, default_value_t = Base::Usuarios)]
    base: Base,
    /// Operation
    #[arg(value_enum, short, long, default_value_t = Operation::Get)]
    op: Operation,
    /// Keys (multiple if needed)
    #[arg(short, long)]
    key: Vec<String>,
    /// Values (if needed)
    ///
    /// # Example
    ///
    /// To create a new book, run:
    /// `cargo run --bin=adm-client -- -p 9000 -b livros -o add -k 1234 -v titulo -v autor -v 5`
    #[arg(short, long)]
    val: Vec<String>,
}

/// Data store to execute the operation
#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Base {
    Livros,
    Usuarios,
}

/// Allowed operations
#[derive(ValueEnum, Clone, Debug, PartialEq)]
enum Operation {
    Get,
    Add,
    Upd,
    Del,
    GetAll,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("\nArguments = {:?}", args);
    let mut client =
        PortalCadastroClient::connect(format!("http://{}:{}", args.address, args.port)).await?;

    let k = args.key;
    let v = args.val;

    if args.base == Base::Livros {
        let reply: String = match args.op {
            Operation::Get => {
                assert!(k.len() > 0, "Informe o identificador do livro!");
                let id = Identificador {
                    id: k.get(0).unwrap().to_owned(),
                };
                format!(
                    "{:?}",
                    client
                        .obtem_livro(Request::new(id))
                        .await
                        .unwrap()
                        .into_inner()
                )
            }
            Operation::Add => {
                assert!(k.len() > 0, "Informe o identificador do livro!");
                assert!(
                    v.len() == 3,
                    "Informe os seguintes valores: titulo, autor e total!"
                );
                let l = Livro {
                    isbn: k.get(0).unwrap().to_owned(),
                    titulo: v.get(0).unwrap().to_owned(),
                    autor: v.get(1).unwrap().to_owned(),
                    total: v.get(2).unwrap().parse().unwrap(),
                };
                format!(
                    "{:?}",
                    client
                        .novo_livro(Request::new(l))
                        .await
                        .unwrap()
                        .into_inner()
                )
            }
            Operation::Upd => {
                assert!(k.len() > 0, "Informe o identificador do livro!");
                assert!(
                    v.len() == 3,
                    "Informe os seguintes valores: titulo, autor e total!"
                );
                let l = Livro {
                    isbn: k.get(0).unwrap().to_owned(),
                    titulo: v.get(0).unwrap().to_owned(),
                    autor: v.get(1).unwrap().to_owned(),
                    total: v.get(2).unwrap().parse().unwrap(),
                };
                format!(
                    "{:?}",
                    client
                        .edita_livro(Request::new(l))
                        .await
                        .unwrap()
                        .into_inner()
                )
            }
            Operation::Del => {
                assert!(k.len() > 0, "Informe o identificador do livro!");
                let id = Identificador {
                    id: k.get(0).unwrap().to_owned(),
                };
                format!(
                    "{:?}",
                    client
                        .remove_livro(Request::new(id))
                        .await
                        .unwrap()
                        .into_inner()
                )
            }
            Operation::GetAll => {
                let mut stream = client
                    .obtem_todos_livros(Request::new(Vazia {}))
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
    } else if args.base == Base::Usuarios {
        let reply: String = match args.op {
            Operation::Get => {
                assert!(k.len() > 0, "Informe o identificador do usuario!");
                let id = Identificador {
                    id: k.get(0).unwrap().to_owned(),
                };
                format!(
                    "{:?}",
                    client
                        .obtem_usuario(Request::new(id))
                        .await
                        .unwrap()
                        .into_inner()
                )
            }
            Operation::Add => {
                assert!(k.len() > 0, "Informe o identificador do usuario!");
                assert!(v.len() == 1, "Informe o nome do usuario!");
                let u = Usuario {
                    cpf: k.get(0).unwrap().to_owned(),
                    nome: v.get(0).unwrap().to_owned(),
                };
                format!(
                    "{:?}",
                    client
                        .novo_usuario(Request::new(u))
                        .await
                        .unwrap()
                        .into_inner()
                )
            }
            Operation::Upd => {
                assert!(k.len() > 0, "Informe o identificador do usuario!");
                assert!(v.len() == 1, "Informe o nome do usuario!");
                let u = Usuario {
                    cpf: k.get(0).unwrap().to_owned(),
                    nome: v.get(0).unwrap().to_owned(),
                };
                format!(
                    "{:?}",
                    client
                        .edita_usuario(Request::new(u))
                        .await
                        .unwrap()
                        .into_inner()
                )
            }
            Operation::Del => {
                assert!(k.len() > 0, "Informe o identificador do usuario!");
                let id = Identificador {
                    id: k.get(0).unwrap().to_owned(),
                };
                format!(
                    "{:?}",
                    client
                        .remove_usuario(Request::new(id))
                        .await
                        .unwrap()
                        .into_inner()
                )
            }
            Operation::GetAll => {
                let mut stream = client
                    .obtem_todos_usuarios(Request::new(Vazia {}))
                    .await?
                    .into_inner();
                let mut s = String::new();
                while let Some(usuario) = stream.message().await? {
                    s = s + format!("{:?}\n", usuario).as_str();
                }
                s
            }
        };
        println!("Reply = {}", reply);
    }
    Ok(())
}
