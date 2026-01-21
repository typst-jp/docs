<<<<<<< HEAD
外部ファイルからのデータの読み込み。

これらの関数は、
例えば実験結果のようなデータの読み込みと埋め込みを支援します。
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
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
