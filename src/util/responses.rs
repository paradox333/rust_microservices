use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Response<T> {
    pub status: u16,
    pub message: String,
    pub result: Option<T>,
}

pub struct ResponseBuilder<T> {
    status: u16,
    message: Option<String>,
    result: Option<T>,
}

impl<T> ResponseBuilder<T> {
    // Constructor del builder
    pub fn new(status: u16) -> Self {
        Self {
            status,
            message: None,
            result: None,
        }
    }

    // Método para definir el mensaje dinámicamente
    pub fn message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }

    // Método para definir los datos opcionalmente
    pub fn result(mut self, result: T) -> Self {
        self.result = Some(result);
        self
    }

    // Método para construir la respuesta final
    pub fn build(self) -> Response<T> {
        Response {
            status: self.status,
            message: self.message.unwrap_or_else(|| "No message provided".to_string()),
            result: self.result,
        }
    }
}
