// Test case distinction.

--- math-cases ---
$ f(x, y) := cases(
  1 quad &"if" (x dot y)/2 <= 0,
  2 &"if" x divides 2,
  3 &"if" x in NN,
  4 &"else",
) $

--- math-cases-gap ---
#set math.cases(gap: 1em)
$ x = cases(1, 2) $

--- math-cases-delim ---
<<<<<<< HEAD
#set math.cases(delim: sym.angle.l)
$ cases(a, b, c) $

--- math-cases-linebreaks ---
// Currently linebreaks are equivalent to commas, though this behaviour may
// change in the future.
=======
#set math.cases(delim: sym.chevron.l)
$ cases(a, b, c) $

--- math-cases-linebreaks ---
// Warning: 40-49 linebreaks are ignored in branches
// Hint: 40-49 use commas instead to separate each line
>>>>>>> dd1e6e94f73db6a257a5ac34a6320e00410a2534
$ cases(a, b, c) cases(reverse: #true, a \ b \ c) $
