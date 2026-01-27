# Implementation Plan

- [x] 1. Set up core fee insights module structure
  - Create `src/insights/` directory with module files
  - Define core data structures and enums for fee insights
  - Set up error types specific to insights operations
  - Add insights module to main service integration
  - _Requirements: 4.1, 4.2, 4.3_

- [ ] 2. Implement fee data abstraction layer
  - [x] 2.1 Create FeeDataProvider trait and FeeDataPoint struct
    - Define trait interface for data source independence
    - Implement FeeDataPoint with all required fields
    - Create provider error types and result handling
    - _Requirements: 4.1_

  - [x] 2.2 Implement HorizonFeeDataProvider adapter
    - Create adapter that implements FeeDataProvider for HorizonClient
    - Transform HorizonFeeStats into FeeDataPoint format
    - Handle data conversion and error mapping
    - _Requirements: 4.1, 4.4_

  - [ ]* 2.3 Write property test for data source independence
    - **Property 7: Data source independence**
    - **Validates: Requirements 4.1, 4.4, 4.5**

- [ ] 3. Implement rolling average calculator
  - [x] 3.1 Create RollingAverageCalculator with circular buffer storage
    - Implement circular buffer for efficient data storage
    - Create time window configuration and management
    - Add methods for adding data points and calculating averages
    - _Requirements: 1.1, 1.2_

  - [x] 3.2 Add time window separation and partial data handling
    - Implement separate calculations for different time windows
    - Add logic for partial data marking when insufficient samples
    - Create timestamp tracking for calculation results
    - _Requirements: 1.2, 1.4_

  - [ ]* 3.3 Write property test for rolling average correctness
    - **Property 1: Rolling Average Calculation Correctness**
    - **Validates: Requirements 1.1, 1.3, 1.5**

  - [ ]* 3.4 Write property test for time window separation
    - **Property 2: Time Window Separation**
    - **Validates: Requirements 1.2**

- [ ] 4. Implement extremes tracking system
  - [x] 4.1 Create ExtremesTracker with current and historical tracking
    - Implement current period min/max tracking
    - Add historical period storage and management
    - Create methods for updating and querying extremes
    - _Requirements: 2.1, 2.2_

  - [x] 4.2 Add immediate update logic and period reset functionality
    - Implement immediate extreme value updates during processing
    - Add period reset with historical preservation
    - Create timestamp and metadata tracking for extremes
    - _Requirements: 2.3, 2.5_

  - [ ]* 4.3 Write property test for extremes tracking accuracy
    - **Property 3: Extremes Tracking Accuracy**
    - **Validates: Requirements 2.1, 2.3, 2.4**

  - [ ]* 4.4 Write property test for historical extremes preservation
    - **Property 4: Historical Extremes Preservation**
    - **Validates: Requirements 2.2, 2.5**

- [ ] 5. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 6. Implement congestion detection system
  - [x] 6.1 Create CongestionDetector with spike detection logic
    - Implement spike detection algorithm comparing fees to baselines
    - Add spike classification by severity and duration
    - Create data structures for spike representation
    - _Requirements: 3.1, 3.2_

  - [x] 6.2 Add trend analysis and aggregation capabilities
    - Implement multiple spike aggregation into trend indicators
    - Add trend strength calculation and duration prediction
    - Create congestion trend data structures and updates
    - _Requirements: 3.3, 3.4, 3.5_

  - [ ]* 6.3 Write property test for spike detection accuracy
    - **Property 5: Spike Detection Accuracy**
    - **Validates: Requirements 3.1, 3.2**

  - [ ]* 6.4 Write property test for congestion trend analysis
    - **Property 6: Congestion Trend Analysis**
    - **Validates: Requirements 3.3, 3.4, 3.5**

- [ ] 7. Implement central fee insights engine
  - [x] 7.1 Create FeeInsightsEngine orchestrator
    - Implement main engine struct with all component integration
    - Add configuration management and initialization
    - Create main processing method that coordinates all calculations
    - _Requirements: 1.3, 2.3, 3.5_

  - [x] 7.2 Add insights querying and data retrieval methods
    - Implement methods to get current insights, averages, extremes, trends
    - Add data quality indicators and metadata
    - Create comprehensive insights data structure
    - _Requirements: 1.5, 2.4_

  - [ ]* 7.3 Write unit tests for engine integration
    - Create unit tests for engine initialization and configuration
    - Test component integration and data flow
    - Verify error handling and edge cases
    - _Requirements: 1.3, 2.3, 3.5_

- [ ] 8. Integrate with existing service architecture
  - [x] 8.1 Add insights engine to main service
    - Integrate FeeInsightsEngine into main.rs service loop
    - Add insights processing to existing polling cycle
    - Create configuration integration with existing config system
    - _Requirements: 1.3, 2.3, 3.5_

  - [x] 8.2 Add insights API endpoints
    - Create REST endpoints for querying insights data
    - Add JSON serialization for all insights data structures
    - Implement error handling and response formatting
    - _Requirements: 1.5, 2.4_

  - [ ]* 8.3 Write integration tests for service integration
    - Test end-to-end insights processing in service context
    - Verify API endpoints return correct data
    - Test polling cycle integration and timing
    - _Requirements: 1.3, 2.3, 3.5_

- [ ] 9. Add comprehensive error handling and resilience
  - [ ] 9.1 Implement error handling for all calculation scenarios
    - Add handling for invalid data, numerical errors, memory issues
    - Implement graceful degradation for data source failures
    - Create comprehensive error logging and monitoring
    - _Requirements: 4.4, 1.4_

  - [ ] 9.2 Add data quality monitoring and metrics
    - Implement data quality indicators and tracking
    - Add metrics for calculation performance and accuracy
    - Create monitoring for data gaps and processing delays
    - _Requirements: 4.4, 1.4_

  - [ ]* 9.3 Write unit tests for error handling scenarios
    - Test all error conditions and recovery mechanisms
    - Verify graceful degradation and data quality indicators
    - Test edge cases and boundary conditions
    - _Requirements: 4.4, 1.4_

- [ ] 10. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.