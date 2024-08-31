# Dev Containerについて

Visual Studio Codeで編集する場合には、[Dev Container](https://code.visualstudio.com/docs/devcontainers/containers)を使用してローカル環境でWebページの仕上がりを確認することができます。
Visual Studio Codeでtypst-jp.github.ioディレクトリを開き以下の操作を実施してください。
1. Ctrl+Shift+Pから`>Dev Containers: Reopen in Container`を実行
2. ビルドが完了したらブラウザで http://localhost:3000 に接続
3. ページを更新した際には、Ctrl+shift+Pから`Tasks: Run task`を実行し`preview: typst-jp documentation`を選択。ビルドが完了したらブラウザを更新。


## 別のエディターを使用している場合

別のエディターで編集している場合にもDockerfileの使用のみであれば可能です。
ターミナルから typst-jp.github.io ディレクトリ上で以下のコマンドを実行してください。
1. Docker imageをビルドして実行
    ```
    docker build . -f .devcontainer/Dockerfile -t typst-jp-doc
    docker run --name typst-jp-doc -p 3000:3000 -it -v "$(pwd):/workspace" -w /workspace --rm typst-jp-doc /bin/bash
    ```
2. Dockerコンテナ内でビルド
    ```
    cargo test --package typst-docs --lib -- tests::test_docs --exact --nocapture && python3 ./gen.py && npx serve -n ./dist
    ```
3. ビルドが完了したらブラウザで http://localhost:3000 に接続
4. ファイルを更新した際には、2 のコマンドを再度実行して、ブラウザを更新。
