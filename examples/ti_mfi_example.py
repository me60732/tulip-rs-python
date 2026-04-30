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
    min_data_accuracy = tulip_rs.indicators.mfi.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    # Full calculation with all optional outputs
    optional_count = len(eval(info["optional_outputs"]))
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


if __name__ == "__main__":
    main()
