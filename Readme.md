# Blend Result

Result blender for robotframework output.xml files.
At least it should become one.

Currently under heavy development.

At current it can parse output.xml files and convert them into flat lists of
calls and csv files.

# Run

    cargo run  -- robot/results/output_a.xml output.csv
    cargo test && cargo run  -- robot/results/output_a.xml output.csv
    cargo test test_parser_c -- --show-output
    RUST_LOG=debug cargo test test_parser_c -- --show-output
    RUST_LOG=debug cargo test create_elements -- --show-output

# Setup

    python3 -m venv venv
    . venv/bin/activate
    pip install robotframework

# Documentation

Relevant crates:

https://crates.io/crates/quick-xml
https://docs.rs/quick-xml/0.37.4/quick_xml/
https://github.com/tafia/quick-xml

https://crates.io/crates/clap


https://github.com/robotframework/robotframework/blob/master/doc/schema/result.xsd
https://docs.rs/log/0.4.27/log/
https://docs.rs/simple_logger/latest/simple_logger/
