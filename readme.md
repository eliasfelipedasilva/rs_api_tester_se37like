# 🚀 rs_api_tester_se37like

**"Testing SAP APIs without needing 4GB of RAM to open Postman or 3 meetings to explain what a Header is."**
**"Testando APIs SAP sem precisar de 4GB de RAM para abrir o Postman ou de 3 reuniões para explicar o que é um Header."**

---

## 📸 Screenshots & Previews / Telas do Programa

### 1. Main Menu / Menu Principal
> *The entry point to your salvation. / O ponto de partida para sua salvação.*
> [cite_start]![Main Menu](https://drive.google.com/uc?export=view&id=1g9Zw_x_UUGThjN10eNo8WjIcE3HiQMW3) [cite: 2]

### 2. API Call Selection / Seleciona Chamada API
> *Choosing the service endpoint. / Escolhendo o endpoint do serviço.*
> [cite_start]![API Call Selection](https://drive.google.com/uc?export=view&id=1s1SgtgK_5X7F8dGZ2m1MAERe0-W_g6Gb) [cite: 4]

### 3. API Operation Selection / Seleciona Operação da API
> *Choosing the HTTP method (GET, POST, etc). / Escolhendo o método HTTP.*
> [cite_start]![API Operation](https://drive.google.com/uc?export=view&id=1Hx4he3lhH4oSevNoSV8SEvG6u9PjBzX4) [cite: 6]

### 4. Filters and View Format / Seleciona Filtros e Formato de View
> *Refining your search and choosing the output. / Refinando a busca e escolhendo a saída.*
> [cite_start]![Filters and View](https://drive.google.com/uc?export=view&id=1JbfEc1dub6XGT7uFmf21j2XWz5uqGa-0) [cite: 5]

### 5. ALV Grid Visualization / Tabela ALV Visualização
> *Structured data just like the classic SAP grid. / Dados estruturados como o grid clássico do SAP.*
> [cite_start]![ALV Grid](https://drive.google.com/uc?export=view&id=1mqr7LGd-OWjgxbvrKT-9QLd9Z0vWN1cW) [cite: 7]

### 6. Save Variant / Salva Variante
> *Storing your parameters for future use. / Armazenando seus parâmetros para uso futuro.*
> [cite_start]![Save Variant](https://drive.google.com/uc?export=view&id=1CpmiapKQHasJWIAtgzP90EbJqzZrGZqj) [cite: 3]

### 7. Consult Saved Variant / Consulta Variante Salva
> *Execution of a previously saved test. / Execução de um teste salvo anteriormente.*
> [cite_start]![Consult Variant](https://drive.google.com/uc?export=view&id=1gt4VzHCAZ3O9s-bI7IRAS0nbwAroFXcC) [cite: 1]

### 8. Selecting Saved Variants / Variantes Salvas Selecionando
> *Quick access to your history. / Acesso rápido ao seu histórico.*
> [cite_start]![Selecting Variants](https://drive.google.com/uc?export=view&id=1xFBu_Me9bcefT9LECXxNHqu5xQk5--TS) [cite: 8]

---

## 🇺🇸 English Version

### 🧐 What is this?
Do you know what **SE37** is? If not, close this repo. If you do, you understand the trauma of S/4HANA migrations and fighting with OData, REST, and the godforsaken **X-CSRF-Token**.

This is a **Rust-powered** API tester for those whose patience with bloated software has peaked. It brings 90s comfort — the good old **ALV Grid** — straight to your terminal. It's fast, grumpy, and won't eat your RAM while you wait for SAP to decide between a 500 error or a timeout.

### 💾 Download (Binaries)
No Rust/Cargo needed. Just download, add your JSONs to the same folder, and hit the virtual F8.

* **🪟 [Download for Windows (.exe)](https://github.com/eliasfelipedasilva/rs_api_tester_se37like/releases/download/0.0.1/rs_api_tester_se37like.exe)**
* **🐧 [Download for Linux](https://github.com/eliasfelipedasilva/rs_api_tester_se37like/releases/download/0.0.1/rs_api_tester_se37like)**
* **🍎 Mac Version:** I don't use drugs. (NÃO USO DROGAS).

---

## 🇧🇷 Versão em Português

### 🧐 O que é isso?
Você sabe o que é a **SE37**? Se não sabe, feche este repositório agora. Se sabe, você entende o trauma de migrar para o S/4HANA e ter que lidar com OData, REST e o inferno do **X-CSRF-Token**.

Este testador é escrito em **Rust** (porque sua paciência com software lento já esgotou) e traz o conforto dos anos 90 — o bom e velho **ALV Grid** — direto para o seu terminal. Rápido e ranzinza, ele não devora sua RAM enquanto você espera o SAP decidir entre um erro 500 ou um timeout.

### 💾 Download (Binários Prontos)
Não precisa instalar nada. Baixe, coloque seus arquivos JSON na mesma pasta e aperte o F8 virtual.

* **🪟 [Baixar para Windows (.exe)](https://github.com/eliasfelipedasilva/rs_api_tester_se37like/releases/download/0.0.1/rs_api_tester_se37like.exe)**
* **🐧 [Baixar para Linux](https://github.com/eliasfelipedasilva/rs_api_tester_se37like/releases/download/0.0.1/rs_api_tester_se37like)**
* **🍎 Versão MAC:** NÃO USO DROGAS.

---

## 📂 Template: ambiente.json
```json
{
  "host": "[https://seu-sap-s4.com](https://seu-sap-s4.com)",
  "base_path": "/sap/opu/odata/sap/API_BUSINESS_PARTNER",
  "auth_type": "basic",
  "username": "SEU_USER",
  "password": "SUA_PASSWORD"
}