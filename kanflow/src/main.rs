#![allow(non_snake_case)]

use dioxus_router::prelude::*;
use dioxus_desktop::{Config, WindowBuilder};

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    
    dioxus_desktop::launch_cfg(
        app,
            Config::default().with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string()).with_window(WindowBuilder::new().with_resizable(true).with_inner_size(
                dioxus_desktop::wry::application::dpi::LogicalSize::new(1080.0, 720.0),
            )),
    );
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Kanban { id: i32 },
}

#[component]
fn Kanban(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div{
            class: "flex flex-row items-center justify-center h-screen w-screen ",
            section {
                
                class:"text-4xl flex flex-col gap-12 items-center justify-evenly font-bold ",
                    Kanflow_Icon{},
                    p {
                        class: "text-2xl my-12",
                        "Welcome to Kanflow"
                    }

                    input{
                        placeholder: "Username",
                        class:" bg-transparent text-white p-2 my-2",
                    }
                    input{
                        placeholder: "Password",
                        class:" bg-transparent text-white p-2 my-2",
                    }
                    Link { to: Route::Kanban { id: **(count) }, "Go to login " }
                }
        }
    })
}


pub fn Kanflow_Icon(cx: Scope) -> Element {
    cx.render(rsx!(
        svg {
            width: "404",
            height: "195",
            view_box: "0 0 404 195",
            fill: "none", 
            xmlns: "http://www.w3.org/2000/svg",
            rect {
                width: "404",
                height: "195",
                rx: "15",
                fill: "#88C800"
            },
            path {
                d: "M44 0H141V130C141 148.778 125.778 164 107 164H78C59.2223 164 44 148.778 44 130V0Z",
                fill: "#1E2530"
            },
            path {
                d: "M264 0H361V130C361 148.778 345.778 164 327 164H298C279.222 164 264 148.778 264 130V0Z",
                fill: "#1E2530"
            },
            path {
                d: "M154 0H251V67C251 85.7777 235.778 101 217 101H188C169.222 101 154 85.7777 154 67V0Z",
                fill: "#1E2530"
            }
        }
    ))
}