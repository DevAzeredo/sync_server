use actix_web::{web, HttpRequest, HttpResponse};
use std::fs::File;
use std::io::prelude::*;

pub async fn post_file(req: HttpRequest, body: web::Bytes) -> HttpResponse {
    let res: HttpResponse;
    let filename = "example.jpg"; // nome do arquivo
    let filepath = format!(r"D:\RUST\sync_server\src\arquivos/{}", filename); // caminho do arquivo
    let mut file = match File::create(filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("Erro ao criar o arquivo: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Grava o corpo da requisição no arquivo
    match file.write_all(&body) {
        Ok(_) => println!("Arquivo salvo com sucesso!"),
        Err(e) => {
            println!("Erro ao salvar o arquivo: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().body(format!("Arquivo {} salvo com sucesso!", filename))
}

fn is_client_valid(client: &str) -> bool {
    if client == "" {
        return false;
    } else {
        return true;
    }
}

/*vArrayClientes := TClientes.ObterInstancia;

  LStream := Req.Body<TMemoryStream>;
  vArquivo := TArquivo.Create;

  vErro := vArquivo.ValidarBody(LStream);
  writeln('Recebendo Arquivo');
  try
    if vErro = '' then
      begin
        vCliente := Req.Headers.Field('cliente').AsString;
        vErro := vArquivo.ValidarHeader(vCliente, vArrayClientes);

        if vErro = '' then
          Res.Send(vArquivo.CriarArquivo(LStream, vCliente, vArrayClientes)).Status(201)
        else
          Res.Send(vErro).Status(400);
      end
    else
      Res.Send(vErro).Status(415);
  finally
    FreeAndNil(vArquivo);
  end;
end;*/
