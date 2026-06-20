# Benchmark: mama (MESA Adaptive Moving Average)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.mama.indicator([data.close], options)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.mama(pd.Series(data.close))


BENCHMARK = BenchmarkDef(
    name="mama",
    options_list=[[0.5, 0.05], [0.4, 0.04], [0.6, 0.06], [0.7, 0.07]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
)
