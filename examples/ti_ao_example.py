#!/usr/bin/env python3
"""
Python example for the AO indicator from tulip_rs_python.

This example demonstrates:
1. Basic AO calculation
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
    # Sample data from Rust example - base data extended 4 times
    high_base = [
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
    low_base = [
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

    # Extend the data 4 times as in Rust example
    high = high_base * 5  # Original + 4 extensions = 5 total
    low = low_base * 5

    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    inputs = [high_vec, low_vec]

    # Options for the AO calculation (none required)
    options = []

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.ao.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    if "optional_outputs" in info:
        print(f"Optional Outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.ao.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.ao.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Calculating the Full AO Line with Optional Outputs
    # Full calculation - request ALL optional outputs
    optional_count = (
        len(eval(info["optional_outputs"])) if info.get("optional_outputs") else 0
    )
    optional_outputs = [True] * optional_count if optional_count > 0 else None
    outputs, _ = tulip_rs.indicators.ao.indicator(inputs, options, optional_outputs)

    print(f"Full AO Line: {outputs[0]}")
    if optional_count > 0:
        print(f"\nShort sma: {outputs[1]}")
        print(f"\nLong sma: {outputs[2]}")

    ################################################### Calculating the partial AO Line
    # Use partial data for state demo
    high_vec2 = np.array(high[:-5], dtype=np.float64)
    low_vec2 = np.array(low[:-5], dtype=np.float64)
    inputs2 = [high_vec2, low_vec2]

    # Partial calculation - main outputs only (no optional outputs for state continuation)
    outputs2, state2 = tulip_rs.indicators.ao.indicator(inputs2, options)
    print(f"\n\nPartial AO Line: {outputs2[0]}")

    ################################################### State Continuation Demo
    print("Demonstrating state continuation...")

    # Get state info if available
    try:
        state_info = state2.get_info()
        print(f"State info: {state_info}")
    except AttributeError:
        print("State info: AO State - internal state for Awesome Oscillator")

    # Use the state to continue calculation with new data
    print("Adding new data to existing state...")
    # Continue with remaining data
    new_high_vec = np.array(high[-5:], dtype=np.float64)
    new_low_vec = np.array(low[-5:], dtype=np.float64)
    new_data = [new_high_vec, new_low_vec]
    # State continuation - main outputs only
    final_outputs = state2.batch_indicator(new_data)
    print(f"\nFinal AO Line: {final_outputs[0]}")

    # Verify by calculating full sequence at once
    print("Verification - calculating full sequence:")
    # Verify with full data - use same optional outputs as full calculation
    full_outputs, _ = tulip_rs.indicators.ao.indicator(
        inputs, options, optional_outputs
    )
    print(f"Verification AO Line: {full_outputs[0]}")
    if optional_count > 0:
        print(f"Verification Short SMA: {full_outputs[1]}")
        print(f"Verification Long SMA: {full_outputs[2]}")


if __name__ == "__main__":
    main()
