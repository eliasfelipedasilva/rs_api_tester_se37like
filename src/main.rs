use std::{env, fs, path::PathBuf, time::Instant};
use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use reqwest::blocking::ClientBuilder;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use base64::{engine::general_purpose, Engine};
use comfy_table::{Table, presets::UTF8_FULL, Cell, Color, Attribute, Row};
use colored::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Ambiente {
    host: String,
    base_path: Option<String>,
    auth_type: String,
    username: Option<String>,
    password: Option<String>,
    token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct HistoricoChamada {
    nome_variante: String,
    url_final: String,
    metodo: String,
    auth_header: String,
    params_query: Vec<(String, String)>,
    payload_body: Map<String, Value>,
}

fn main() {
    limpar_tela();
    println!("{}", "======================================================".blue());
    println!("{}", "   RS_API_TESTER_SE37LIKE - THE POSTMAN KILLER       ".bold().blue());
    println!("{}", "======================================================".blue());

    let modo = Select::new("O QUE DESEJA FAZER?", vec!["🆕 Iniciar Novo Teste", "📂 Consultar Chamadas Salvas (History)"])
        .prompt()
        .ok();

    if let Some(m) = modo {
        let chamada = if m.contains("Novo") {
            montar_nova_chamada()
        } else {
            carregar_historico()
        };

        if let Some(mut req_data) = chamada {
            if revisar_e_editar(&mut req_data) {
                executar_fluxo_final(&mut req_data);
            }
        }
    }
}

// --- LÓGICA DE TRATAMENTO DE TIPOS (ESSENCIAL PARA O SAP) ---

fn tipar_valor(valor: String, tipo_swagger: &str) -> Value {
    if valor.is_empty() { return Value::Null; }
    match tipo_swagger {
        "boolean" => {
            let v = valor.to_lowercase();
            Value::Bool(v == "true" || v == "x" || v == "s" || v == "1")
        },
        "integer" | "number" | "int32" | "int64" => {
            if let Ok(n) = valor.parse::<i64>() {
                Value::Number(n.into())
            } else {
                Value::String(valor)
            }
        },
        _ => Value::String(valor),
    }
}

// --- MAPEAMENTO E MONTAGEM DA CHAMADA ---

fn montar_nova_chamada() -> Option<HistoricoChamada> {
    let cam_env = buscar_arquivo_f4("📂 SELECIONE O ARQUIVO DE AMBIENTE (JSON)");
    let env_content = fs::read_to_string(cam_env).ok()?;
    let config_env: Ambiente = serde_json::from_str(&env_content).ok()?;

    let cam_api = buscar_arquivo_f4("📖 SELECIONE O SWAGGER/OPENAPI (JSON)");
    let api_content = fs::read_to_string(cam_api).ok()?;
    let api_json: Value = serde_json::from_str(&api_content).ok()?;

    let paths = api_json["paths"].as_object()?;
    let mut rotas: Vec<String> = paths.keys().cloned().collect();
    rotas.sort();
    let rota_sel = Select::new("Escolha o Endpoint:", rotas).prompt().ok()?;

    let metodos_map = paths[&rota_sel].as_object()?;
    let mut metodos: Vec<String> = metodos_map.keys().cloned().collect();
    let m_sel = Select::new("Escolha a Operação:", metodos).prompt().ok()?.to_uppercase();

    // Mapeamento extraindo Tipos do Swagger
    let campos = mapear_com_tipos(&api_json, &rota_sel, &m_sel);
    
    let mut q_params = Vec::new();
    let mut b_params = Map::new();

    println!("\n{}", "✏️  PARAMETRIZAÇÃO DA CHAMADA:".bold().yellow());
    for (nome, tipo, categoria) in campos {
        let valor = Text::new(&format!("{} ({}):", nome, tipo.cyan())).prompt().unwrap_or_default();
        
        if categoria == "BODY" {
            let val_tipado = tipar_valor(valor, &tipo);
            if val_tipado != Value::Null {
                b_params.insert(nome, val_tipado);
            }
        } else {
            if !valor.is_empty() || nome.contains("$top") {
                let v = if valor.is_empty() && nome.contains("$top") { "50".into() } else { valor };
                q_params.push((nome, v));
            }
        }
    }

    // Montagem correta da URL (S/4HANA Cloud Friendly)
    let base_host = config_env.host.trim_end_matches('/');
    let raw_path = api_json["servers"][0]["url"].as_str().unwrap_or("");
    let clean_path = if raw_path.contains("://") {
        raw_path.find("/sap").map(|i| &raw_path[i..]).unwrap_or("")
    } else { raw_path }.replace("{host}", "").replace("{port}", "");

    let url_final = format!("{}{}{}", base_host, clean_path.trim_end_matches('/'), rota_sel);

    let auth = if config_env.auth_type.to_lowercase() == "basic" {
        let creds = format!("{}:{}", config_env.username.as_ref().unwrap_or(&"".into()), config_env.password.as_ref().unwrap_or(&"".into()));
        format!("Basic {}", general_purpose::STANDARD.encode(creds))
    } else { 
        format!("Bearer {}", config_env.token.as_ref().unwrap_or(&"".into())) 
    };

    Some(HistoricoChamada { nome_variante: "Nova".into(), url_final, metodo: m_sel, auth_header: auth, params_query: q_params, payload_body: b_params })
}

fn mapear_com_tipos(json: &Value, rota: &str, metodo: &str) -> Vec<(String, String, String)> {
    let mut c = vec![];
    let m = metodo.to_lowercase();
    
    if m == "get" {
        c.push(("$filter".to_string(), "string".into(), "QUERY".into()));
        c.push(("$top".to_string(), "integer".into(), "QUERY".into()));
        c.push(("$select".to_string(), "string".into(), "QUERY".into()));
    }

    if let Some(op) = json["paths"][rota].get(&m) {
        if let Some(ps) = op["parameters"].as_array() {
            for p in ps {
                let name = p["name"].as_str().unwrap_or("").to_string();
                let tipo = p["type"].as_str().or(p["schema"]["type"].as_str()).unwrap_or("string").to_string();
                let loc = p["in"].as_str().unwrap_or("query").to_uppercase();
                c.push((name, tipo, loc));
            }
        }
        
        let schema_ptr = op.get("requestBody").and_then(|rb| rb.get("content")).and_then(|cont| cont.get("application/json")).and_then(|j| j.get("schema"));

        if let Some(schema) = schema_ptr {
            let mut target = schema;
            if let Some(ref_path) = schema.get("$ref").and_then(|r| r.as_str()) {
                target = buscar_ref(json, ref_path).unwrap_or(schema);
            }

            if let Some(props) = target.get("properties").and_then(|p| p.as_object()) {
                for (k, v) in props {
                    let tipo = v["type"].as_str().unwrap_or("string").to_string();
                    c.push((k.clone(), tipo, "BODY".into()));
                }
            }
        }
    }
    c
}

fn buscar_ref<'a>(json: &'a Value, path: &str) -> Option<&'a Value> {
    let mut curr = json;
    for p in path.split('/').filter(|&s| s != "#") {
        if let Some(next) = curr.get(p) { curr = next; } else { return None; }
    }
    Some(curr)
}

// --- INTERFACE DE REVISÃO E EXECUÇÃO ---

fn revisar_e_editar(req_data: &mut HistoricoChamada) -> bool {
    loop {
        limpar_tela();
        println!("{}", "======================================================".yellow());
        println!("{}", "   TELA DE PARÂMETROS (REVISÃO ANTES DO F8)          ".bold().yellow());
        println!("{}", "======================================================".yellow());
        println!("{}: {}", "URL".bold(), req_data.url_final.cyan());
        println!("{}: {}", "MÉTODO".bold(), req_data.metodo.green());
        
        println!("\n{}:", "QUERY/PATH".bold());
        for (k, v) in &req_data.params_query { println!("  {}: {}", k.yellow(), v); }

        if req_data.metodo != "GET" {
            println!("\n{}:", "BODY (JSON PAYLOAD)".bold());
            println!("{}", serde_json::to_string_pretty(&req_data.payload_body).unwrap_or_default().white());
        }

        let acao = Select::new("AÇÃO:", vec!["🚀 Executar (F8)", "✏️  Editar Query", "📦 Editar Body", "💾 Salvar Variante Agora", "🗑️  Cancelar"])
            .prompt()
            .ok();

        match acao {
            Some(a) if a.contains("Executar") => return true,
            Some(a) if a.contains("Query") => {
                let k = Text::new("Campo:").prompt().unwrap_or_default();
                let v = Text::new("Valor:").prompt().unwrap_or_default();
                if !k.is_empty() {
                    req_data.params_query.retain(|(x, _)| x != &k);
                    req_data.params_query.push((k, v));
                }
            },
            Some(a) if a.contains("Body") => {
                let k = Text::new("Campo Técnico:").prompt().unwrap_or_default();
                let v = Text::new("Valor:").prompt().unwrap_or_default();
                if !k.is_empty() {
                    // Tenta inferir tipo na edição manual rápida
                    let val = if v.to_lowercase() == "true" { Value::Bool(true) }
                              else if v.to_lowercase() == "false" { Value::Bool(false) }
                              else if let Ok(n) = v.parse::<i64>() { Value::Number(n.into()) }
                              else { Value::String(v) };
                    req_data.payload_body.insert(k, val);
                }
            },
            Some(a) if a.contains("Salvar") => {
                let nome = Text::new("Nome da variante:").prompt().unwrap_or_default();
                if !nome.is_empty() {
                    fs::create_dir_all("./history").ok();
                    req_data.nome_variante = nome.clone();
                    fs::write(format!("./history/{}.json", nome), serde_json::to_string_pretty(&req_data).unwrap()).ok();
                    println!("{}", "✅ Variante salva!".green());
                }
            },
            _ => return false,
        }
    }
}

fn executar_fluxo_final(req_data: &mut HistoricoChamada) {
    let client = ClientBuilder::new().cookie_store(true).danger_accept_invalid_certs(true).build().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, req_data.auth_header.parse().unwrap());
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    if req_data.metodo != "GET" {
        println!("{}", "🛡️  Fetching CSRF Token...".yellow());
        let _ = client.get(&req_data.url_final).headers(headers.clone()).header("x-csrf-token", "fetch").send().map(|r| {
            if let Some(t) = r.headers().get("x-csrf-token") { headers.insert("x-csrf-token", t.clone()); }
        });
    }

    let req_builder = match req_data.metodo.as_str() {
        "POST" => client.post(&req_data.url_final).json(&req_data.payload_body),
        "PUT" => client.put(&req_data.url_final).json(&req_data.payload_body),
        "PATCH" => client.patch(&req_data.url_final).json(&req_data.payload_body),
        "DELETE" => client.delete(&req_data.url_final),
        _ => client.get(&req_data.url_final),
    };

    let req = req_builder.headers(headers).query(&req_data.params_query);

    println!("\n⏳ Aguardando Resposta do SAP...");
    let inicio = Instant::now();
    
    if let Ok(res) = req.send() {
        let status = res.status();
        let duracao = inicio.elapsed();
        let texto = res.text().unwrap_or_default();
        let json_val: Value = serde_json::from_str(&texto).unwrap_or(Value::Null);
        
        loop {
            limpar_tela();
            let status_display = if status.is_success() { status.to_string().green() } else { status.to_string().red() };
            println!("\nSTATUS: {} | TEMPO: {:?}", status_display, duracao);
            
            let acao = Select::new("AÇÃO:", vec!["📊 Ver Tabela ALV", "🔍 Ver JSON Técnico", "💾 Salvar Variante", "⬅️ Sair"]).prompt().ok();
            
            match acao {
                Some(a) if a.contains("Tabela") => {
                    if let Some(l) = extrair_lista_sap(&json_val) { renderizar_tabela(l); }
                    else { println!("{}", "⚠️  Nenhum dado de lista encontrado na resposta.".yellow()); }
                    let _ = Text::new("Pressione ENTER para voltar...").prompt();
                },
                Some(a) if a.contains("JSON") => {
                    println!("{}", serde_json::to_string_pretty(&json_val).unwrap_or(texto.clone()).cyan());
                    let _ = Text::new("Pressione ENTER para voltar...").prompt();
                },
                Some(a) if a.contains("Salvar") => {
                    let nome = Text::new("Nome da variante:").prompt().unwrap_or_default();
                    if !nome.is_empty() {
                        fs::create_dir_all("./history").ok();
                        fs::write(format!("./history/{}.json", nome), serde_json::to_string_pretty(&req_data).unwrap()).ok();
                    }
                    break;
                },
                _ => break,
            }
        }
    } else {
        println!("{}", "❌ Falha crítica na conexão com o servidor SAP.".red());
    }
}

// --- UTILITÁRIOS ---

fn renderizar_tabela(lista: &Vec<Value>) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    if let Some(obj) = lista.get(0).and_then(|v| v.as_object()) {
        let chaves: Vec<String> = obj.keys().filter(|k| !k.starts_with("__")).take(8).cloned().collect();
        let mut h = Row::new();
        for c in &chaves { h.add_cell(Cell::new(c).fg(Color::Blue).add_attribute(Attribute::Bold)); }
        table.set_header(h);
        for item in lista.iter().take(25) {
            let mut r = Row::new();
            for k in &chaves {
                let v = item.get(k).map(|v| v.to_string().replace("\"", "")).unwrap_or_default();
                r.add_cell(Cell::new(if v.len() > 30 { format!("{}...", &v[..27]) } else { v }));
            }
            table.add_row(r);
        }
    }
    println!("\n{table}");
}

fn extrair_lista_sap(v: &Value) -> Option<&Vec<Value>> {
    if let Some(d) = v.get("d") {
        if let Some(r) = d.get("results") { return r.as_array(); }
        return d.as_array();
    }
    if let Some(val) = v.get("value") { return val.as_array(); }
    v.as_array()
}

fn buscar_arquivo_f4(instrucao: &str) -> PathBuf {
    let mut p = env::current_dir().unwrap();
    loop {
        let mut items = vec![String::from(".. (Voltar)")];
        if let Ok(es) = fs::read_dir(&p) { 
            for e in es.flatten() { items.push(e.file_name().to_string_lossy().to_string()); } 
        }
        let s = Select::new(&format!("{} | {}", instrucao.bold().cyan(), p.display()), items).prompt().unwrap();
        if s == ".. (Voltar)" { p.pop(); } else {
            let n = p.join(&s);
            if n.is_dir() { p = n; } else { return n; }
        }
    }
}

fn carregar_historico() -> Option<HistoricoChamada> {
    if let Ok(entries) = fs::read_dir("./history") {
        let files: Vec<String> = entries.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
        if files.is_empty() { return None; }
        let sel = Select::new("Escolha a Variante Salva:", files).prompt().ok()?;
        return serde_json::from_str(&fs::read_to_string(format!("./history/{}", sel)).ok()?).ok();
    }
    None
}

fn limpar_tela() { print!("{}[2J{}[1;1H", 27 as char, 27 as char); }