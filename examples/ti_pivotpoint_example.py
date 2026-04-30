#!/usr/bin/env python3
"""
Python example for the PIVOTPOINT indicator from tulip_rs_python.

This example demonstrates:
1. Basic PIVOTPOINT calculation
2. Indicator info display
3. Pivot point levels display
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
    # Sample data from Rust example
    high = [
        82.15,
        81.89,
        83.03,
        83.3,
        83.85,
        83.9,
        83.33,
        84.3,
        84.84,
        85.0,
        85.9,
        86.58,
        86.98,
        88.0,
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
        82.3,
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

    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    inputs = [high_vec, low_vec, close_vec]

    # Options from Rust example
    options = [5.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.pivotpoint.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    if "optional_outputs" in info:
        print(f"Optional Outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.pivotpoint.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.pivotpoint.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Calculating the Pivot Points
    # Full calculation
    optional_count = (
        len(eval(info["optional_outputs"])) if info.get("optional_outputs") else 0
    )
    optional_outputs = [True] * optional_count if optional_count > 0 else None
    outputs, _ = tulip_rs.indicators.pivotpoint.indicator(
        inputs, options, optional_outputs
    )

    # Display pivot points in the same format as Rust example
    pivot_data = outputs[0]
    print(
        f"Pivot Points: s3: {pivot_data[0]}, s2: {pivot_data[1]}, s1: {pivot_data[2]}, "
        f"pp: {pivot_data[3]}, r1: {pivot_data[4]}, r2: {pivot_data[5]}, r3: {pivot_data[6]}"
    )

    if optional_count > 0:
        for i in range(1, len(outputs)):
            print(f"Optional output {i}: {outputs[i]}")


if __name__ == "__main__":
    main()
