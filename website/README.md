# website

## Develop

> [!NOTE]
> 全文検索のインデックスは[pagefind](https://pagefind.app/)で生成していますが、インデックスの出力先が`dist/`になっているため、現在は開発サーバーで全文検索が機能しません。検索機能関連の開発をする場合は、`bun run build`を実行してから、`bun run preview`でビルド後の状態を確認してください。

```sh
bun run dev
```

## Build

```sh
bun run build
```

## Preview

```sh
bun run preview
```
