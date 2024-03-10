# Chess Fitz

A chess engine designed to imitate human players.

- Inspired by [Maia](https://maiachess.com/)
- Using data from [lichess.org](https://database.lichess.org/)
- Input encoding taken from LeelaChessZero (see [encoder.c](https://github.com/LeelaChessZero/lc0/blob/master/src/neural/encoder.cc))
- Output encoding also from [LC0 (Classic Head)](https://lczero.org/dev/backend/nn/#format-policy_classical)
- Network topology to be decided, most likely LC0 or AlphaZero inspired

Everything except for the Neural Network itself is implemented at the moment.


### Preliminary usage

Prepare data:
`cargo run -r -p prepare_data`

Run / Train:
`cargo run -r`


## Good references

- https://www.chessprogramming.org/AlphaZero
- https://chrisbutner.github.io/ChessCoach/technical-explanation.html
