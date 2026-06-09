#!/usr/bin/env python3
"""
Python example for the ChaikinMF indicator from tulip_rs_python.

This example demonstrates:
1. Basic ChaikinMF calculation
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
    # Test Input Data - 20 bars
    high = [
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
        88.50,
        89.20,
        89.75,
        90.10,
        89.80,
    ]
    low = [
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
        87.60,
        88.15,
        88.90,
        89.40,
        88.95,
    ]
    close = [
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
        88.10,
        88.80,
        89.50,
        89.95,
        89.25,
    ]
    volume = [
        5653100.0,
        6447400.0,
        7690900.0,
        3831400.0,
        4455100.0,
        3798000.0,
        3936200.0,
        4732000.0,
        4841300.0,
        3915300.0,
        6830800.0,
        6694100.0,
        5293600.0,
        7985800.0,
        4807900.0,
        5100000.0,
        6200000.0,
        5800000.0,
        7100000.0,
        4900000.0,
    ]
    period = 5.0
    options = [period]

    # Prepare inputs
    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    volume_vec = np.array(volume, dtype=np.float64)
    inputs = [high_vec, low_vec, close_vec, volume_vec]

    # Show indicator info
    info = tulip_rs.indicators.chaikinmf.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.chaikinmf.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.chaikinmf.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    # Full calculation (ChaikinMF has no optional outputs)
    outputs, _ = tulip_rs.indicators.chaikinmf.indicator(inputs, options)
    print(f"Full CMF Line: {outputs[0]}")

    # Partial calculation (first 15 bars)
    partial_high_vec = np.array(high[:15], dtype=np.float64)
    partial_low_vec = np.array(low[:15], dtype=np.float64)
    partial_close_vec = np.array(close[:15], dtype=np.float64)
    partial_volume_vec = np.array(volume[:15], dtype=np.float64)
    inputs2 = [partial_high_vec, partial_low_vec, partial_close_vec, partial_volume_vec]

    outputs2, state = tulip_rs.indicators.chaikinmf.indicator(inputs2, options)
    print(f"\nPartial CMF Line (15 bars): {outputs2[0]}")

    # State info
    state_info = state.get_info()
    print(f"State info: {state_info}")

    # Continuation with remaining data (last 5 bars)
    continuation_high_vec = np.array(high[15:], dtype=np.float64)
    continuation_low_vec = np.array(low[15:], dtype=np.float64)
    continuation_close_vec = np.array(close[15:], dtype=np.float64)
    continuation_volume_vec = np.array(volume[15:], dtype=np.float64)
    new_inputs = [
        continuation_high_vec,
        continuation_low_vec,
        continuation_close_vec,
        continuation_volume_vec,
    ]

    new_outputs = state.batch_indicator(new_inputs)
    print(f"\nContinuation CMF Line (5 bars): {new_outputs[0]}")

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Create data for 4 assets (SIMD lane requirement: 2, 4, 8, or 16)
    asset1_high_vec = np.array(high_vec, copy=True)
    asset1_low_vec = np.array(low_vec, copy=True)
    asset1_close_vec = np.array(close_vec, copy=True)
    asset1_volume_vec = np.array(volume_vec, copy=True)

    asset2_high_vec = high_vec * 1.2
    asset2_low_vec = low_vec * 1.2
    asset2_close_vec = close_vec * 1.2
    asset2_volume_vec = volume_vec * 1.2

    asset3_high_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset3_low_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(low_vec)], dtype=np.float64
    )
    asset3_close_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(close_vec)], dtype=np.float64
    )
    asset3_volume_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(volume_vec)], dtype=np.float64
    )

    asset4_high_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset4_low_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(low_vec)], dtype=np.float64
    )
    asset4_close_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(close_vec)], dtype=np.float64
    )
    asset4_volume_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(volume_vec)], dtype=np.float64
    )

    simd_inputs = [
        [asset1_high_vec, asset1_low_vec, asset1_close_vec, asset1_volume_vec],
        [asset2_high_vec, asset2_low_vec, asset2_close_vec, asset2_volume_vec],
        [asset3_high_vec, asset3_low_vec, asset3_close_vec, asset3_volume_vec],
        [asset4_high_vec, asset4_low_vec, asset4_close_vec, asset4_volume_vec],
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        simd_outputs, simd_states = tulip_rs.indicators.chaikinmf.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} CMF values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.chaikinmf.indicator(
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
    expanded_high = np.tile(high, 20).astype(np.float64)
    expanded_low = np.tile(low, 20).astype(np.float64)
    expanded_close = np.tile(close, 20).astype(np.float64)
    expanded_volume = np.tile(volume, 20).astype(np.float64)
    expanded_inputs = [expanded_high, expanded_low, expanded_close, expanded_volume]

    simd_options = [
        [3.0],  # Option set 1
        [5.0],  # Option set 2 (Original)
        [7.0],  # Option set 3
        [10.0],  # Option set 4
    ]

    print(f"Processing {len(simd_options)} option sets simultaneously using SIMD...")
    for i, opt in enumerate(simd_options):
        print(f"Option set {i + 1}: {opt}")
    print()

    try:
        simd_opt_outputs, simd_opt_states = (
            tulip_rs.indicators.chaikinmf.simd_by_options(expanded_inputs, simd_options)
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_opt_outputs, simd_opt_states)):
            print(f"Option set {i + 1} CMF values (first 5): {output[0][:5]}")

        print("\nVerification - calculating each option set individually:")
        for i, opt in enumerate(simd_options):
            individual_output, _ = tulip_rs.indicators.chaikinmf.indicator(
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
