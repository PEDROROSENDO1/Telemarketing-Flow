use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, EventControllerKey};



use webkit6::{WebView};
use webkit6::prelude::WebViewExt;

const APP_ID: &str = "com.telemarketing.flow";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Telemarketing Flow")
        .default_width(1024)
        .default_height(768)
        .build();

    let webview = WebView::new();
    
    // Embutindo o HTML e CSS diretamente no binário durante a compilação
    let html_content = include_str!("../index.html");
    let css_content = include_str!("../css/style.css");
    
    // Substitui a tag <link> pelo bloco <style> com o CSS embutido
    let final_html = html_content.replace(
        "<link rel=\"stylesheet\" href=\"css/style.css\" />",
        &format!("<style>\n{}\n</style>", css_content)
    );
    
    // Carrega o conteúdo HTML embutido. Usamos "http://localhost/" como base_uri padrão.
    webview.load_html(&final_html, Some("http://localhost/"));

    // Cria um controlador de evento para teclas
    let key_controller = EventControllerKey::new();

    // Clona a referência da janela para ser usada na closure do evento de teclado
    let window_clone = window.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, _| {
        // Verifica se a tecla pressionada é F11
        // `gdk4::Key::F11` representa o código da tecla F11
        if keyval == gdk4::Key::F11 {
            // Verifica o estado atual de fullscreen da janela
            if window_clone.is_fullscreen() {
                // Se estiver em tela cheia, sai da tela cheia
                window_clone.unfullscreen();
            } else {
                // Se não estiver em tela cheia, entra em tela cheia
                window_clone.fullscreen();
            }
            // Retorna um valor booleano indicando se o evento foi tratado.
            // Retornar `glib::Propagation::Stop` impede que o evento se propague para outros manipuladores.
            glib::Propagation::Stop
        } else {
            // Se não for F11, permite que o evento se propague.
            glib::Propagation::Proceed
        }
    });

    // Adiciona o controlador de evento à janela
    window.add_controller(key_controller);

    window.set_child(Some(&webview));
    window.present();
}