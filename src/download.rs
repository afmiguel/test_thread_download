use std::fs::File;
use std::path::Path;
use reqwest::blocking::Response; // Importação explícita para clareza
use reqwest::StatusCode; // Importado para usar reqwest::StatusCode::NOT_FOUND

/// Realiza o download de um arquivo a partir de uma URL especificada e o salva
/// no diretório local "downloads".
///
/// O diretório "downloads" será criado se ainda não existir.
///
/// # Argumentos
///
/// * `url`: Uma string (`&str`) que representa a URL base de onde o arquivo será baixado.
/// * `filename`: Uma string (`&str`) que representa o nome do arquivo a ser baixado.
///   Este nome também será usado para salvar o arquivo localmente no diretório "downloads".
///
/// # Panics
///
/// Esta função entrará em pânico (`panic!`) em diversas situações:
///
/// * Se houver uma falha ao enviar a requisição GET inicial (ex: erro de DNS,
///   falha de conexão de rede).
/// * Se o servidor responder com um status de erro HTTP (4xx ou 5xx).
///     * Especificamente para um erro 404 (Not Found), uma mensagem customizada de pânico será exibida.
///     * Para outros erros HTTP, uma mensagem de pânico detalhando o status e o erro será mostrada.
/// * Se houver falha ao criar o diretório "downloads".
/// * Se houver falha ao criar o arquivo local onde o conteúdo será salvo.
/// * Se houver falha ao copiar o conteúdo da resposta HTTP para o arquivo local.
///
/// # Exemplos
///
/// ```no_run
/// // Supondo que esta função esteja acessível (ex: no mesmo módulo ou importada)
/// // fn download_file(url: &str, filename: &str) { /* ... */ }
///
/// fn main() {
///     // Exemplo de download de um arquivo (substitua com uma URL e arquivo válidos para teste)
///     // Este exemplo provavelmente falhará se o arquivo não existir, causando um pânico.
///     // download_file("[https://exemplo.com/dados](https://exemplo.com/dados)", "meuarquivo.txt");
///
///     // Exemplo com um arquivo que pode existir (usado para testes de API pública)
///     // Note que "todos/1" será salvo como "1" no diretório "downloads".
///     // download_file("[https://jsonplaceholder.typicode.com](https://jsonplaceholder.typicode.com)", "todos/1");
/// }
/// ```
pub fn download_file(url: &str, filename: &str) {
    // Constrói a URL completa do arquivo combinando a URL base e o nome do arquivo.
    let file_url = format!("{}/{}", url, filename);

    // Envia uma requisição GET bloqueante para o servidor para obter o arquivo.
    // A chamada é bloqueante, o que significa que a thread atual esperará pela resposta.
    let response_result: Result<Response, reqwest::Error> = reqwest::blocking::get(&file_url);

    // 1. Trata erros potenciais na própria requisição HTTP (antes de obter uma resposta).
    //    Isso inclui erros de rede, falhas de DNS, etc.
    let http_response: Response = match response_result {
        Ok(r) => {
            // Requisição bem-sucedida em termos de comunicação, recebemos uma resposta.
            r
        }
        Err(e) => {
            // A requisição falhou em um nível fundamental (ex: rede).
            // Entra em pânico com uma mensagem de erro detalhada.
            panic!(
                "Falha ao enviar requisição GET para a URL '{}': {}",
                file_url, e
            );
        }
    };

    // 2. Verifica o status HTTP da resposta recebida.
    //    O método `error_for_status()` consome a `http_response` e retorna:
    //    - `Ok(Response)` se o status HTTP for de sucesso (2xx).
    //    - `Err(reqwest::Error)` se o status HTTP for de erro (4xx ou 5xx).
    //      Neste caso, o `reqwest::Error` conterá informações sobre o status de erro.
    let mut successful_response: Response = match http_response.error_for_status() {
        Ok(resp_ok) => {
            // O status HTTP indica sucesso (ex: 200 OK).
            // A `resp_ok` é a mesma resposta, agora confirmada como bem-sucedida.
            resp_ok
        }
        Err(err_with_status) => {
            // O servidor respondeu com um código de status de erro HTTP.
            // `err_with_status` é um `reqwest::Error` que encapsula este erro de status.
            if err_with_status.status() == Some(StatusCode::NOT_FOUND) {
                // Trata especificamente o erro 404 (Not Found).
                panic!(
                    "Arquivo '{}' não encontrado na URL '{}'. O servidor retornou 404 Not Found.",
                    filename, file_url
                );
            } else {
                // Trata outros erros HTTP (4xx ou 5xx).
                panic!(
                    "Erro HTTP ao tentar baixar o arquivo '{}' da URL '{}'. Status: {:?}. Detalhes: {}",
                    filename,
                    file_url,
                    err_with_status.status(), // Exibe o código de status (ex: Some(500))
                    err_with_status           // Exibe os detalhes completos do reqwest::Error
                );
            }
        }
    };

    // Define o nome do diretório onde os arquivos baixados serão salvos.
    let download_dir_name = "downloads";
    let download_path = Path::new(download_dir_name);

    // Cria o diretório "downloads" se ele ainda não existir.
    // `create_dir_all` cria todos os diretórios pais necessários e não falha se o diretório já existir.
    // Entra em pânico se houver uma falha na criação do diretório (ex: permissões).
    if let Err(e) = std::fs::create_dir_all(download_path) {
        panic!(
            "Falha ao criar o diretório '{}': {}",
            download_path.display(),
            e
        );
    }

    // Define o caminho completo para o arquivo local, incluindo o diretório "downloads".
    let local_file_path = download_path.join(filename);

    // Cria (ou sobrescreve, se já existir) o arquivo local onde o conteúdo será salvo.
    // Entra em pânico se houver falha na criação do arquivo (ex: permissões, caminho inválido).
    let mut local_file = match File::create(&local_file_path) {
        Ok(file) => file,
        Err(e) => {
            panic!(
                "Falha ao criar o arquivo local '{}': {}",
                local_file_path.display(),
                e
            );
        }
    };

    // Copia o conteúdo da resposta HTTP (que foi confirmada como bem-sucedida)
    // para o arquivo local. A função `io::copy` lê de `successful_response`
    // (que implementa `Read`) e escreve em `local_file` (que implementa `Write`).
    // Entra em pânico se houver erro durante a cópia (ex: disco cheio, conexão interrompida).
    match std::io::copy(&mut successful_response, &mut local_file) {
        Ok(bytes_copied) => {
            // Conteúdo copiado com sucesso. Imprime uma mensagem de sucesso.
            println!(
                "Download do arquivo '{}' para '{}' ({} bytes) concluído com sucesso!",
                filename,
                local_file_path.display(),
                bytes_copied
            );
        }
        Err(e) => {
            panic!(
                "Falha ao copiar o conteúdo baixado para o arquivo '{}': {}",
                local_file_path.display(),
                e
            );
        }
    }
}

/*
// Exemplo de como usar a função em um `main`
fn main() {
    println!("Iniciando o processo de download...");

    // Teste 1: Tentar baixar um arquivo que provavelmente não existe (deve causar pânico com 404)
    println!("\nTentativa 1: Baixando um arquivo inexistente...");
    // Para observar o comportamento sem parar o programa, você precisaria
    // de `std::panic::catch_unwind` ou modificar `download_file` para retornar `Result`.
    // download_file("https://jsonplaceholder.typicode.com", "arquivo-que-nao-existe-12345.txt");
    // Se a linha acima for descomentada, o programa provavelmente parará aqui.

    // Teste 2: Tentar baixar um arquivo de uma URL base inválida (deve causar pânico na conexão)
    // println!("\nTentativa 2: Baixando de uma URL base inválida...");
    // download_file("https://dominio-inexistente-e-com-certeza-nao-funciona.com", "qualquercoisa.txt");
    // Se a linha acima for descomentada, o programa provavelmente parará aqui.

    // Teste 3: Tentar baixar um arquivo que deve existir
    println!("\nTentativa 3: Baixando um arquivo que deve existir...");
    download_file("https://jsonplaceholder.typicode.com", "todos/1");
    // Se bem-sucedido, você encontrará um arquivo chamado "1" no diretório "downloads".

    println!("\nProcesso de download (ou tentativas) finalizado.");
}
*/