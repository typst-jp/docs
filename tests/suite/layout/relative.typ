--- relative-fields ---
// Test relative length fields.
#test((100% + 2em + 2pt).ratio, 100%)
#test((100% + 2em + 2pt).length, 2em + 2pt)
#test((100% + 2pt).length, 2pt)
#test((100% + 2pt - 2pt).length, 0pt)
#test((56% + 2pt - 56%).ratio, 0%)

<<<<<<< HEAD
--- double-percent ---
// Test for two percent signs in a row.
#3.1%%

--- double-percent-error ---
// Error: 7-8 the character `%` is not valid in code
=======
--- double-percent-embedded ---
// Test for two percent signs in a row.
// Error: 2-7 invalid number suffix: %%
#3.1%%

--- double-percent-parens ---
// Error: 3-8 invalid number suffix: %%
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#(3.1%%)
