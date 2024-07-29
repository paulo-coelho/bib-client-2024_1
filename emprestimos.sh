#!/bin/bash

echo "Realizando algumas pesquisas...."
./bib-client.sh -p 8000 -o pesquisa -v titulo:Livro_E\&isbn:A
./bib-client.sh -p 8000 -o pesquisa -v titulo:Livro_E\&autor:Autor_E
./bib-client.sh -p 8001 -o pesquisa -v titulo:Livro_E\&autor:Autor_
./bib-client.sh -p 8001 -o pesquisa -v titulo:Livro_E\|autor:Autor_

echo "Emprestando 1 livro"
./bib-client.sh -p 8000 -o empresta -k 11111 -k DDDDD

echo "Emprestando mais de 1 livro"
./bib-client.sh -p 8001 -o empresta -k 33333 -k DDDDD -k CCCCC

echo "Verificando livros emprestados"
./bib-client.sh -p 8000 -o lista-emprestados

echo "Verificando livros em falta"
./bib-client.sh -p 8001 -o lista-em-falta

echo "Verificando usuarios"
./bib-client.sh -p 8000 -o lista-bloqueados

echo "Dormindo um pouco mais de 10s"
for i in `seq 1 12`; do echo -n "."; sleep 1; done

echo "Bloqueando usuarios"
./bib-client.sh -p 8000 -o bloqueia
./bib-client.sh -p 8001 -o bloqueia

echo "Tentando emprestar livro indisponível"
./bib-client.sh -p 8000 -o empresta -k 22222 -k DDDDD

echo "Tentando emprestar após 'bloqueio'"
./bib-client.sh -p 8001 -o empresta -k 11111 -k EEEEE

echo "Verificando livros emprestados"
./bib-client.sh -p 8001 -o lista-emprestados

echo "Verificando livros em falta"
./bib-client.sh -p 8000 -o lista-em-falta

echo "Verificando usuarios"
./bib-client.sh -p 8001 -o lista-bloqueados

echo "Devolvendo 1 livro"
./bib-client.sh -p 8001 -o devolve -k 11111 -k DDDDD

echo "Liberando usuarios"
./bib-client.sh -p 8000 -o libera
./bib-client.sh -p 8001 -o libera

echo "Verificando livros emprestados"
./bib-client.sh -p 8000 -o lista-emprestados

echo "Verificando livros em falta"
./bib-client.sh -p 8001 -o lista-em-falta

echo "Verificando usuarios"
./bib-client.sh -p 8000 -o lista-bloqueados

echo "Tentando emprestar após 'liberacao'"
./bib-client.sh -p 8001 -o empresta -k 11111 -k EEEEE

echo "Verificando livros emprestados"
./bib-client.sh -p 8000 -o lista-emprestados

echo "Verificando livros em falta"
./bib-client.sh -p 8001 -o lista-em-falta

echo "Verificando usuarios"
./bib-client.sh -p 8000 -o lista-bloqueados

