import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import List, Any

import matplotlib.pyplot as plt

START_BEAT = 14


@dataclass
class BeatData:
    beat_number: int
    samples: List[int]
    fft: List[float]
    dominant_frequency: float


def load_beats(path: Path) -> List[BeatData]:
    with path.open("r", encoding="utf-8") as f:
        raw: Any = json.load(f)

    beats: List[BeatData] = []
    for item in raw:
        beats.append(
            BeatData(
                beat_number=int(item["number"]),
                samples=[int(v) for v in item["samples"]],
                fft=[float(v) for v in item["fft"]],
                dominant_frequency=float(item["dominant_frequency"]),
            )
        )
    return beats


def show_beats(beats: List[BeatData]) -> None:
    for beat in beats[START_BEAT:]:
        fig, (ax_time, ax_freq) = plt.subplots(2, 1, figsize=(10, 6), sharex=False)
        fig.suptitle(
            f"Beat {beat.beat_number} — dominant freq: {beat.dominant_frequency:.2f}"
        )

        # Time-domain samples
        ax_time.plot(range(len(beat.samples)), beat.samples)
        ax_time.set_xlabel("Sample index")
        ax_time.set_ylabel("Amplitude")
        ax_time.grid(True)

        # FFT magnitudes
        x_fft = list(range(len(beat.fft)))  # assume bins are indices
        ax_freq.plot(x_fft, beat.fft)
        ax_freq.axvline(beat.dominant_frequency, linestyle="--")
        ax_freq.set_xlabel("Frequency bin")
        ax_freq.set_ylabel("Magnitude")
        ax_freq.grid(True)

        plt.tight_layout()
        plt.show()  # blocks until window is closed


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Visualize beat data JSON.")
    parser.add_argument(
        "json_path", type=Path, help="Path to JSON file with beat data."
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    beats = load_beats(args.json_path)
    show_beats(beats)


if __name__ == "__main__":
    main()
