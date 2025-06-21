# Blend Result

Result blender for robotframework output.xml files.

Currently under heavy development.

At current it can parse output.xml files and convert them into flat lists of
calls and csv files.
Also some basic blending functionality and output formats csv and ods are
in an experimental stage.

Writing ods files is slow for large xml files. Use release mode!

# Run

Parse files and dump them to CSV:

    cargo run --release -- parse robot/results/output_a.xml output_a.csv

Blend Files:

    cargo run -- blend 4 stuff.csv robot/results/*.xml

Testint with filter:

    RUST_LOG=debug cargo test test_parser_c -- --show-output

# Setup

    python3 -m venv venv
    . venv/bin/activate
    pip install robotframework

# Documentation

Relevant crates:

* https://crates.io/crates/quick-xml
* https://docs.rs/quick-xml/0.37.4/quick_xml/
* https://github.com/tafia/quick-xml

* https://crates.io/crates/clap

* https://github.com/robotframework/robotframework/blob/master/doc/schema/result.xsd
* https://docs.rs/log/0.4.27/log/
* https://docs.rs/simple_logger/latest/simple_logger/

* https://crates.io/crates/spreadsheet-ods
* https://docs.rs/spreadsheet-ods/0.25.0/spreadsheet_ods/index.html

