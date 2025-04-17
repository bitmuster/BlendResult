*** Settings ***
Documentation    Example suite

*** Variables ***
${MESSAGE}       Hello, world!

*** Test Cases ***

Demo Test A
    [Documentation]    This is a demo test
    Keyword A   ${MESSAGE}

Demo Test B
    No Operation
    Keyword B

Demo Test C
    No Operation
    Keyword C

Demo Test D
    Log To Console  ${MESSAGE}

*** Keywords ***

Keyword A
    [Arguments]    ${args}
    No Operation

Keyword B
    Keyword A   ${MESSAGE}
    No Operation

Keyword C
    Keyword B
    No Operation
