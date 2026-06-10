# Benchmark: emv (Ease of Movement)
from __future__ import annotations

from typing import Any, List

import ta.volume

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.emv.indicator(
        [data.high, data.low, data.volume], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.EaseOfMovementIndicator(
        high=data.high,
        low=data.low,
        volume=data.volume,
    ).ease_of_movement()


BENCHMARK = BenchmarkDef(
    name="emv",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
