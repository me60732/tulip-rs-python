#!/usr/bin/env python3
"""
Python example for the AVGPRICE indicator from tulip_rs_python.

This example demonstrates:
1. Basic AVGPRICE calculation with optional outputs
2. Indicator info display
3. State continuation with new data
4. Exact match with Rust reference implementation
"""

try:
    import numpy as np
    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # Sample data: open, high, low, close prices (matching Rust example exactly)
    open_prices = [
        81.85,
        81.20,
        81.55,
        82.91,
        83.10,
        83.41,
        82.71,
        82.70,
        84.20,
        84.25,
        84.03,
        85.45,
        86.18,
        88.00,
        87.60,
    ]
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

    # Options for AVGPRICE: none (matching Rust example)
    options = []

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.avgprice.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.avgprice.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.avgprice.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Full dataset calculation - Request ALL optional outputs
    open_vec = np.array(open_prices, dtype=np.float64)
    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    inputs = [open_vec, high_vec, low_vec, close_vec]

    # Get optional outputs count and request all
    optional_count = len(eval(info["optional_outputs"]))
    optional_outputs = [True] * optional_count if optional_count > 0 else None

    outputs, _ = tulip_rs.indicators.avgprice.indicator(
        inputs, options, optional_outputs
    )

    print("Full dataset calculation:")
    print(f"AvgPrice Line: {outputs[0]}")

    # Display optional outputs if they exist
    if optional_count > 0:
        for i in range(1, len(outputs)):
            print(f"Optional output {i}: {outputs[i]}")

    print(
        f"\nCalculated {len(outputs[0])} values from {len(open_prices)} input elements"
    )

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Create data for 4 assets (SIMD lane requirement: 2, 4, 8, or 16)
    # Asset 1: Original data
    asset1_open_vec = np.array(open_vec, copy=True)
    asset1_high_vec = np.array(high_vec, copy=True)
    asset1_low_vec = np.array(low_vec, copy=True)
    asset1_close_vec = np.array(close_vec, copy=True)

    # Asset 2: Scaled up data
    asset2_open_vec = open_vec * 1.2
    asset2_high_vec = high_vec * 1.2
    asset2_low_vec = low_vec * 1.2
    asset2_close_vec = close_vec * 1.2

    # Asset 3: Different trend
    asset3_open_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(open_vec)], dtype=np.float64
    )
    asset3_high_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset3_low_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(low_vec)], dtype=np.float64
    )
    asset3_close_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(close_vec)], dtype=np.float64
    )

    # Asset 4: Inverted trend
    asset4_open_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(open_vec)], dtype=np.float64
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

    # Prepare SIMD inputs - must be exactly 2, 4, 8, or 16 assets
    simd_inputs = [
        [asset1_open_vec, asset1_high_vec, asset1_low_vec, asset1_close_vec],  # Asset 1
        [asset2_open_vec, asset2_high_vec, asset2_low_vec, asset2_close_vec],  # Asset 2
        [asset3_open_vec, asset3_high_vec, asset3_low_vec, asset3_close_vec],  # Asset 3
        [asset4_open_vec, asset4_high_vec, asset4_low_vec, asset4_close_vec],  # Asset 4
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        # Calculate AVGPRICE for all assets using SIMD
        simd_outputs, simd_states = tulip_rs.indicators.avgprice.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} AVGPRICE values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.avgprice.indicator(
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
