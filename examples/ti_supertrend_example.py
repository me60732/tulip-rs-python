#!/usr/bin/env python3
"""
Python example for the Supertrend indicator from tulip_rs_python.

This example demonstrates:
1. Basic supertrend calculation with optional outputs
2. Indicator info display
3. State continuation with new data
4. SIMD by assets and by options
"""

try:
    import numpy as np

    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # 40-bar OHLC price data
    high = np.array(
        [
            82.15,
            81.89,
            83.03,
            83.30,
            83.85,
            83.90,
            83.33,
            84.30,
            84.84,
            85.00,
            85.90,
            86.58,
            86.98,
            88.00,
            87.87,
            88.20,
            88.70,
            89.10,
            88.50,
            89.00,
            89.60,
            89.90,
            89.30,
            90.10,
            90.50,
            91.00,
            90.30,
            91.00,
            91.60,
            92.00,
            91.30,
            92.00,
            92.60,
            93.00,
            92.30,
            93.00,
            93.60,
            94.00,
            93.30,
            94.10,
        ],
        dtype=np.float64,
    )
    low = np.array(
        [
            81.29,
            80.64,
            81.31,
            82.65,
            83.07,
            83.11,
            82.49,
            82.30,
            84.15,
            84.11,
            84.03,
            85.39,
            85.76,
            87.17,
            87.01,
            87.20,
            87.80,
            88.20,
            87.60,
            88.00,
            88.60,
            88.90,
            88.30,
            89.00,
            89.40,
            89.80,
            89.20,
            89.90,
            90.50,
            90.80,
            90.20,
            90.90,
            91.50,
            91.80,
            91.20,
            91.90,
            92.50,
            92.80,
            92.20,
            92.90,
        ],
        dtype=np.float64,
    )
    close = np.array(
        [
            81.59,
            81.06,
            82.87,
            83.00,
            83.61,
            83.15,
            82.84,
            83.99,
            84.55,
            84.36,
            85.53,
            86.54,
            86.89,
            87.77,
            87.29,
            87.50,
            88.10,
            88.50,
            87.90,
            88.20,
            88.80,
            89.10,
            88.70,
            89.30,
            89.70,
            90.10,
            89.50,
            90.20,
            90.80,
            91.10,
            90.50,
            91.20,
            91.80,
            92.10,
            91.50,
            92.20,
            92.80,
            93.10,
            92.50,
            93.20,
        ],
        dtype=np.float64,
    )

    inputs = [high, low, close]
    options = [7.0, 3.0]  # period, step (multiplier)

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.supertrend.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional Outputs: {info['optional_outputs']}")

    min_data = tulip_rs.indicators.supertrend.min_data(options)
    print(f"Minimum data required: {min_data}")
    print()

    ################################################### Calculating the Full Supertrend
    optional_outputs = [True] * len(info["optional_outputs"])
    outputs, _ = tulip_rs.indicators.supertrend.indicator(
        inputs, options, optional_outputs
    )

    print(f"FULL Supertrend: {outputs[0]}")
    print(f"FULL ATR (optional): {outputs[1]}")
    print(f"FULL TR (optional): {outputs[2]}")
    print(f"FULL Median Price (optional): {outputs[3]}")

    ################################################### Calculating the Partial Supertrend
    inputs_partial = [high[:-5], low[:-5], close[:-5]]
    outputs_partial, state = tulip_rs.indicators.supertrend.indicator(
        inputs_partial, options
    )
    print(f"\nSupertrend: {outputs_partial[0]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")
    new_data = [high[-5:], low[-5:], close[-5:]]
    continued_outputs = state.batch_indicator(new_data)
    print(f"\nNew Supertrend: {continued_outputs[0]}")

    print("\nVerification - calculating full sequence:")
    full_outputs, _ = tulip_rs.indicators.supertrend.indicator(
        inputs, options, optional_outputs
    )
    print(f"Verification Supertrend: {full_outputs[0]}")
    print(f"Verification ATR (optional): {full_outputs[1]}")
    print(f"Verification TR (optional): {full_outputs[2]}")
    print(f"Verification Median Price (optional): {full_outputs[3]}")

    print(f"\nData split: {len(close) - 5} + 5 = {len(close)} total elements")

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Asset 2: scale all inputs by 1.2
    asset2_high = high * 1.2
    asset2_low = low * 1.2
    asset2_close = close * 1.2

    # Asset 3: different upward trend (monotone transform preserves high >= low)
    asset3_high = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(high)], dtype=np.float64
    )
    asset3_low = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(low)], dtype=np.float64
    )
    asset3_close = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(close)], dtype=np.float64
    )

    # Asset 4: downward trend (monotone transform preserves high >= low)
    asset4_high = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(high)], dtype=np.float64
    )
    asset4_low = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(low)], dtype=np.float64
    )
    asset4_close = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(close)], dtype=np.float64
    )

    simd_inputs = [
        [high, low, close],
        [asset2_high, asset2_low, asset2_close],
        [asset3_high, asset3_low, asset3_close],
        [asset4_high, asset4_low, asset4_close],
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        simd_outputs, simd_states = tulip_rs.indicators.supertrend.simd_by_assets(
            simd_inputs, options, optional_outputs
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} Supertrend values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.supertrend.indicator(
                asset_inputs, options, optional_outputs
            )
            print(f"Asset {i + 1} individual: {individual_output[0]}")

            if np.allclose(
                simd_outputs[i][0], individual_output[0], rtol=1e-10, equal_nan=True
            ):
                print(f"✓ Asset {i + 1} SIMD matches individual calculation")
            else:
                print(f"✗ Asset {i + 1} SIMD does not match individual calculation")

        print("\nSIMD by Assets demonstration completed successfully!")

    except Exception as e:
        print(f"SIMD by Assets error: {e}")

    ################################################### SIMD by Options Demo
    print("\n" + "=" * 60)
    print("SIMD BY OPTIONS DEMONSTRATION")
    print("=" * 60)

    # Expand inputs to ensure we have enough data for larger period options
    expanded_high = np.tile(high, 10).astype(np.float64)
    expanded_low = np.tile(low, 10).astype(np.float64)
    expanded_close = np.tile(close, 10).astype(np.float64)
    expanded_inputs = [expanded_high, expanded_low, expanded_close]

    simd_options = [
        [5.0, 2.0],  # Option set 1: period=5, multiplier=2.0
        [7.0, 3.0],  # Option set 2: period=7, multiplier=3.0 (original)
        [10.0, 2.5],  # Option set 3: period=10, multiplier=2.5
        [14.0, 2.0],  # Option set 4: period=14, multiplier=2.0
    ]

    print(f"Processing {len(simd_options)} option sets simultaneously using SIMD...")
    for i, opt in enumerate(simd_options):
        print(f"Option set {i + 1}: period={opt[0]}, multiplier={opt[1]}")
    print()

    try:
        simd_opt_outputs, simd_opt_states = (
            tulip_rs.indicators.supertrend.simd_by_options(
                expanded_inputs, simd_options, optional_outputs
            )
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_opt_outputs, simd_opt_states)):
            print(f"Option set {i + 1} Supertrend values (first 5): {output[0][:5]}")

        print("\nVerification - calculating each option set individually:")
        for i, opt in enumerate(simd_options):
            individual_output, _ = tulip_rs.indicators.supertrend.indicator(
                expanded_inputs, opt, optional_outputs
            )
            print(
                f"Option set {i + 1} individual (first 5): {individual_output[0][:5]}"
            )

            if np.allclose(
                simd_opt_outputs[i][0], individual_output[0], rtol=1e-10, equal_nan=True
            ):
                print(f"✓ Option set {i + 1} SIMD matches individual calculation")
            else:
                print(
                    f"✗ Option set {i + 1} SIMD does not match individual calculation"
                )

        print("\nSIMD by Options demonstration completed successfully!")

    except Exception as e:
        print(f"SIMD by Options error: {e}")


if __name__ == "__main__":
    main()
