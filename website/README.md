# website metadata

このディレクトリでは、ドキュメントのWebサイトの構築に必要なメタデータを管理しています。

また、SSG（静的サイトジェネレーター）の本体は、Git submoduleとして別リポジトリの[typst-docs-web](https://github.com/typst-community/typst-docs-web)で管理されています。

## Git submoduleの初期化

リポジトリを`git clone`する際に`--recursive`オプションを付けていない場合は、以下のコマンドでsubmoduleを初期化・更新できます。

```sh
git submodule update --init --recursive
```

これにより、`typst-docs-web`ディレクトリが正しく取得されます。

## Git submoduleの更新

submoduleの更新は以下の手順で行います。

```sh
# submoduleをリモートの最新状態に更新
git submodule update --remote website/typst-docs-web

# 変更をコミット
git add website/typst-docs-web
git commit -m "chore: update typst-docs-web"
```
