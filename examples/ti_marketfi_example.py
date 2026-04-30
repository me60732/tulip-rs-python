#!/usr/bin/env python3
"""
Python example for the MARKETFI indicator from tulip_rs_python.

This example demonstrates:
1. Basic MARKETFI calculation
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
    # Sample data from Rust example
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

    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    volume_vec = np.array(volume, dtype=np.float64)
    inputs = [high_vec, low_vec, volume_vec]

    # Options from Rust example
    options = []

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.marketfi.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    if "optional_outputs" in info:
        print(f"Optional Outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.marketfi.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.marketfi.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Calculating the Full MARKETFI Line
    # Full calculation
    optional_count = (
        len(eval(info["optional_outputs"])) if info.get("optional_outputs") else 0
    )
    optional_outputs = [True] * optional_count if optional_count > 0 else None
    outputs, _ = tulip_rs.indicators.marketfi.indicator(
        inputs, options, optional_outputs
    )

    print(f"Full MarketFI Line: {outputs[0]}")
    if optional_count > 0:
        for i in range(1, len(outputs)):
            print(f"Optional output {i}: {outputs[i]}")

    ################################################### Calculating the partial MARKETFI Line
    # Use partial data for state demo
    high_vec2 = np.array(high[:-5], dtype=np.float64)
    low_vec2 = np.array(low[:-5], dtype=np.float64)
    volume_vec2 = np.array(volume[:-5], dtype=np.float64)
    inputs2 = [high_vec2, low_vec2, volume_vec2]

    # Partial calculation - main outputs only (no optional outputs for state continuation)
    outputs2, state2 = tulip_rs.indicators.marketfi.indicator(inputs2, options)
    print(f"MarketFI Line: {outputs2[0]}")

    ################################################### State Continuation Demo
    print("Demonstrating state continuation...")

    # Get state info if available
    try:
        state_info = state2.get_info()
        print(f"State info: {state_info}")
    except AttributeError:
        print(
            "State info: MARKETFI State - internal state for Market Facilitation Index"
        )

    # Use the state to continue calculation with new data
    print("Adding new data to existing state...")
    # Continue with remaining data
    new_high_vec = np.array(high[-5:], dtype=np.float64)
    new_low_vec = np.array(low[-5:], dtype=np.float64)
    new_volume_vec = np.array(volume[-5:], dtype=np.float64)
    new_data = [new_high_vec, new_low_vec, new_volume_vec]
    # State continuation - main outputs only
    final_outputs = state2.batch_indicator(new_data)
    print(f"New MarketFI Line: {final_outputs[0]}")

    # Verify by calculating full sequence at once
    print("Verification - calculating full sequence:")
    # Verify with full data - use same optional outputs as full calculation
    full_outputs, _ = tulip_rs.indicators.marketfi.indicator(
        inputs, options, optional_outputs
    )
    print(f"Verification MarketFI Line: {full_outputs[0]}")
    if optional_count > 0:
        for i in range(1, len(full_outputs)):
            print(f"Verification optional output {i}: {full_outputs[i]}")


if __name__ == "__main__":
    main()
