--- json ---
// Test reading JSON data.
#let data = json("/assets/data/zoo.json")
#test(data.len(), 3)
#test(data.at(0).name, "Debby")
#test(data.at(2).weight, 150)

--- json-invalid ---
<<<<<<< HEAD
// Error: 7-30 failed to parse JSON (expected value at line 3 column 14)
=======
// Error: "/assets/data/bad.json" 3:14 failed to parse JSON (expected value at line 3 column 14)
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#json("/assets/data/bad.json")

--- json-decode-deprecated ---
// Warning: 15-21 `json.decode` is deprecated, directly pass bytes to `json` instead
<<<<<<< HEAD
=======
// Hint: 15-21 it will be removed in Typst 0.15.0
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#let _ = json.decode

--- issue-3363-json-large-number ---
// Big numbers (larger than what i64 can store) should just lose some precision
// but not overflow
#let bignum = json("/assets/data/big-number.json")
#bignum
<<<<<<< HEAD
=======

--- json-decode-number ---
#import "edge-case.typ": large-integer, representable-integer

#for (name, source) in representable-integer {
  assert.eq(
    type(json(bytes(source))),
    int,
    message: "failed to decode " + name,
  )
}

#for (name, source) in large-integer {
  assert.eq(
    type(json(bytes(source))),
    float,
    message: "failed to approximately decode " + name,
  )
}

--- json-encode-any ---
#import "edge-case.typ": special-types-for-human
#for value in special-types-for-human {
  test(
    json.encode(value),
    json.encode(repr(value)),
  )
}
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
