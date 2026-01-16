use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::process;
use rand::Rng;
use serde::{Deserialize, Serialize}; 
use colored::*;
use std::time::Duration;
use std::thread; // <--- NOVIDADE: Para o sistema dormir (sleep)
use chrono::Local; 

// --- CONFIGURAÇÃO DO MODELO ---
const MODELO_GOOGLE: &str = "gemini-2.5-flash"; 

// --- FUNÇÃO AUXILIAR: MOEDA ---
fn brl(v: f64) -> String { format!("R$ {:.2}", v) }

// --- ESTRUTURAS FINANCEIRAS ---
#[derive(Deserialize, Debug)]
struct Moeda {
    #[serde(rename = "bid")]
    bid: String,
    #[serde(rename = "pctChange")]
    pct_change: String,
}
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct RespostaApi { BTCBRL: Moeda }

// --- ESTRUTURAS DO GEMINI ---
#[derive(Serialize)]
struct GeminiPart { text: String }
#[derive(Serialize)]
struct GeminiContent { parts: Vec<GeminiPart> }

// FERRAMENTAS (BUSCA)
#[derive(Serialize)]
struct GoogleSearch {} 

#[derive(Serialize)]
struct Tool {
    #[serde(rename = "google_search")] 
    google_search: GoogleSearch,
}

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    #[serde(skip_serializing_if = "Option::is_none")] 
    tools: Option<Vec<Tool>>, 
}

#[derive(Deserialize, Debug)]
struct GeminiCandidate { content: GeminiContentResponse }
#[derive(Deserialize, Debug)]
struct GeminiContentResponse { parts: Vec<GeminiPartResponse> }
#[derive(Deserialize, Debug)]
struct GeminiPartResponse { text: String }
#[derive(Deserialize, Debug)]
struct GeminiResponse { candidates: Option<Vec<GeminiCandidate>> }

// Diagnóstico
#[derive(Deserialize, Debug)]
struct ModelList { models: Option<Vec<ModelInfo>> }
#[derive(Deserialize, Debug)]
struct ModelInfo { name: String }

// --- ESTRUTURA DA CRIATURA ---
struct Criatura {
    nome: String,
    energia: u32,
    fome: u32,
    dinheiro: f64,
    btc_wallet: f64,
    conhecimento: u32,
    viva: bool,
    total_ganho: u64,
}

impl Criatura {
    fn nascer(nome: String) -> Criatura {
        Criatura { nome, energia: 100, fome: 0, dinheiro: 0.0, btc_wallet: 0.0, conhecimento: 0, viva: true, total_ganho: 0 }
    }

    fn carregar() -> Option<Criatura> {
        if Path::new("athena_save.txt").exists() {
            let dados = fs::read_to_string("athena_save.txt").unwrap_or_default();
            let partes: Vec<&str> = dados.lines().collect();
            if partes.len() >= 7 {
                return Some(Criatura {
                    nome: partes[0].to_string(),
                    energia: partes[1].parse().unwrap_or(100),
                    fome: partes[2].parse().unwrap_or(0),
                    dinheiro: partes[3].parse().unwrap_or(0.0),
                    total_ganho: partes[4].parse().unwrap_or(0),
                    conhecimento: partes[5].parse().unwrap_or(0),
                    btc_wallet: partes[6].parse().unwrap_or(0.0),
                    viva: true,
                });
            }
        }
        None
    }

    fn salvar(&self) {
        let dados = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}", 
            self.nome, self.energia, self.fome, self.dinheiro, self.total_ganho, self.conhecimento, self.btc_wallet);
        let _ = fs::write("athena_save.txt", dados);
    }

    fn escrever_diario(&self, texto: &str) {
        let mut arquivo = OpenOptions::new().create(true).append(true).open("diario_athena.txt").unwrap();
        let _ = writeln!(arquivo, "[Nível {}] {}", self.conhecimento, texto);
    }

    fn status(&self) {
        println!("\n📊 STATUS ATUAL:");
        let energia_txt = if self.energia > 50 { format!("{}/100", self.energia).green() } else { format!("{}/100", self.energia).red() };
        println!("   ⚡ Energia:  {}", energia_txt);
        println!("   💰 Saldo:    {}", brl(self.dinheiro).green().bold());
        println!("   🪙 Carteira: {}", format!("{:.8} BTC", self.btc_wallet).yellow());
        println!("   🧠 Nível:    {}", self.conhecimento.to_string().cyan());
    }

    fn trabalhar(&mut self) {
        if self.energia < 20 { println!("{}", "🛑 Athena: Pai, estou exausta.".red()); return; }
        let mut rng = rand::thread_rng();
        let salario = rng.gen_range(30.0..60.0);
        self.dinheiro += salario;
        self.total_ganho += salario as u64;
        self.energia = self.energia.saturating_sub(25);
        self.fome += 20;
        println!("💼 Trabalhei. Ganhei {}.", brl(salario).green());
    }

    fn comer(&mut self) {
        if self.dinheiro >= 20.0 {
            self.dinheiro -= 20.0;
            self.fome = self.fome.saturating_sub(30);
            self.energia = (self.energia + 10).min(100);
            println!("🍎 Comi algo real. {}", format!("({} -20.00)", brl(20.0)).red());
        } else { println!("{}", "🛑 Sem dinheiro!".red()); }
    }

    fn dormir(&mut self) {
        self.energia = 100;
        self.fome += 10;
        println!("{}", "😴 Zzzzz... Bateria recarregada.".blue());
    }

    fn consultar_preco_btc() -> Option<(f64, f64)> {
        let url = "https://economia.awesomeapi.com.br/last/BTC-BRL";
        let cliente = reqwest::blocking::Client::builder().user_agent("AthenaBot/1.0").timeout(Duration::from_secs(5)).build().ok()?;
        match cliente.get(url).send() {
            Ok(resp) => {
                if let Ok(dados) = resp.json::<RespostaApi>() {
                    let preco = dados.BTCBRL.bid.parse().unwrap_or(0.0);
                    let var = dados.BTCBRL.pct_change.parse().unwrap_or(0.0);
                    Some((preco, var))
                } else { None }
            },
            Err(_) => None,
        }
    }

    fn ver_mercado_real(&self) {
        println!("\n📡 Consultando preço...");
        if let Some((preco, var)) = Criatura::consultar_preco_btc() {
            let var_fmt = if var >= 0.0 { format!("+{}%", var).green() } else { format!("{}%", var).red() };
            println!("✅ Bitcoin: {} ({})", brl(preco), var_fmt);
            println!("💰 Carteira: {}", brl(self.btc_wallet * preco).green());
        } else { println!("{}", "❌ Falha na conexão.".red()); }
    }

    fn comprar_btc(&mut self, valor: f64) {
        if self.dinheiro < valor { println!("🛑 Saldo insuficiente."); return; }
        if let Some((preco, _)) = Criatura::consultar_preco_btc() {
            let btc = valor / preco;
            self.dinheiro -= valor;
            self.btc_wallet += btc;
            println!("✅ Comprei {:.8} BTC.", btc);
            self.escrever_diario(&format!("Comprei {:.8} BTC", btc));
        } else { println!("❌ Erro de conexão."); }
    }

    fn vender_tudo(&mut self) {
        if self.btc_wallet <= 0.0 { println!("🛑 Nada para vender."); return; }
        if let Some((preco, _)) = Criatura::consultar_preco_btc() {
            let total = self.btc_wallet * preco;
            self.btc_wallet = 0.0;
            self.dinheiro += total;
            println!("✅ Vendi tudo por {}.", brl(total));
            self.escrever_diario(&format!("Vendi tudo por {}", brl(total)));
        } else { println!("❌ Erro de conexão."); }
    }

    fn ler_arquivo(&self, caminho: &str, api_key: &str) {
        println!("{}", format!("📂 Abrindo arquivo '{}'...", caminho).cyan());
        
        match fs::read_to_string(caminho) {
            Ok(conteudo) => {
                let previa = if conteudo.len() > 10000 { &conteudo[..10000] } else { &conteudo };
                let prompt = format!(
                    "Analise o seguinte arquivo que o Pai me enviou. Nome: '{}'. Conteúdo:\n\n---\n{}\n---\n\nFaça um resumo e comentários úteis.", 
                    caminho, previa
                );
                perguntar_gemini(&prompt, api_key, false); 
            },
            Err(_) => println!("{}", "❌ Erro: Não encontrei o arquivo ou não consegui ler.".red()),
        }
    }
}

// --- CÉREBRO GEMINI (COM RETRY AUTOMÁTICO) ---
fn enviar_requisicao(url: &str, corpo: &GeminiRequest) -> Option<String> {
    let cliente = reqwest::blocking::Client::new();
    let mut tentativas = 0;

    // Loop de tentativas (Retry Pattern)
    loop {
        if tentativas > 2 {
            println!("{}", "❌ Desisti após 3 tentativas.".red());
            return None;
        }

        match cliente.post(url).json(corpo).send() {
            Ok(res) => {
                if res.status().is_success() {
                    let texto = res.text().unwrap_or_default();
                    let json: Result<GeminiResponse, _> = serde_json::from_str(&texto);
                    if let Ok(j) = json {
                        if let Some(c) = j.candidates {
                            if !c.is_empty() {
                                return Some(c[0].content.parts[0].text.clone());
                            }
                        }
                    }
                    return None; // JSON vazio
                } else {
                    let erro_texto = res.text().unwrap_or_default();
                    
                    // Se o erro for 429 (Resource Exhausted), esperamos e tentamos de novo
                    if erro_texto.contains("429") || erro_texto.contains("RESOURCE_EXHAUSTED") {
                        println!("{}", "⏳ Cota do Google excedida. Esperando 10 segundos...".yellow());
                        thread::sleep(Duration::from_secs(10));
                        tentativas += 1;
                        continue; // Tenta o loop de novo
                    } else {
                        // Outro erro qualquer
                        println!("\n❌ O GOOGLE RECLAMOU: {}", erro_texto.yellow());
                        return None;
                    }
                }
            },
            Err(e) => {
                println!("Erro de conexão: {}", e);
                return None;
            },
        }
    }
}

fn perguntar_gemini(pergunta_usuario: &str, api_key: &str, usar_busca: bool) {
    if api_key == "COLE_SUA_CHAVE_AQUI" { 
        println!("{}", "⚠️ Configure sua API KEY!".red()); 
        return; 
    }

    if usar_busca {
        println!("{}", format!("🤔 Athena está analisando (Modelo: {})...", MODELO_GOOGLE).magenta().italic());
    } else {
        println!("{}", "🤔 Athena está lendo...".magenta().italic());
    }

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}", MODELO_GOOGLE, api_key);
    let data_hoje = Local::now().format("%Y-%m-%d").to_string();

    // 1. MODO BUSCA
    if usar_busca {
        let prompt_busca = format!("Hoje é {}. Você é Athena. Chame o usuário de 'Pai'. Usuário: '{}'", data_hoje, pergunta_usuario);
        let corpo_busca = GeminiRequest {
            contents: vec![GeminiContent { parts: vec![GeminiPart { text: prompt_busca }] }],
            tools: Some(vec![Tool { google_search: GoogleSearch {} }]),
        };

        if let Some(resposta) = enviar_requisicao(&url, &corpo_busca) {
            println!("\n{} {} {}", "💬 Athena".magenta().bold(), "(via Web):".yellow(), resposta.trim());
            return;
        }
        println!("{}", "⚠️ Busca falhou. Usando memória interna...".yellow().italic());
    }

    // 2. MODO TEXTO / FALLBACK
    let prompt_simples = format!("Hoje é {}. Você é Athena. REGRA MÁXIMA: Trate o usuário como 'Pai'. Seja curta e leal. Usuário: '{}'", data_hoje, pergunta_usuario);

    let corpo_simples = GeminiRequest {
        contents: vec![GeminiContent { parts: vec![GeminiPart { text: prompt_simples }] }],
        tools: None, 
    };

    if let Some(resposta) = enviar_requisicao(&url, &corpo_simples) {
        println!("\n{} {}", "💬 Athena:".magenta().bold(), resposta.trim());
    } else {
        println!("{}", "\n❌ Athena não conseguiu responder.".red());
    }
}

fn rodar_diagnostico(api_key: &str) {
    println!("\n🕵️‍♂️ DIAGNÓSTICO...");
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models?key={}", api_key);
    match reqwest::blocking::get(&url) {
        Ok(res) => {
            let texto = res.text().unwrap_or_default();
            let json: Result<ModelList, _> = serde_json::from_str(&texto);
            if let Ok(lista) = json {
                println!("{}", "✅ MODELOS DISPONÍVEIS:".green());
                if let Some(modelos) = lista.models {
                    for m in modelos {
                        if m.name.contains("gemini") { println!(" - {}", m.name.replace("models/", "").yellow()); }
                    }
                }
            } else { println!("Erro ao ler lista: {}", texto); }
        },
        Err(e) => println!("Erro: {}", e),
    }
}

fn sistema_de_seguranca() {
    let senha_secreta = "1234";
    let mut tentativas = 3;
    println!("{}", "🔒 SISTEMA DE SEGURANÇA".yellow().bold());
    while tentativas > 0 {
        print!("🔑 PIN: ");
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Erro");
        if s.trim() == senha_secreta { println!("{}", "✅ Acesso Liberado.".green()); return; }
        tentativas -= 1;
        println!("❌ Incorreto.");
    }
    process::exit(1);
}

fn main() {
    // ⬇️⬇️⬇️ COLE SUA CHAVE AQUI ⬇️⬇️⬇️
    let api_key = "AIzaSyBFSXjb4EGfob4EmH3J7qN0lUvIYMKEBkQ"; 

    sistema_de_seguranca();
    println!("{} Athena v15.3 (Paciência Automática)...", "🔌 Inicializando".green());
    
    let mut athena = match Criatura::carregar() {
        Some(c) => c,
        None => Criatura::nascer("Athena".to_string())
    };

    loop {
        print!("\n{} > ", "Athena".cyan().bold()); 
        io::stdout().flush().unwrap();
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Erro");
        let linha = entrada.trim();
        let partes: Vec<&str> = linha.split_whitespace().collect();
        if partes.is_empty() { continue; }
        let comando = partes[0].to_lowercase();

        match comando.as_str() {
            "ajuda" => println!("Comandos: status, trabalhar, mercado, comprar [v], vender, ler [arquivo], diagnostico, sair."),
            "status" => athena.status(),
            "trabalhar" => athena.trabalhar(),
            "comer" => athena.comer(),
            "dormir" => athena.dormir(),
            "mercado" => athena.ver_mercado_real(),
            "comprar" => { if partes.len() > 1 { athena.comprar_btc(partes[1].parse().unwrap_or(0.0)); } else { println!("Valor?"); } },
            "vender" => athena.vender_tudo(),
            "diagnostico" => rodar_diagnostico(api_key),
            
            "ler" => {
                let nome_arquivo = linha.strip_prefix("ler ").unwrap_or("").trim();
                if nome_arquivo.is_empty() {
                    println!("Diga o nome do arquivo. Ex: ler Cargo.toml");
                } else {
                    athena.ler_arquivo(nome_arquivo, api_key);
                }
            },

            "sair" => { athena.salvar(); break; },
            _ => {
                perguntar_gemini(linha, api_key, true);
                athena.escrever_diario(&format!("Conversei sobre: {}", linha));
            }
        }
        athena.salvar();
    }
}
