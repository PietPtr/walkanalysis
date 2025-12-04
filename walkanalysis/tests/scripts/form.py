import math
from typing import Tuple
from pprint import pprint


def f(frequency: float) -> Tuple[int, float]:
    note_index = (math.log10(frequency / 55.0) * 12.0) / math.log10(2.0)
    return int(round(note_index)), (note_index + 0.5) % 1.0 - 0.5


def note_to_freq(n) -> float:
    return 2 ** (n / 12) * 55


def main() -> None:
    print(f(49.732185))
    return

    records = {}
    for i in range(200, 2000):
        freq = i / 10.0
        (note_index, error) = f(freq)
        print(f"{freq:.1f} {(error)}")

        try:
            print(note_index, records[note_index])
            if abs(error) < records[note_index][0]:
                records[note_index][0] = abs(error)
                records[note_index][3] = freq

            records[note_index][1] = (
                error if error > records[note_index][1] else records[note_index][1]
            )

            records[note_index][2] = (
                error if error < records[note_index][2] else records[note_index][2]
            )

        except KeyError:
            records[note_index] = [
                abs(error),
                error,
                error,
                freq,
                round(note_to_freq(note_index), 1),
            ]

    pprint(records)


main()
