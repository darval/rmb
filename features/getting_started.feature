Feature: Getting Started

  Scenario: Initing a transport
    Given nothing
    When I choose a internal transport
    Then init the internal transport

  Scenario: Initing the message manager
    Given an inited internal transport
    Then init the internal message manager

  Scenario: Initing the bus
    Given an inited internal transport
    And an inited msgmgr
    Then init the bus

  Scenario: Confirm internal transport
    Given an inited internal transport
    And an inited msgmgr
    Then querying the msgmsg should show internal transport
