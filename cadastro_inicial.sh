#!/bin/bash

echo "cadastrando 8 usuarios..."
./adm-client.sh -p 9000 -b usuarios -o add -k 11111 -v usuario_numero_1
./adm-client.sh -p 9000 -b usuarios -o add -k 22222 -v usuario_numero_2
./adm-client.sh -p 9000 -b usuarios -o add -k 33333 -v usuario_numero_3
./adm-client.sh -p 9000 -b usuarios -o add -k 44444 -v usuario_numero_4
./adm-client.sh -p 9001 -b usuarios -o add -k 55555 -v usuario_numero_5
./adm-client.sh -p 9001 -b usuarios -o add -k 66666 -v usuario_numero_6
./adm-client.sh -p 9001 -b usuarios -o add -k 77777 -v usuario_numero_7
./adm-client.sh -p 9001 -b usuarios -o add -k 88888 -v usuario_numero_8

echo "cadastrando 8 livros..."
./adm-client.sh -p 9000 -b livros -o add -k AAAAA -v Livro_AAAAA -v Autor_AAAAA -v 5
./adm-client.sh -p 9000 -b livros -o add -k BBBBB -v Livro_BBBBB -v Autor_BBBBB -v 4
./adm-client.sh -p 9000 -b livros -o add -k CCCCC -v Livro_CCCCC -v Autor_CCCCC -v 3
./adm-client.sh -p 9000 -b livros -o add -k DDDDD -v Livro_DDDDD -v Autor_DDDDD -v 2
./adm-client.sh -p 9001 -b livros -o add -k EEEEE -v Livro_EEEEE -v Autor_EEEEE -v 1
./adm-client.sh -p 9001 -b livros -o add -k FFFFF -v Livro_FFFFF -v Autor_FFFFF -v 5
./adm-client.sh -p 9001 -b livros -o add -k GGGGG -v Livro_GGGGG -v Autor_GGGGG -v 3
./adm-client.sh -p 9001 -b livros -o add -k HHHHH -v Livro_HHHHH -v Autor_HHHHH -v 1


echo "verificando valores em cada servidor..."
./adm-client.sh -p 9000 -b usuarios -o get-all
./adm-client.sh -p 9001 -b usuarios -o get-all
./adm-client.sh -p 9000 -b livros -o get-all
./adm-client.sh -p 9001 -b livros -o get-all

echo "editando 1 livro e 1 usuario..."
./adm-client.sh -p 9001 -b usuarios -o upd -k 66666 -v usuario_EDITADO_numero_6
./adm-client.sh -p 9000 -b livros -o upd -k AAAAA -v Livro_EDITADO_AAAAA -v Autor_EDITADO_AAAAA -v 8
echo "excluindo 1 livro e 1 usuario..."
./adm-client.sh -p 9001 -b usuarios -o del -k 88888
./adm-client.sh -p 9000 -b livros -o del -k BBBBB

echo "verificando valores em cada servidor..."
./adm-client.sh -p 9000 -b usuarios -o get-all
./adm-client.sh -p 9001 -b usuarios -o get-all
./adm-client.sh -p 9000 -b livros -o get-all
./adm-client.sh -p 9001 -b livros -o get-all

