
*** Settings ***
Documentation    Regression suite

*** Variables ***
${MESSAGE}       Hello, world!

*** Test Cases ***

Demo Test Scoped Variable
    [Documentation]    This is a test for regressions
    VAR    ${some_var}    Some var

