Feature: Getting Started

  Scenario: Initing a transport
    Given nothing
    When I choose a transport
    Then init the transport

  Scenario: Initing the message manager
    Given an inited transport
    Then init the message manager

  Scenario: Initing the bus
    Given an inited transport
    Then init the bus

  Rule: A rule
    
    Scenario: a scenario inside a rule
      Given I am in inside a rule
      Then things are working
      