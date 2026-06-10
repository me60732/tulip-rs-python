# Benchmark: bbands (Bollinger Bands)
from __future__ import annotations

from typing import Any, List

import ta.volatility

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.bbands.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    # BollingerBands computes all three bands; .bollinger_hband() triggers the
    # full rolling-std computation (same cost as upper/lower/middle together).
    return ta.volatility.BollingerBands(
        close=data.close, window=int(options[0]), window_dev=options[1]
    ).bollinger_hband()


BENCHMARK = BenchmarkDef(
    name="bbands",
    options_list=[[5.0, 2.0], [14.0, 2.0], [20.0, 2.0], [50.0, 2.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)