#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from dataclasses import dataclass
from pathlib import Path
from typing import List, Any, Tuple

import matplotlib.pyplot as plt

import matplotlib as mpl

mpl.rcParams["figure.dpi"] = 200
mpl.rcParams["savefig.dpi"] = 200


@dataclass
class BeatData:
    beat_number: int
    samples: List[int]
    fft: List[float]
    dominant_frequency: float
    maximum_amplitude: int
    human_readable_note: str
    root_frequency: str


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
                root_frequency=float(item["root_frequency"]),
                maximum_amplitude=int(item["maximum_amplitude"]),
                human_readable_note=item["human_readable_note"],
            )
        )
    return beats


def build_time_series(
    beats: List[BeatData],
) -> Tuple[List[int], List[int], List[Tuple[int, int]]]:
    all_samples: List[int] = []
    beat_indices: List[int] = []
    beat_ranges: List[Tuple[int, int]] = []

    current_index: int = 0
    for beat in beats:
        beat_start: int = current_index
        all_samples.extend(beat.samples)
        current_index += len(beat.samples)
        beat_end: int = current_index - 1

        beat_indices.append(beat_start)
        beat_indices.append(current_index)  # end marker after last sample
        beat_ranges.append((beat_start, beat_end))

    time_axis: List[int] = list(range(len(all_samples)))
    return time_axis, all_samples, beat_ranges


def plot_time_domain(beats: List[BeatData]) -> None:
    time_axis, all_samples, beat_ranges = build_time_series(beats)

    fig, ax = plt.subplots(figsize=(12, 6))
    ax.plot(time_axis, all_samples)
    ax.set_xlabel("Sample index (global)")
    ax.set_ylabel("Amplitude")
    ax.set_title("Time domain signal with beat markers")
    ax.grid(True)

    if all_samples:
        y_max: float = max(all_samples)
        y_min: float = min(all_samples)
        y_span: float = y_max - y_min if y_max != y_min else 1.0
        text_y: float = y_max - 0.15 * y_span

        for beat, (start, end) in zip(beats, beat_ranges):
            # Start marker
            ax.axvline(start, linestyle="--", linewidth=0.5)
            ax.text(
                start,
                text_y,
                f"Beat {beat.beat_number}: {beat.human_readable_note} ({beat.root_frequency:.2f}Hz)",
                rotation=0,
                va="bottom",
                ha="left",
                fontsize=12,
            )

            # horizontal max amplitude lines for this beat
            ax.hlines(
                [beat.maximum_amplitude, -beat.maximum_amplitude],
                xmin=start,
                xmax=end,
                linestyles="solid",
                linewidth=1,
                colors=["red", "red"],
            )

    plt.tight_layout()
    plt.show()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Show full time-domain signal with beat markers."
    )
    parser.add_argument(
        "json_path", type=Path, help="Path to JSON file with beat data."
    )
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    beats = load_beats(args.json_path)
    if not beats:
        return
    plot_time_domain(beats)


if __name__ == "__main__":
    main()
