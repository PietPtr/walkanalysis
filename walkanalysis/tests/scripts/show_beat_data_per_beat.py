import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import List, Any

import matplotlib.pyplot as plt
import numpy as np

START_BEAT = 13
FFT_CUTOFF = 400


@dataclass
class BeatData:
    beat_number: int
    samples: List[float]
    fft: List[float]
    dominant_frequency: float
    root_frequency: float


@dataclass
class TranscriptionData:
    beat_data: List[BeatData]
    sample_rate: int
    upsample_rate: int


def load_beats(path: Path) -> List[TranscriptionData]:
    with path.open("r", encoding="utf-8") as f:
        raw: Any = json.load(f)

    beats: List[BeatData] = []
    for item in raw["beat_data"]:
        beats.append(
            BeatData(
                beat_number=int(item["number"]),
                samples=[float(v) for v in item["samples"]],
                fft=[float(v) for v in item["fft"]],
                dominant_frequency=float(item["dominant_frequency"]),
                root_frequency=float(item["root_frequency"])
                if item["root_frequency"] is not None
                else None,
            )
        )
    return TranscriptionData(
        beat_data=beats,
        sample_rate=raw["sample_rate"],
        upsample_rate=raw["upsample_rate"],
    )


def show_beats(data: TranscriptionData) -> None:
    for beat in data.beat_data[START_BEAT:]:
        fig, (ax_time, ax_freq) = plt.subplots(2, 1, figsize=(10, 6), sharex=False)
        fig.suptitle(f"Beat {beat.beat_number} — root freq: {beat.root_frequency:.2f}")

        # Time-domain samples
        ax_time.plot(range(len(beat.samples)), beat.samples)
        ax_time.set_xlabel("Sample index")
        ax_time.set_ylabel("Amplitude")
        ax_time.grid(True)

        # FFT magnitudes
        N = len(beat.fft)
        fs = data.sample_rate  # sample rate

        freqs = np.fft.rfftfreq(N, d=1 / fs)  # frequency for each FFT bin

        ax_freq.plot(freqs[0 : N // 2], beat.fft[0 : N // 2], marker=".")
        ax_freq.axvline(beat.root_frequency, linestyle="--", color="red")
        ax_freq.set_xlabel("Frequency (Hz)")
        ax_freq.set_ylabel("Magnitude")
        ax_freq.set_xlim((-10, 400))
        ax_freq.grid(True)

        plt.tight_layout()
        figManager = plt.get_current_fig_manager()
        figManager.window.showMaximized()
        plt.show()


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
