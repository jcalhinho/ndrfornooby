#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    // Vos états existants ici
    let mut count = use_signal(|| 0);
    

    rsx! {
        div {
            class: "text-black ", // Classe Tailwind pour la navbar
            div {
                class: "flex bg-gray-800 text-white  justify-between items-center h-[58px] ",
                div {
                    class: "text-lg bg-red-400",
                    "Dioxus App" // Titre ou logo de l'application
                }
                ul {
                    class: "flex gap-4 ",
                    li { Link { to: Route::Home {}, class: "hover:bg-gray-700   rounded", "Home" } }
                    li { Link { to: Route::Blog { id: count() }, class: "hover:bg-gray-700 rounded", "Blog" } }
                }
            }
            // Le contenu existant de Home
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
           
        }
    }
}
