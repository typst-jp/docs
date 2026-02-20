描画とデータの可視化。

より高度な図やプロットを作成したい場合は、
[CeTZ](https://github.com/johannes-wolf/cetz)パッケージや、
あなたのユースケースに合わせた、より専門的な[パッケージ]($universe)も参照してください。

# アクセシビリティ { #accessibility }

Typstが描画する全ての図形とパスは、PDFエクスポート時に支援技術（AT）から見えなくするために
[アーティファクト]($pdf.artifact)として自動的にマークされます。
ただし、その内容（存在する場合）はアクセシブルなままです。

このモデルの関数を使って意味論的意味を持つ図を作成する場合は、
[`figure`]関数呼び出しで包むことでアクセシブルにしてください。
[代替説明]($guides/accessibility/#textual-representations)を提供するには、
figure関数の [alt パラメーター]($figure.alt)で指定します。
