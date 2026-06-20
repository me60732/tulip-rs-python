#!/usr/bin/env python3
"""
Python example for the VWAP (Volume Weighted Average Price) indicator from tulip_rs_python.

This example demonstrates:
1. Basic vwap calculation with optional outputs
2. Indicator info display
3. State continuation with new data
4. SIMD by assets (OPTIONS_WIDTH=0, so no SIMD by options)
"""

try:
    import numpy as np

    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # 40-bar OHLCV price data
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
    volume = np.array(
        [
            5653100,
            6447400,
            7690900,
            3831400,
            4455100,
            3798000,
            3936200,
            4732000,
            4841300,
            3915300,
            6830800,
            6694100,
            5293600,
            7985800,
            4807900,
            5100000,
            5300000,
            4900000,
            5200000,
            4800000,
            5600000,
            5800000,
            5400000,
            6000000,
            6200000,
            5800000,
            6400000,
            6600000,
            6200000,
            6800000,
            7000000,
            6600000,
            7200000,
            7400000,
            7000000,
            7600000,
            7800000,
            7400000,
            8000000,
            8200000,
        ],
        dtype=np.float64,
    )

    inputs = [high, low, close, volume]
    options = []

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.vwap.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional Outputs: {info['optional_outputs']}")

    min_data = tulip_rs.indicators.vwap.min_data(options)
    print(f"Minimum data required: {min_data}")
    print()

    ################################################### Calculating the Full VWAP
    optional_outputs = [True] * len(info["optional_outputs"])
    outputs, _ = tulip_rs.indicators.vwap.indicator(inputs, options, optional_outputs)

    print(f"FULL VWAP: {outputs[0]}")
    print(f"FULL Typical Price (optional): {outputs[1]}")

    ################################################### Calculating the Partial VWAP
    inputs_partial = [high[:-5], low[:-5], close[:-5], volume[:-5]]
    outputs_partial, state = tulip_rs.indicators.vwap.indicator(inputs_partial, options)
    print(f"\nVWAP: {outputs_partial[0]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")
    new_data = [high[-5:], low[-5:], close[-5:], volume[-5:]]
    continued_outputs = state.batch_indicator(new_data)
    print(f"\nNew VWAP: {continued_outputs[0]}")

    print("\nVerification - calculating full sequence:")
    full_outputs, _ = tulip_rs.indicators.vwap.indicator(
        inputs, options, optional_outputs
    )
    print(f"Verification VWAP: {full_outputs[0]}")
    print(f"Verification Typical Price (optional): {full_outputs[1]}")

    print(f"\nData split: {len(close) - 5} + 5 = {len(close)} total elements")

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Asset 2: scale price inputs by 1.2, volume by 1.1
    asset2_high = high * 1.2
    asset2_low = low * 1.2
    asset2_close = close * 1.2
    asset2_volume = volume * 1.1

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
    asset3_volume = volume * 0.9

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
    asset4_volume = volume * 1.3

    simd_inputs = [
        [high, low, close, volume],
        [asset2_high, asset2_low, asset2_close, asset2_volume],
        [asset3_high, asset3_low, asset3_close, asset3_volume],
        [asset4_high, asset4_low, asset4_close, asset4_volume],
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% price, +10% volume)")
    print("Asset 3: Different upward trend (-10% volume)")
    print("Asset 4: Downward trend (+30% volume)")
    print()

    try:
        simd_outputs, simd_states = tulip_rs.indicators.vwap.simd_by_assets(
            simd_inputs, options, optional_outputs
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} VWAP values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.vwap.indicator(
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


if __name__ == "__main__":
    main()
