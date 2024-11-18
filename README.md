# API do desafio técnico para vaga de desenvolvedor React Native da empresa OOriginal.

Esse projeto é um desafio técnico feito para demonstrar os conhecimentos em uma aplicação back-end utilizando Rust e Docker. 

## Pré-requisitos

- Cargo 1.82 (não testei em versões menores, mas deve funcionar)
- Docker

## Instalação

Passos para instalar o projeto:

1. Clone o repositório:
   ```bash
   git clone https://github.com/minotti21/test-ooriginal-api.git
   ```

2. Navegue para o diretório do projeto:
   ```bash
   cd test-ooriginal-api
   ```

3. Crie um arquivo .env na raíz do projeto com a seguinte chave/valor: DATABASE_URL=postgresql://postgres.pdxpjrgcttjuxrvabbkd:QyzvWcYR2i3QxiDC@aws-0-sa-east-1.pooler.supabase.com:5432/postgres

## Executando o Projeto

1. Crie uma imagem docker:
   ```bash
   docker build -t ooriginal-qrcode-api .
   ```
   
2. Rode a imagem:
   ```bash
   docker run -p 8080:8080 ooriginal-qrcode-api
   ```

## Pronto, seu servidor estará rodando no localhost na porta 8080
