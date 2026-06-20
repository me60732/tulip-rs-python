# Benchmark: ichimoku (Ichimoku Cloud)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ichimoku.indicator(
        [data.high, data.low, data.close], options
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.ichimoku(
        pd.Series(data.high), pd.Series(data.low), pd.Series(data.close)
    )


BENCHMARK = BenchmarkDef(
    name="ichimoku",
    options_list=[[9.0, 26.0], [5.0, 10.0], [7.0, 14.0], [9.0, 52.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
)
