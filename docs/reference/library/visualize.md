<<<<<<< HEAD
描画とデータの可視化。

より高度な図やプロットを作成したい場合は、
[CeTZ](https://github.com/johannes-wolf/cetz)パッケージや、
用途に合わせた、より専門的な[パッケージ]($universe)も参照してください。

# アクセシビリティ { #accessibility }

Typstが描画する全ての図形とパスは、PDFエクスポート時に支援技術（Assistive Technology、AT）から認識されないよう、
[アーティファクト]($pdf.artifact)として自動的にマークされます。
ただし、その内容（存在する場合）はアクセシブルなままです。

このカテゴリーの関数を使ってセマンティックな意味を持つ図を作成する場合は、
[`figure`]関数で囲んでアクセシブルにしてください。
[テキストによる表現]($guides/accessibility/#textual-representations)を提供するには、
`figure`関数の[`alt`パラメーター]($figure.alt)で指定します。
=======
Drawing and data visualization.

If you want to create more advanced drawings or plots, also have a look at the
[CeTZ](https://github.com/johannes-wolf/cetz) package as well as more
specialized [packages]($universe) for your use case.

# Accessibility

All shapes and paths drawn by Typst are automatically marked as
[artifacts]($pdf.artifact) to make them invisible to Assistive Technology (AT)
during PDF export. However, their contents (if any) remain accessible.

If you are using the functions in this model to create an illustration with
semantic meaning, make it accessible by wrapping it in a [`figure`] function
call. Use its [`alt` parameter]($figure.alt) to provide an
[alternative description]($guides/accessibility/#textual-representations).
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
