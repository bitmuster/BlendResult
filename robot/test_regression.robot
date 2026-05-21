
*** Settings ***
Documentation    Regression suite

*** Variables ***
${MESSAGE}       Hello, world!

*** Test Cases ***

Demo Test Scoped Variable
    [Documentation]    This is a test for regressions
    VAR    ${some_var}    Some var
    VAR    ${some_var2}    Some var    scope=TEST
    VAR    ${some_var3}    Some var    scope=SUITE
    VAR    ${some_var4}    Some var    scope=SUITES
    VAR    ${some_var5}    Some var    scope=GLOBAL

