# Benchmark: supertrend (Super Trend)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.supertrend.indicator(
        [data.high, data.low, data.close], options
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.supertrend(
        pd.Series(data.high),
        pd.Series(data.low),
        pd.Series(data.close),
        length=int(options[0]),
        multiplier=float(options[1]),
    )


BENCHMARK = BenchmarkDef(
    name="supertrend",
    options_list=[[7.0, 3.0], [5.0, 2.0], [10.0, 2.5], [14.0, 2.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
)
