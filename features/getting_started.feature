Feature: Getting Started

  Scenario Outline: Initing a transport
    Given the following <type> transports:
    When I choose a <type> transport
    Then init the <type> transport

    Examples: 
      | type     |
      | internal |
      | local    |
      | network  |


  Scenario Outline: Initing the message manager
    Given an inited <type> transport
    Then init the <type> message manager

    Examples: 
      | type     |
      | internal |
      | local    |
      | network  |

  Scenario Outline: Initing the bus
    Given an inited <type> transport
    And an inited <type> msgmgr
    Then init the <type> bus

    Examples: 
      | type     |
      | internal |
      | local    |
      | network  |

  Scenario Outline: Confirm the transport
    Given an inited <type> transport
    And an inited <type> msgmgr
    Then querying the msgmgr should show <type> transport

    Examples: 
      | type     |
      | internal |
      | local    |
      | network  |
