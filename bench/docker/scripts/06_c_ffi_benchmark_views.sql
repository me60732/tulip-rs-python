-- =============================================================================
-- 06_c_ffi_benchmark_views.sql
-- Adds C/FFI-comparison views to the existing indicator_benchmark database.
-- Does NOT recreate the database or touch existing tables/views.
--
-- Applied automatically by Docker on first init.
-- Comment out the volume mount in docker-compose.yaml to skip these views.
--
-- Run manually:
--   psql -U postgres -h localhost -d indicator_benchmark \
--        -f scripts/06_c_ffi_benchmark_views.sql
--
-- Implementation types written by the C benchmark harness
-- (tulip_rs_ffi/bench):
--   'tulip_rs_ffi_c'        — tulip-rs called via the hand-rolled extern "C" FFI wrapper
--   'C_tulip'               — Tulip Indicators (C), tulip_rs_ffi/bench/tulip_indicators submodule
--   'talib'                 — TA-Lib, tulip_rs_ffi/bench/ta_lib_src submodule
--
-- (Superseded 'tulip_rs_diplomat_c' / Diplomat-generated FFI harness — the
-- Diplomat bindings and their benchmark harness have been removed entirely.)
--
-- Unlike the Rust/Python/Node harnesses, this one has no SIMD/batched code
-- path (it's a straight zero-copy FFI wrapper around the same per-call
-- Rust functions), so there are no *_simd_* views here.
--
-- Both comparison views show tulip_rs_ffi_c results even when no reference
-- library ran the same indicator (comparison columns will be NULL in that
-- case).
-- =============================================================================

\c indicator_benchmark

\echo '>>> Creating C/FFI benchmark views...'

-- Drop in reverse-dependency order so re-running is safe
DROP VIEW IF EXISTS c_ffi_avg_options_comparison;
DROP VIEW IF EXISTS c_ffi_performance_comparison;
-- Drop the superseded Diplomat-era views if they still exist from a previous run.
DROP VIEW IF EXISTS c_diplomat_avg_options_comparison;
DROP VIEW IF EXISTS c_diplomat_performance_comparison;

-- ---------------------------------------------------------------------------
-- c_ffi_performance_comparison
-- One row per (run, indicator, stock, option-set).
-- Pivots tulip_rs_ffi_c, C_tulip, and talib side by side and computes
-- x-faster ratios relative to tulip_rs_ffi_c.
-- Rows are included whenever tulip_rs_ffi_c has a result; reference
-- columns are NULL when no matching reference run exists for that
-- combination.
-- ---------------------------------------------------------------------------
CREATE VIEW c_ffi_performance_comparison AS
SELECT
    runs.id                            AS run_id,
    runs.run_timestamp                 AS benchmark_date,
    (runs.system_info ->> 'hostname')  AS hostname,
    ind.name                           AS indicator_name,
    res.stock_symbol,
    res.data_source,
    res.input_size,
    res.options,

    -- tulip_rs_ffi_c
    max(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
             THEN res.mean_time_ns END)                              AS tulip_rs_ffi_c_mean_ns,
    max(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
             THEN res.std_dev_ns END)                                AS tulip_rs_ffi_c_stddev_ns,

    -- C_tulip (Tulip Indicators C library)
    max(CASE WHEN res.implementation_type = 'C_tulip'
             THEN res.mean_time_ns END)                              AS c_tulip_mean_ns,
    max(CASE WHEN res.implementation_type = 'C_tulip'
             THEN res.std_dev_ns END)                                AS c_tulip_stddev_ns,

    -- talib
    max(CASE WHEN res.implementation_type = 'talib'
             THEN res.mean_time_ns END)                              AS talib_mean_ns,
    max(CASE WHEN res.implementation_type = 'talib'
             THEN res.std_dev_ns END)                                AS talib_stddev_ns,

    -- C_tulip / tulip_rs_ffi_c  (> 1 means tulip_rs_ffi_c is faster)
    round(
        (max(CASE WHEN res.implementation_type = 'C_tulip'
                  THEN res.mean_time_ns END))::numeric
        / NULLIF(
            (max(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                      THEN res.mean_time_ns END))::numeric,
          0),
    2)                                                               AS c_tulip_to_ffi_ratio,

    -- talib / tulip_rs_ffi_c  (> 1 means tulip_rs_ffi_c is faster)
    round(
        (max(CASE WHEN res.implementation_type = 'talib'
                  THEN res.mean_time_ns END))::numeric
        / NULLIF(
            (max(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                      THEN res.mean_time_ns END))::numeric,
          0),
    2)                                                               AS talib_to_ffi_ratio,

    -- % time saved vs C_tulip (NULL when C_tulip has no result)
    round(
        (
          (max(CASE WHEN res.implementation_type = 'C_tulip'
                    THEN res.mean_time_ns END)
           - max(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                      THEN res.mean_time_ns END))::numeric
          / NULLIF(
              max(CASE WHEN res.implementation_type = 'C_tulip'
                        THEN res.mean_time_ns END)::numeric,
            0)
        ) * 100,
    2)                                                               AS ffi_speedup_pct_vs_c_tulip,

    -- % time saved vs talib (NULL when talib has no result)
    round(
        (
          (max(CASE WHEN res.implementation_type = 'talib'
                    THEN res.mean_time_ns END)
           - max(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                      THEN res.mean_time_ns END))::numeric
          / NULLIF(
              max(CASE WHEN res.implementation_type = 'talib'
                        THEN res.mean_time_ns END)::numeric,
            0)
        ) * 100,
    2)                                                               AS ffi_speedup_pct_vs_talib

FROM benchmark_runs runs
JOIN benchmark_results res ON runs.id = res.run_id
JOIN indicators ind        ON res.indicator_id = ind.id
WHERE res.implementation_type IN ('tulip_rs_ffi_c', 'C_tulip', 'talib')
GROUP BY
    runs.id, runs.run_timestamp, runs.system_info,
    ind.name, res.stock_symbol, res.data_source, res.input_size, res.options
-- Require tulip_rs_ffi_c to be present; reference libraries are optional.
HAVING max(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c' THEN 1 END) = 1
ORDER BY runs.run_timestamp DESC, ind.name, res.stock_symbol;

-- ---------------------------------------------------------------------------
-- c_ffi_avg_options_comparison
-- One row per (run, indicator) — averaged across all option sets and stocks.
-- Includes all indicators that have a tulip_rs_ffi_c result; reference
-- columns are NULL when no matching reference run exists for that indicator.
-- ---------------------------------------------------------------------------
CREATE VIEW c_ffi_avg_options_comparison AS
SELECT
    runs.id                            AS run_id,
    runs.run_timestamp                 AS benchmark_date,
    (runs.system_info ->> 'hostname')  AS hostname,
    ind.name                           AS indicator_name,

    -- tulip_rs_ffi_c
    round(avg(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                   THEN res.mean_time_ns END))                       AS tulip_rs_ffi_c_avg_ns,
    count(DISTINCT CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                        THEN res.options END)                        AS ffi_options_count,

    -- C_tulip
    round(avg(CASE WHEN res.implementation_type = 'C_tulip'
                   THEN res.mean_time_ns END))                       AS c_tulip_avg_ns,
    count(DISTINCT CASE WHEN res.implementation_type = 'C_tulip'
                        THEN res.options END)                        AS c_tulip_options_count,

    -- talib
    round(avg(CASE WHEN res.implementation_type = 'talib'
                   THEN res.mean_time_ns END))                       AS talib_avg_ns,
    count(DISTINCT CASE WHEN res.implementation_type = 'talib'
                        THEN res.options END)                        AS talib_options_count,

    -- C_tulip / tulip_rs_ffi_c
    round(
        avg(CASE WHEN res.implementation_type = 'C_tulip'
                 THEN res.mean_time_ns END)
        / NULLIF(
            avg(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                     THEN res.mean_time_ns END),
          0),
    2)                                                               AS c_tulip_to_ffi_ratio,

    -- talib / tulip_rs_ffi_c
    round(
        avg(CASE WHEN res.implementation_type = 'talib'
                 THEN res.mean_time_ns END)
        / NULLIF(
            avg(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                     THEN res.mean_time_ns END),
          0),
    2)                                                               AS talib_to_ffi_ratio,

    -- % time saved vs C_tulip
    round(
        (
          avg(CASE WHEN res.implementation_type = 'C_tulip'
                   THEN res.mean_time_ns END)
          - avg(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                     THEN res.mean_time_ns END)
        )
        / NULLIF(
            avg(CASE WHEN res.implementation_type = 'C_tulip'
                     THEN res.mean_time_ns END),
          0) * 100,
    2)                                                               AS ffi_speedup_pct_vs_c_tulip,

    -- % time saved vs talib
    round(
        (
          avg(CASE WHEN res.implementation_type = 'talib'
                   THEN res.mean_time_ns END)
          - avg(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c'
                     THEN res.mean_time_ns END)
        )
        / NULLIF(
            avg(CASE WHEN res.implementation_type = 'talib'
                     THEN res.mean_time_ns END),
          0) * 100,
    2)                                                               AS ffi_speedup_pct_vs_talib

FROM benchmark_runs runs
JOIN benchmark_results res ON runs.id = res.run_id
JOIN indicators ind        ON res.indicator_id = ind.id
WHERE res.implementation_type IN ('tulip_rs_ffi_c', 'C_tulip', 'talib')
GROUP BY runs.id, runs.run_timestamp, runs.system_info, ind.name
-- Require tulip_rs_ffi_c to be present; reference libraries are optional.
HAVING max(CASE WHEN res.implementation_type = 'tulip_rs_ffi_c' THEN 1 END) = 1
ORDER BY runs.run_timestamp DESC, ind.name;

\echo '>>> C/FFI benchmark views ready.'
