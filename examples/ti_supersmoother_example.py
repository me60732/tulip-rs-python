#!/usr/bin/env python3
"""
Python example for the Super Smoother Filter indicator from tulip_rs_python.

This example demonstrates:
1. Basic supersmoother calculation (no optional outputs)
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
    # 40-bar close price data
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

    inputs = [close]
    options = [10.0]  # period

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.supersmoother.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional Outputs: {info['optional_outputs']}")

    min_data = tulip_rs.indicators.supersmoother.min_data(options)
    print(f"Minimum data required: {min_data}")
    print()

    ################################################### Calculating the Full Super Smoother
    # No optional outputs for this indicator
    optional_outputs = None
    outputs, _ = tulip_rs.indicators.supersmoother.indicator(
        inputs, options, optional_outputs
    )

    print(f"FULL Super Smoother: {outputs[0]}")

    ################################################### Calculating the Partial Super Smoother
    inputs_partial = [close[:-5]]
    outputs_partial, state = tulip_rs.indicators.supersmoother.indicator(
        inputs_partial, options
    )
    print(f"\nSuper Smoother: {outputs_partial[0]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")
    new_data = [close[-5:]]
    continued_outputs = state.batch_indicator(new_data)
    print(f"\nNew Super Smoother: {continued_outputs[0]}")

    print("\nVerification - calculating full sequence:")
    full_outputs, _ = tulip_rs.indicators.supersmoother.indicator(inputs, options)
    print(f"Verification Super Smoother: {full_outputs[0]}")

    print(f"\nData split: {len(close) - 5} + 5 = {len(close)} total elements")

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    asset1_close = np.array(close, copy=True)
    asset2_close = close * 1.2
    asset3_close = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(close)], dtype=np.float64
    )
    asset4_close = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(close)], dtype=np.float64
    )

    simd_inputs = [
        [asset1_close],
        [asset2_close],
        [asset3_close],
        [asset4_close],
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        simd_outputs, simd_states = tulip_rs.indicators.supersmoother.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} Super Smoother values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.supersmoother.indicator(
                asset_inputs, options
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
    expanded_close = np.tile(close, 10).astype(np.float64)
    expanded_inputs = [expanded_close]

    simd_options = [
        [5.0],  # Option set 1: period=5
        [10.0],  # Option set 2: period=10 (original)
        [14.0],  # Option set 3: period=14
        [20.0],  # Option set 4: period=20
    ]

    print(f"Processing {len(simd_options)} option sets simultaneously using SIMD...")
    for i, opt in enumerate(simd_options):
        print(f"Option set {i + 1}: period={opt[0]}")
    print()

    try:
        simd_opt_outputs, simd_opt_states = (
            tulip_rs.indicators.supersmoother.simd_by_options(
                expanded_inputs, simd_options
            )
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_opt_outputs, simd_opt_states)):
            print(
                f"Option set {i + 1} Super Smoother values (first 5): {output[0][:5]}"
            )

        print("\nVerification - calculating each option set individually:")
        for i, opt in enumerate(simd_options):
            individual_output, _ = tulip_rs.indicators.supersmoother.indicator(
                expanded_inputs, opt
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
