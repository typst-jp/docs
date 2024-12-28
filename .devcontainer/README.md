# ローカル環境を構築するDockerfile

[Docker](https://docs.docker.com/)を用いてWebページの仕上がりを確認できます。
以下の操作はDockerがインストール済み、かつDockerデーモンを起動していることが前提となります。

## VS Codeを使用している場合

[Dev Container](https://code.visualstudio.com/docs/devcontainers/containers)を使用します。
Visual Studio Codeでtypst-jp.github.ioディレクトリを開き、以下の操作を実施してください。

1. Ctrl+Shift+Pから`> Dev Containers: Reopen in Container`を実行
2. Webサーバーが起動したらブラウザで http://localhost:8000 に接続
3. ページを更新した際には、Ctrl+Shift+Pから`> Tasks: Run task`を実行し`generate-docs`を選択、ビルドが完了したらブラウザを更新
4. 体裁を確認したい場合、同様に`> Tasks: Run task`を実行し`textlint-md`（markdownファイルを翻訳した場合）または`textlint-html`（Rustソースコードを翻訳した場合）を選択
5. 自動修正を実施したい場合、markdownファイルの添削であれば、同様に`textlint-md:fix`を選択（Rustコードは対応していなため、該当箇所を手動で修正してください。）


## 別のエディターを使用している場合

ターミナルからDockerfileをビルドして、コマンド実行します。
typst-jp.github.io ディレクトリ上で以下のコマンドを実行してください。

1. Docker imageをビルドしてコンテナを作成
    ```
    docker build . -f .devcontainer/Dockerfile -t typst-jp-doc
    docker run --name typst-jp-doc -p 8000:8000 -it -v "$(pwd):/workspace" -w /workspace --rm typst-jp-doc /bin/bash
    ```
2. Dockerコンテナ内でページを生成
    ```
    cargo test --package typst-docs --lib -- tests::test_docs --exact --nocapture && python3 ./gen.py && npx serve -n ./dist
    ```
3. Webサーバーが起動したらブラウザで http://localhost:8000 に接続
4. ファイルを更新した際には、2 のコマンドを一旦 Ctrl+C で終了して再度実行、その後ブラウザを更新
5. 体裁を確認したい場合には、初めの一回だけ`bun install --frozen-lockfile`を実行した後、`bun run textlint-md`や`bun run textlint-html`を実行
6. markdownファイルの体裁を修正したい場合には`textlint-md:fix`を実行
