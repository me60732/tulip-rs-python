# Benchmark: homodynediscriminator (Homodyne Discriminator)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.homodynediscriminator.indicator([data.close], [])


BENCHMARK = BenchmarkDef(
    name="homodynediscriminator",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs=None,
)
