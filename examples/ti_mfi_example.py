#!/usr/bin/env python3
"""
Python example for the MFI indicator from tulip_rs_python.

This example demonstrates:
1. Basic MFI calculation with optional outputs
2. Indicator info display
3. State continuation with new data
4. Matches the Rust reference example exactly
"""

try:
    import numpy as np
    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # Test Input Data - matches Rust example exactly
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
    info = tulip_rs.indicators.mfi.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.mfi.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    print()

    # Full calculation with all optional outputs
    optional_count = len(info["optional_outputs"])
    optional_outputs = [True] * optional_count if optional_count > 0 else None

    outputs, _ = tulip_rs.indicators.mfi.indicator(inputs, options, optional_outputs)
    print(f"Full MFI Line: {outputs[0]}")
    print(f"Full Typical Price Line: {outputs[1]}")

    # Partial calculation (remove last 5 points) - matches Rust example
    partial_high = high[:-5]
    partial_low = low[:-5]
    partial_close = close[:-5]
    partial_volume = volume[:-5]

    partial_high_vec = np.array(partial_high, dtype=np.float64)
    partial_low_vec = np.array(partial_low, dtype=np.float64)
    partial_close_vec = np.array(partial_close, dtype=np.float64)
    partial_volume_vec = np.array(partial_volume, dtype=np.float64)
    inputs2 = [partial_high_vec, partial_low_vec, partial_close_vec, partial_volume_vec]

    outputs2, state = tulip_rs.indicators.mfi.indicator(
        inputs2, options, optional_outputs
    )
    print(f"\nMFI Line: {outputs2[0]}")
    print(f"Typical Price Line: {outputs2[1]}")

    # State info
    state_info = state.get_info()
    print(f"State info: {state_info}")

    # Continuation with remaining data (last 5 points) - matches Rust example
    continuation_high = high[-5:]
    continuation_low = low[-5:]
    continuation_close = close[-5:]
    continuation_volume = volume[-5:]

    continuation_high_vec = np.array(continuation_high, dtype=np.float64)
    continuation_low_vec = np.array(continuation_low, dtype=np.float64)
    continuation_close_vec = np.array(continuation_close, dtype=np.float64)
    continuation_volume_vec = np.array(continuation_volume, dtype=np.float64)
    new_inputs = [
        continuation_high_vec,
        continuation_low_vec,
        continuation_close_vec,
        continuation_volume_vec,
    ]

    new_outputs = state.batch_indicator(new_inputs, optional_outputs)
    print(f"\nNew MFI Line: {new_outputs[0]}")
    print(f"New Typical Price Line: {new_outputs[1]}")

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Create data for 4 assets (SIMD lane requirement: 2, 4, 8, or 16)
    # Asset 1: Original data
    asset1_high_vec = np.array(high_vec, copy=True)
    asset1_low_vec = np.array(low_vec, copy=True)
    asset1_close_vec = np.array(close_vec, copy=True)
    asset1_volume_vec = np.array(volume_vec, copy=True)

    # Asset 2: Scaled up data
    asset2_high_vec = high_vec * 1.2
    asset2_low_vec = low_vec * 1.2
    asset2_close_vec = close_vec * 1.2
    asset2_volume_vec = volume_vec * 1.2

    # Asset 3: Different trend
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

    # Asset 4: Inverted trend
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

    # Prepare SIMD inputs - must be exactly 2, 4, 8, or 16 assets
    simd_inputs = [
        [asset1_high_vec, asset1_low_vec, asset1_close_vec, asset1_volume_vec],  # Asset 1
        [asset2_high_vec, asset2_low_vec, asset2_close_vec, asset2_volume_vec],  # Asset 2
        [asset3_high_vec, asset3_low_vec, asset3_close_vec, asset3_volume_vec],  # Asset 3
        [asset4_high_vec, asset4_low_vec, asset4_close_vec, asset4_volume_vec],  # Asset 4
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        # Calculate MFI for all assets using SIMD
        simd_outputs, simd_states = tulip_rs.indicators.mfi.simd_by_assets(
            simd_inputs, options, optional_outputs
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} MFI values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.mfi.indicator(
                asset_inputs, options, optional_outputs
            )
            print(f"Asset {i + 1} individual: {individual_output[0]}")

            # Verify SIMD matches individual calculation
            if np.allclose(simd_outputs[i][0], individual_output[0], rtol=1e-10, equal_nan=True):
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
        # Calculate MFI for all option sets using SIMD
        simd_opt_outputs, simd_opt_states = tulip_rs.indicators.mfi.simd_by_options(
            expanded_inputs, simd_options, optional_outputs
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_opt_outputs, simd_opt_states)):
            print(f"Option set {i + 1} MFI values (first 5): {output[0][:5]}")

        print("\nVerification - calculating each option set individually:")
        for i, opt in enumerate(simd_options):
            individual_output, _ = tulip_rs.indicators.mfi.indicator(
                expanded_inputs, opt, optional_outputs
            )
            print(
                f"Option set {i + 1} individual (first 5): {individual_output[0][:5]}"
            )

            # Verify SIMD matches individual calculation
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
