# Benchmark: donchianchannel (Donchian Channel)
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
    return tulip_rs.indicators.donchianchannel.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    # DonchianChannel requires close even though tulip-rs only uses high/low.
    return ta.volatility.DonchianChannel(
        high=data.high, low=data.low, close=data.close, window=int(options[0])
    ).donchian_channel_mband()


BENCHMARK = BenchmarkDef(
    name="donchianchannel",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
