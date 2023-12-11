This repo shows an issue with using futures with dioxus.
In example there are two pages, in the first page the user
clicks a button which changes the page to the second page and
spawns a future. The future gets spawned but never completes.
It does not matter if the future is spawned by `spawn` or `spawn_forever`.

You can run the example with::

  $ dx serve --hot-reload --port 8000
