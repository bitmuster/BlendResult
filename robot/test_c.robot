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
    IF    ${failhere} == True
        Keyword C
        Log    The if branch
        Fail
    ELSE
        Log    The else branch
    END

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
