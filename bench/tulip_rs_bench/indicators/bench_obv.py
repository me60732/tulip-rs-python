# Benchmark: obv (On Balance Volume)
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
    return tulip_rs.indicators.obv.indicator([data.close, data.volume], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.OnBalanceVolumeIndicator(
        close=data.close, volume=data.volume
    ).on_balance_volume()


BENCHMARK = BenchmarkDef(
    name="obv",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)