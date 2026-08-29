<<<<<<< HEAD
外部ファイルからのデータの読み込み。

これらの関数は、
例えば実験結果のようなデータの読み込みと埋め込みを支援します。

# エンコーディング { #encoding }
[`cbor.encode`]($cbor.encode)などの一部の関数はエンコードも可能です。
これらを使えば、構造化データを[プラグイン]($plugin)に渡しやすくなります。

ただし、各データ形式は、それぞれ独自の型を持っています。
そのため、Typstにおける任意の値については、エンコードとデコードの往復で情報を失う可能性があります。
一般に、数値、文字列、およびそれらから構成される[配列]($array)や[辞書]($dictionary)は確実に変換できますが、
その他の型は[`repr`]($repr)経由で文字列にフォールバックされる場合があります。これは[デバッグ目的のみ]($repr/#debugging-only)に使用するものです。
詳細は各データ形式のページを参照してください。
=======
Data loading from external files.

These functions help you with loading and embedding data, for example from the
results of an experiment.

# Encoding
Some of the functions are also capable of encoding, e.g. [`cbor.encode`]. They
facilitate passing structured data to [plugins]($plugin).

However, each data format has its own native types. Therefore, for an arbitrary
Typst value, the encode-to-decode roundtrip might be lossy. In general, numbers,
strings, and [arrays]($array) or [dictionaries]($dictionary) composed of them
can be reliably converted, while other types may fall back to strings via [`repr`],
which is [for debugging purposes only]($repr/#debugging-only). Please refer to
the page of each data format for details.
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
