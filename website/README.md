# website metadata

このディレクトリでは、ドキュメントのWebサイトの構築に必要なメタデータを管理しています。

また、SSG（静的サイトジェネレーター）の本体は、Git submoduleとして別リポジトリの[typst-docs-web](https://github.com/typst-community/typst-docs-web)で管理されています。

## Git submoduleの初期化について

リポジトリを`git clone`する際に`--recursive`オプションを付けていない場合は、以下のコマンドでsubmoduleを初期化・更新できます。

```sh
git submodule update --init --recursive
```

これにより、`typst-docs-web`ディレクトリが正しく取得されます。
