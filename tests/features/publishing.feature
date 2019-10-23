Feature: Publishing

  Scenario Outline: Publishing a message
    Given an inited <type> bus
    When I publish <num> message(s)
    Then the pending message count should increment by <num>

    Examples: 
      | type     | num |
      | internal |  1  |
      | local    |  3  |
      | network  |  10 |
