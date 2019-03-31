use nomenclate_http::NomenclateHttpClient;

pub fn setup() -> NomenclateHttpClient {
    NomenclateHttpClient::new("http://localhost:8080")
}
