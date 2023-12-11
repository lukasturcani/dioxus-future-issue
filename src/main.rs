use dioxus::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App)
}

enum Page {
    First,
    Second,
}

#[component]
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Page::First);
    let page = use_shared_state::<Page>(cx).unwrap();
    cx.render(rsx! {
        match *page.read() {
            Page::First => rsx! {
                First{}
            },
            // THIS IS FINE!!!
            // Page::First => rsx! {
            //     div {
            //         "First page"
            //         button {
            //             onclick: |_| {
            //                 *page.write() = Page::Second;
            //                 // cx.spawn(do_async_stuff());
            //                 // cx.spawn_forever(do_async_stuff());
            //                 do_async_stuff()
            //             },
            //             "Go to second page"
            //         }
            //     }
            // },
            Page::Second => rsx! {
                div {
                    "Second page"
                }
            }
        }
    })
}

#[component]
fn First(cx: Scope) -> Element {
    let page = use_shared_state::<Page>(cx).unwrap();
    cx.render(rsx! {
        div {
            "First page"
            button {
                onclick: |_| {
                    *page.write() = Page::Second;
                    // cx.spawn(do_async_stuff());
                    // cx.spawn_forever(do_async_stuff());
                    do_async_stuff()
                },
                "Go to second page"
            }
        }
    })
}

async fn do_async_stuff() {
    log::info!("Doing async stuff");
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    log::info!("Done doing async stuff");
}
