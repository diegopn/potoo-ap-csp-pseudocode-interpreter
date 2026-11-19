# AP CSP pseudocode reference coverage

This crate targets the textual pseudocode on the College Board's 2026 AP Computer Science Principles exam reference sheet. It does not provide a block editor or define a particular robot grid.

| Reference area | Implemented behavior | Runtime boundary |
| --- | --- | --- |
| Assignment, display, and input (pp. 1–2) | Assignment copies evaluated values; `DISPLAY` forwards one value to the host; `INPUT()` obtains one value from the host | The CLI reads one line per input call and prints the value followed by a space. |
| Arithmetic and numeric procedures (p. 1) | `+`, `-`, `*`, `/`, `MOD`, and inclusive `RANDOM(a,b)`; integer and range constraints are checked | Random-number quality and source of randomness are supplied by the host. |
| Relational and Boolean operators (pp. 1–2) | `=`, `≠`, `>`, `<`, `≥`, `≤`, `NOT`, `AND`, and `OR`, with short-circuit evaluation for `AND` and `OR` | Ordered comparisons require two numbers or two strings of the same type. |
| Selection and iteration (pp. 2–3) | `IF` / `ELSE`, `REPEAT n TIMES`, and post-body `REPEAT UNTIL(condition)` | A configurable execution-step limit prevents nonterminating programs from running indefinitely. |
| Lists (pp. 3–4) | Empty and nonempty literals, copy assignment, one-based element reads/writes, `INSERT`, `APPEND`, `REMOVE`, `LENGTH`, and `FOR EACH`; invalid indices terminate with an error | Values are dynamically typed. `FOR EACH` visits a snapshot of the list at loop entry because mutation during iteration is unspecified. In a nonempty loop, the item variable retains the last assigned element after the loop. |
| Procedures (p. 5) | Procedure declarations, value parameters, calls, `RETURN(expression)`, recursion, and returning a procedure value into an assignment | The sheet does not define local/global name resolution; the implementation uses local parameters and local variables when no global of that name exists. |
| Robot (pp. 5–6) | `MOVE_FORWARD()`, `ROTATE_LEFT()`, `ROTATE_RIGHT()`, and `CAN_MOVE(direction)` for left/right/forward/backward | The host supplies the grid. A blocked forward move leaves position unchanged and terminates execution as the sheet specifies. |

The parser accepts the textual notation only. Robot actions require a `RobotGrid` implementation; the CLI intentionally reports an error if a robot program runs without one. The crate targets the 2026 sheet and does not infer additional language rules from later exam materials.

## Source

- [College Board AP CSP 2026 Exam Reference Information](https://apcentral.collegeboard.org/media/pdf/ap-computer-science-principles-exam-reference-sheet.pdf).
