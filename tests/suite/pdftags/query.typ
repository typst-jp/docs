--- query-tags-duplicate-heading pdftags ---
// This will display the heading with the same location a second time
#context query(heading).join()
= Hi

--- query-tags-duplicate-labelled-element pdftags ---
#figure(alt: "Text saying: hello there")[
  hello there
] <figure>

#context query(<figure>).at(0)

<<<<<<< HEAD
--- query-tags-ambiguous-parent-place pdftags ---
=======
--- query-tags-ambiguous-parent-place-error pdftags ---
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
// Error: 2-43 PDF/UA-1 error: ambiguous logical parent
// Hint: 2-43 please report this as a bug
#place(float: true, top + left)[something] <placed>

#context query(<placed>).join()

<<<<<<< HEAD
--- query-tags-ambiguous-parent-footnote pdftags ---
=======
--- query-tags-ambiguous-parent-place pdftags nopdfua ---
#place(float: true, top + left)[something] <placed>

#context query(<placed>).join()

--- query-tags-ambiguous-parent-footnote-error pdftags ---
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
// Error: 1:2-1:21 PDF/UA-1 error: ambiguous logical parent
// Hint: 1:2-1:21 please report this as a bug
#footnote[something] <note>

#context query(<note>).join()
<<<<<<< HEAD
=======

--- query-tags-ambiguous-parent-footnote pdftags nopdfua ---
#footnote[something] <note>

#context query(<note>).join()
>>>>>>> eb2027e55f17a91cc2025c7a71674a2c5ea3a363
