#!/usr/bin/env python3
"""
Python example for the AD indicator from tulip_rs_python.

This example demonstrates:
1. Basic AD calculation
2. Indicator info display
3. State continuation with new data
4. Full calculation verification
"""

try:
    import numpy as np
    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # Sample data: high, low, close, volume
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
        87.87,
        88.15,
        87.60,
    ]
    low = [
        81.29,
        80.64,
        82.65,
        82.70,
        83.07,
        82.65,
        82.20,
        83.35,
        84.15,
        84.11,
        85.39,
        86.04,
        86.58,
        87.32,
        87.00,
    ]
    close = [
        81.59,
        81.06,
        82.87,
        83.0,
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
        1000000,
        1100000,
        900000,
        1200000,
        800000,
        950000,
        1050000,
        1150000,
        1300000,
        1000000,
        1100000,
        1250000,
        980000,
        1180000,
        1080000,
    ]

    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    volume_vec = np.array(volume, dtype=np.float64)
    inputs = [high_vec, low_vec, close_vec, volume_vec]

    # Options for the AD calculation
    options = []

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.ad.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.ad.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    print()

    ################################################### Calculating the Full AD Line
    outputs, _ = tulip_rs.indicators.ad.indicator(inputs, options)
    print(f"Full AD Line: {outputs[0]}")

    ################################################### Calculating the partial AD Line
    # Use partial data for state demo
    high_vec2 = np.array(high[:-5], dtype=np.float64)
    low_vec2 = np.array(low[:-5], dtype=np.float64)
    close_vec2 = np.array(close[:-5], dtype=np.float64)
    volume_vec2 = np.array(volume[:-5], dtype=np.float64)
    inputs2 = [high_vec2, low_vec2, close_vec2, volume_vec2]

    outputs2, state2 = tulip_rs.indicators.ad.indicator(inputs2, options)
    print(f"Partial AD Line: {outputs2[0]}")

    ################################################### State Continuation Demo
    print("Demonstrating state continuation...")

    # Note: AD state doesn't have get_info method
    print("State info: AD State - internal state for Accumulation/Distribution Line")

    # Use the state to continue calculation with new data
    print("Adding new data to existing state...")
    # Continue with remaining data
    new_high_vec = np.array(high[-5:], dtype=np.float64)
    new_low_vec = np.array(low[-5:], dtype=np.float64)
    new_close_vec = np.array(close[-5:], dtype=np.float64)
    new_volume_vec = np.array(volume[-5:], dtype=np.float64)
    new_data = [new_high_vec, new_low_vec, new_close_vec, new_volume_vec]
    final_outputs = state2.batch_indicator(new_data)
    print(f"Continued AD Line: {final_outputs[0]}")

    # Verify by calculating full sequence at once
    print("Verification - calculating full sequence:")
    # Verify with full data
    full_high_vec = np.array(high, dtype=np.float64)
    full_low_vec = np.array(low, dtype=np.float64)
    full_close_vec = np.array(close, dtype=np.float64)
    full_volume_vec = np.array(volume, dtype=np.float64)
    full_data = [full_high_vec, full_low_vec, full_close_vec, full_volume_vec]
    full_outputs, _ = tulip_rs.indicators.ad.indicator(full_data, options)
    print(f"Full AD Line: {full_outputs[0]}")

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
        # Calculate AD for all assets using SIMD
        simd_outputs, simd_states = tulip_rs.indicators.ad.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} AD values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.ad.indicator(
                asset_inputs, options
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

if __name__ == "__main__":
    main()
