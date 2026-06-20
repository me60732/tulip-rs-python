# Benchmark: ad (Accumulation/Distribution)
from __future__ import annotations

from typing import Any, List

import ta.volume
import tulipy

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ad.indicator(
        [data.high, data.low, data.close, data.volume], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.AccDistIndexIndicator(
        high=data.high,
        low=data.low,
        close=data.close,
        volume=data.volume,
    ).acc_dist_index()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.ad(data.high, data.low, data.close, data.volume)


BENCHMARK = BenchmarkDef(
    name="ad",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy},
)
