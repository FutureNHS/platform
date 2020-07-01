# FutureNHS Test Approach

At Red Badger, we utilise Agile methodologies to release with regularity and predictability. Cross-functional teams are equipped with resources from all disciplines; Development, Test, User Experience, Design and Insights who collaborate to deliver to a consistently high quality.
Through a combination of automated tests and effective manual testing, our emphasis is on early defect detection. We aim to prioritise and address defects as early as possible in the development cycle. 

We aim to deliver a high quality platform and migrate users in the shortest time possible. To enable this we are proposing to pursue testing strategies that will enable quick delivery, without sacrificing quality. Through test-driven development (TTD), incorporating comprehensive automation testing and targeted, effective manual testing locally and in Production, we are confident that we can minimise the number of issues that users will encounter. 

This change is a test.

## Contents

- [Proposed Testing Flow](#proposed-testing-flow)
  - [Continuous Integration Testing](#continuous-integration-testing)
- [Test Environments](#test-environments)
  - [Local Environment](#local-environment)
  - [Branch Environments](#branch-environments)
  - [Test Data](#test-data)
- [Feature Flags and Staging](#feature-flags-and-staging)
  - [Feature Flags](#feature-flags)
  - [No Staging Environment](#no-staging-environment)
  - [Infrastructure Changes](#infrastructure-changes)
  - [Performance Testing](#performance-testing)
- [Levels of Testing](#levels-of-testing)
  - [The Testing Pyramid](#the-testing-pyramid)
  - [Unit Tests](#unit-tests)
  - [Integration Tests](#integration-tests)
  - [End-to-End Tests](#end-to-end-tests)
  - [Snapshot Testing](#snapshot-testing)
  - [Manual Testing](#manual-testing)
- [Other Testing](#other-testing)
  - [Penetration Testing](#penetration-testing)
  - [Vulnerability Testing](#vulnerability-testing)
- [Testing Requirements](#testing-requirements)
  - [Platform / Browser requirements](#platform--browser-requirements)
- [Accessibility Requirements](#accessibility-requirements)
- [Issue Logging and Handling](#issue-logging-and-handling)     
    - [Triage](#triage)     
    - [Feature Requests ](#feature-requests)    
    - [Tech Debt](#tech-debt)     
    - [Writing a Unit Test](#writing-a-unit-test)
- [Testing Tooling](#testing-tooling)     
    - [Automation](#automation)     
    - [Manual Testing aids](#manual-testing-aids)

## Proposed Testing Flow

In the spirit of pursuing high-quality quick delivery, we are proposing the following process for each ticket:
1. Create a feature flag (if new feature).
2. Write TTD Unit/Integration tests.
3. Build the feature locally.
4. Get the TTD tests passing.
5. Add Snapshot and E2E tests (if relevant).
6. Create a Pull Request and get code reviewed.
7. QA performs exploratory manual testing locally.
8. Any blocker issues discovered are fixed immediately. Minor issues are logged and prioritised in the backlog. 
9. When happy that acceptance criteria are met and there are no blocker issues, QA merges to master.
10. Test suite runs and passes on master.
11. Code deploys to Production as canary deployment/behind a feature flag.
12. QA verifies the code in Production.
13. Feature flag is toggled to allow access to a beta group (stakeholders, beta testers).
14. Once all parties are satisfied with a feature’s completed status, the feature flag is toggled to allow wider user groups to interact with the feature. This process can be segmented gradually i.e. allowing 5% of users see the feature, then 10%, 20%, etc. 
15. Once all users have been given access to the feature for a period of time, the feature flag is removed. 

### Continuous Integration Testing
Unit, Integration and Snapshot tests will be written and run each time before code is committed/merged/deployed, to act as a safety net to prevent feature regressions and help to reduce the number of bugs that make it into Production. 
In addition to these standard tests an end-to-end testing suite will be developed, which will test all parts of the system in conjunction with each other. They will focus on ‘happy paths’ - common user journeys that are essential for the platform to operate effectively. 

## Test Environments
### Local Environment
The earliest possible stage to test code is in a local environment. This would entail the QA checking out code from the GitHub repository to their laptop. They would then compile and run the platform locally and perform manual exploratory testing. 

By testing locally we save time by not having to spin up a branch environment or rebuild any time a minor change to the code is made. 

A snapshot of anonymised Production data will be spun up in a local database when testing locally. This will give us the best possible data to work with.

### Branch Environments
Occasionally we may want to spin up a branch environment for tickets that cannot be tested effectively in a local environment, e.g. if we need to test major infrastructure changes. 

Again these would use a snapshot of anonymised Production data to facilitate testing. There is also potential to have live anonymised Production data available but this requires further investigation. 

### Test Data
As mentioned above, to provide data used in the local branch environments we propose using actual Production data wherever possible. Obviously this presents issues surrounding privacy. To resolve this we propose anonymising everything at the point of snapshotting the Production data. This would entail stripping out any identifying information that could link data back to the person it was posted by or about. 

## Feature Flags and Staging

### Feature Flags
We intend to use Feature Flagging to deploy code safely to Production. Please refer to the [Feature Flags](../architecture/decisions/0003_feature_flags.md) architectural decision record for more information.

### No Staging Environment
We are proposing to forgo a Staging Environment - please refer to the [No Staging Environment](../architecture/decisions/0004_no_staging_env.md) architectural decision record for more information.

### Infrastructure Changes
When it comes to making big infrastructure changes it is likely that we would opt to spin up a new environment entirely to deploy to rather than going straight into Production with a feature flag. This would mitigate the risk that the infrastructure changes might cause catastrophic failures in the live Production environment. 
Once thoroughly tested in a temporary ‘staging’ environment the changes would be rolled out to Production.

### Performance Testing
To avoid negatively affecting Production performance it would be wise to test performance/load testing in a separate temporary environment that mirrors Production. We can use a framework such as Gatling in conjunction with Postman to perform load testing to stress the platform and measure performance. It would be useful to establish a baseline for future comparison.

## Levels of Testing
### The Testing Pyramid

![diagram of the testing pyramid](./test_pyramid.png)

Popularised by Mike Cohn, the idea of the Test Pyramid is fundamental to our approach to testing within Red Badger. Through previous experience and research, we have come to believe that the principles behind this model are of vital importance for a successful delivery built upon a solid foundation of quality assurance. As automation plays an increasingly important role in enabling continuous integration and delivery, it is vital that this area is given the highest attention.

### Unit Tests
These test the individual logic of a particular function or component in the application, isolated from the other components. The most important things to test at this level are conditional logic, looping and any kind of data manipulation (e.g., sorting of a list). 
Unit tests are fast to run and independent of each other. When utilised with a focus on test-driven development (TDD) they act as documentation for a feature. They should be executed as often and early as possible to provide early feedback on code functionality and quality. 
Unit tests are predominantly the concern of the developer working on a feature, but the tester will take an active role in writing tests and ensuring all scenarios are covered. 

### Integration Tests
These tests ensure that the interaction between functions and components are working together to create the intended result. For example, in the React ecosystem, an appropriate level in a component tree (this might be an entire page) is rendered and populated with input data. Click events are simulated on the UI to make sure that the full logical application flow works as expected. 
API tests also fall under this bracket. With these, a tester can verify that the back-end of the application is in a working state, without going through the additional layer of complexity interacting with a browser.

### End-to-End Tests
End-to-end (aka E2E or UI) testing generally mimics as closely as possible user interaction with the application. A browser, driven by software, simulates the user's journey through the web application. These are the most time consuming and therefore expensive tests in the suite.  
We will also have End-to-end API tests that run solely on the API layer in combination with mocking/stubbing of data/responses to simulate a user’s interaction with the system. These will execute more quickly without the overhead of a browser rendering pages and interactions. 

### Snapshot Testing
Snapshot comparison testing checks for styling regressions as new features are added to the application. The rendering of a component is compared with its previous incarnation and flagged up if a change is detected. These tests supplement more expensive UI tests by ensuring there are no regressions to individual components throughout the application, build after build.

### Manual Testing
Manual testing is often deemed an expensive overhead, however, it provides value by testing scenarios that may not have been caught by other testing methods.
A manual tester has the ability to view the site in the context of requirements, find errors and ignore false negatives far easier and faster than any automated test or screenshot comparison tool. 
Manual testers understand the original story requirement and therefore are essential for confirming that a built feature actually solves the original problem defined by a story. They can also find edge cases in the application that may only be found during real browser testing, as opposed to rigid automated tests.
If any regressions are found by manual testing, the fix should come with an automated test at the lowest sensible level in the triangle.
In general, manual tests: 
- Focus on testing features, rather than finding regressions.
- Can be scripted or exploratory in nature.
- Try and find as many bugs and regressions as possible in the previous steps.
- Allow manual testers' pattern recognition ability to quickly find issues on a wide range of browsers and devices.
- Provide feedback on the effectiveness of the automated suite of tests.

## Other Testing
### Penetration Testing
Penetration testing is a specialised style of testing that would largely need to be carried out by a third party service. It also appears to be a suggested requirement from the Government’s technology service manual: https://www.gov.uk/service-manual/technology/vulnerability-and-penetration-testing

### Vulnerability Testing
Azure provides the ability to scan for vulnerabilities in Docker images we push in our registry. This should provide a sufficient level of confidence that our services are secure, but it won’t hurt to include extra vulnerability testing in the remit of whichever third party we contract to perform the penetration testing. 

## Testing Requirements
The Test Lead for the project will be responsible for manually testing to the following standards.
### Platform / Browser requirements
From ‘Platform Non Functional Requirements’:

Windows
- Internet Explorer 11 (35.87%)
- Edge (latest versions) (12.5%)
- Google Chrome (latest versions) (36.19%)
- Mozilla Firefox (latest versions) (0.98%)

MacOS
- Safari 12 and later (3.91%)
- Google Chrome (latest versions) (1.87%)
- Mozilla Firefox (latest versions) (0.2%)

iOS
- Safari for iOS 10.3 and later (5.66%)
- Google Chrome (latest versions) (0.26%)

Android
- Google Chrome (latest versions) (1.42%)
- Samsung Internet (latest versions) (0.6%)

(%s are from Google Analytics for all Kahootz sessions from 01/03/20-31/5/20)

Breakdown by device:
- Desktop: 82.89%
- Tablet: 9.8%
- Mobile: 7.31%

If we are to attempt to test every single user story on all required platforms this will result in a lower rate of delivery without adding much value, given the low numbers of users on the rarer platforms. We are reducing the number of required testing platforms for every user story in favour of periodic wider testing for the less popular platforms. The ideal time to do more comprehensive platform testing would be when a feature is complete in production but before enabling its feature flag for the wider user base. 

The proposed requirements for a pull request to be merged to master/deployed to production should be that it is tested on the following platforms/browsers:

Windows
- Google Chrome (latest version)
- Internet Explorer 11
- Edge

Edge is more of an optional choice given its similarity to Google Chrome.

To reduce risks from not comprehensively testing all platforms we will rely on an automated test suite that can be run to check that the new features at least don’t break anything on the other platforms. 

## Accessibility Requirements
https://www.gov.uk/service-manual/helping-people-to-use-your-service/understanding-wcag

- Text-to-speech (Screen Reader)
  - It should be possible for a user to have any text on screen read back to them by a Screen reader application. 
  - All non-decorative images should have alt-text values that describe the image contents. 
  - Form elements should have explicit labels.
  - Link text should be descriptive.
- Keyboard Navigation
  - It should be possible to operate and switch between every interactable element of a feature solely with the keyboard. 
  - Tabbing through sections of the page should flow in a natural order. 
  - It should be possible to play, pause and stop moving content with the keyboard.
- Magnification
  - It should be possible to see content clearly and interact with features when using a screen magnifier.
- Contrast
  - Chosen colours should provide enough contrast against their backgrounds to remain visible if the user changes their contrast settings. 
- Text size 200%
  - Every feature is still usable when text size is increased by 200%
- Touch target areas (Mobile)
  - Touch target areas for interactive elements should be large enough to be easily selected. 
- Landscape (Mobile)
  - When viewing content on mobile devices, it should still be readable when the device is in landscape orientation.

Other considerations: 
- Any audio or video should have transcripts
- Video should have captions

Further testing information:
https://accessibility.18f.gov/checklist/

We might need to get an accessibility audit according to this https://www.gov.uk/service-manual/helping-people-to-use-your-service/testing-for-accessibility#getting-an-accessibility-audit

## Issue Logging and Handling
Anyone who discovers an issue should feel free to submit it on GitHub. This way the issues sit alongside the codebase and can be resolved automatically upon a relevant bug-fix being merged. When an issue is logged it should be added to the project backlog for prioritisation. If the person logging an issue feels it has enough urgency they should raise it with the product owner/wider team for discussion. Otherwise, regular ‘triage’ sessions will be held to discuss the bug backlog and whether there are any issues that can be brought into the current stream of work to be fixed.

### Triage
> “(In a hospital) the process of deciding how seriously ill or injured a person is, so that the most serious cases can be treated first”

A triage session is where various stakeholders for the project come together to go through the issue backlog to determine their severity and prioritise them for fixing going forwards. 

### Feature Requests 
Anyone within the team should feel free to make suggestions for improving the quality of a feature or the platform as a whole. This can be done in the form of a ‘feature request’ and should be logged in the same manner as a bug. It should then go into the backlog along with other issues and be triaged accordingly. If the feature request is deemed to have some value then it can be spun out into a ticket for further discovery or development.

### Tech Debt
Tech Debt should also be logged as an Issue, so it can be triaged accordingly. We will want to keep on top of Tech Debt so that it doesn’t get out of control and start to negatively impact our ability to deliver a quality platform.

### Writing a Unit Test
It has been suggested that when a bug is logged that the QA write a unit test for the issue. This will aid any developer fixing it, and also help to build a regression suite to ensure that the issue does not reappear in the future. It will be up to the QA to decide if the bug requires one and if their skills are up to the task.

## Testing Tooling
### Automation

- Unit/Integration tests - Jest
- E2E - Cypress
- API - Postman

Potential accessibility tools: 

- [A11y](https://github.com/addyosmani/a11y)
- [AccessLintCI](https://github.com/accesslint/accesslint-ci)
- [Pa11y](http://pa11y.org/)

Honourable mentions:

- Puppeteer (Headless Chrome)
- Gatling (Load testing)

### Manual Testing aids
Some kind of VM for testing Internet Explorer.
Browserstack will be essential for testing cross-platform. 
