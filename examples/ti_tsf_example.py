#!/usr/bin/env python3
"""
Python example for the TSF indicator from tulip_rs_python.

This example demonstrates:
1. Basic TSF calculation with optional outputs
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
    # Sample data from Rust example
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

    close_vec = np.array(close, dtype=np.float64)
    inputs = [close_vec]

    # Options from Rust example
    options = [5.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.tsf.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    if "optional_outputs" in info:
        print(f"Optional Outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.tsf.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.tsf.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Calculating the Full TSF Line with Optional Outputs
    # Full calculation - request ALL optional outputs
    optional_count = (
        len(eval(info["optional_outputs"])) if info.get("optional_outputs") else 0
    )
    optional_outputs = [True] * optional_count if optional_count > 0 else None
    outputs, _ = tulip_rs.indicators.tsf.indicator(inputs, options, optional_outputs)

    print(f"TSF Line: {outputs[0]}")
    if optional_count > 0:
        print(f"Linreg Line: {outputs[1]}")
        print(f"Slope Line: {outputs[2]}")
        print(f"Intercept Line: {outputs[3]}")

    ################################################### Calculating the partial TSF Line
    # Use partial data for state demo
    close_vec2 = np.array(close[:-5], dtype=np.float64)
    inputs2 = [close_vec2]

    # Partial calculation - also with optional outputs (matching Rust pattern)
    outputs2, state2 = tulip_rs.indicators.tsf.indicator(
        inputs2, options, optional_outputs
    )
    print(f"\nPartial TSF Line: {outputs2[0]}")
    if optional_count > 0:
        for i in range(1, len(outputs2)):
            print(f"Partial optional output {i}: {outputs2[i]}")

    ################################################### State Continuation Demo
    print("Demonstrating state continuation...")

    # Get state info if available
    try:
        state_info = state2.get_info()
        print(f"State info: {state_info}")
    except AttributeError:
        print("State info: TSF State - internal state for Time Series Forecast")

    # Use the state to continue calculation with new data
    print("Adding new data to existing state...")
    # Continue with remaining data
    new_close_vec = np.array(close[-5:], dtype=np.float64)
    new_data = [new_close_vec]
    # State continuation - main outputs only (matching Rust pattern)
    final_outputs = state2.batch_indicator(new_data)
    print(f"\nRemaining TSF Line: {final_outputs[0]}")

    # Verify by calculating full sequence at once
    print("Verification - calculating full sequence:")
    # Verify with full data - use same optional outputs as full calculation
    full_outputs, _ = tulip_rs.indicators.tsf.indicator(
        inputs, options, optional_outputs
    )
    print(f"Verification TSF Line: {full_outputs[0]}")
    if optional_count > 0:
        print(f"Verification Linreg Line: {full_outputs[1]}")
        print(f"Verification Slope Line: {full_outputs[2]}")
        print(f"Verification Intercept Line: {full_outputs[3]}")


if __name__ == "__main__":
    main()
