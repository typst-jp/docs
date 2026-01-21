--- smallcaps ---
// Test smallcaps.
#smallcaps[Smallcaps]

--- smallcaps-show-rule ---
// There is no dedicated smallcaps font in typst-dev-assets, so we just use some
// other font to test this show rule.
#show smallcaps: set text(font: "PT Sans")
#smallcaps[Smallcaps]

#show smallcaps: set text(fill: red)
#smallcaps[Smallcaps]

<<<<<<< HEAD
--- smallcaps-all ---
=======
--- smallcaps-all render html ---
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
#smallcaps(all: false)[Test 012] \
#smallcaps(all: true)[Test 012]
