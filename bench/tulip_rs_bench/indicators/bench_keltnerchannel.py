# Benchmark: keltnerchannel (Keltner Channel)
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
    return tulip_rs.indicators.keltnerchannel.indicator(
        [data.high, data.low, data.close], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    # ta's KeltnerChannel uses window_atr for the ATR period; options[1] is an
    # ATR multiplier in tulip-rs so we use ta's default multiplier instead.
    return ta.volatility.KeltnerChannel(
        high=data.high, low=data.low, close=data.close, window=int(options[0])
    ).keltner_channel_mband()


BENCHMARK = BenchmarkDef(
    name="keltnerchannel",
    options_list=[[20.0, 2.0], [20.0, 1.5], [14.0, 2.0], [10.0, 1.5]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
