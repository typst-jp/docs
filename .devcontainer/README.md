# ローカル環境を構築するDockerfile

[Docker](https://docs.docker.com/)を用いてWebページの仕上がりを確認できます。
以下の操作はDockerがインストール済み、かつDockerデーモンを起動していることが前提となります。

## VS Codeを使用している場合

[Dev Container](https://code.visualstudio.com/docs/devcontainers/containers)を使用します。
Visual Studio Codeでtypst-jp.github.ioディレクトリを開き、以下の操作を実施してください。

1. Ctrl+Shift+Pから`> Dev Containers: Reopen in Container`を実行
2. Webサーバーが起動したらブラウザで http://localhost:5173 に接続
3. ページを更新した際には、Ctrl+Shift+Bでビルドを実行
4. 体裁を確認したい場合、Ctrl+Shift+Pから`> Tasks: Run task`を実行し`textlint-md`（markdownファイルを翻訳した場合）または`textlint-html`（Rustソースコードを翻訳した場合）を選択
5. 自動修正を実施したい場合、markdownファイルの添削であれば、同様に`textlint-md:fix`を選択（Rustコードは対応していなため、該当箇所を手動で修正してください。）
