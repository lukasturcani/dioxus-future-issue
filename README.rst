This repo shows an issue with using futures with dioxus.
In the example there are two pages, in the first page the user
clicks a button which changes the page to the second page and
spawns a future. The future gets spawned but never completes.
It does not matter if the future is spawned by ``spawn`` or
``spawn_forever``.

If you flatten the ``First`` component - ie remove it and just
write its contents directly into ``App`` like

.. code-block:: rust

  #[component]
  fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Page::First);
    let page = use_shared_state::<Page>(cx).unwrap();
    cx.render(rsx! {
        match *page.read() {
            Page::First => rsx! {
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
            },
            Page::Second => rsx! {
                div {
                    "Second page"
                }
            }
        }
    })
  }


the issue goes away


You can run the example with::

  $ dx serve --hot-reload --port 8000
