# GBC074 - Sistemas Distribuídos - 2024-1

Projeto de um sistema de empréstimos de livros - Clientes para validação e testes.

* Portal Administrativo: cadastro de usuários e livros.
* Portal Biblioteca: gerência de empréstimos e devoluções de livros, bloqueios e liberações de usuários, bem como consultas específicas.

## Dependências

* protobuf (para uso do binário `protoc`)
* Rust

## Compilação

* `cargo build`

## Uso do portal administrativo

* Cliente: `./adm-client.sh -p 9000 --help`, em que 9000 representa a porta do servidor. A opção `--help` lista os possíveis argumentos adicionais.

## Uso do portal biblioteca

* Cliente: `./bib-client.sh -p 8000 --help`, em que 8000 representa a porta do servidor. A opção `--help` lista os possíveis argumentos adicionais.

## Testes

* O *scripts* `cadastro_inicial.sh` realiza o cadastro inicial de alguns usuários e livros.
* O *scripts* `emprestimos.sh` deve ser executado após o cadastro inicial para realização de empréstimos e devoluções, bloqueios e liberações de usuários.
* Os arquivos `out_cadastro_inicial.txt` e `out_emprestimos.txt` indicam as saídas esperadas de acordo com o comportamento apresentado na descrição do projeto.
* Os *scripts* assumem que existem duas instâncias do portal administrativo nas portas 9000 e 9001, bem como duas instcias do portal biblioteca nas portas 8000 e 8001.
