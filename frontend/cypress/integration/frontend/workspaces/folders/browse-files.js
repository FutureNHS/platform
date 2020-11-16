describe("Browse files", () => {
  it("Shows title and summary", () => {
    const start = `/workspaces/750a5fb7-47da-4a8c-8d2e-399ae8d24cde/folders/b337907b-439d-4d05-9937-4975fa00bed7`;
    cy.visit(start);
    cy.contains("h1", "FutureNHS Case Study Library");
    cy.contains("p", "Test folder with title FutureNHS Case Study Library");
    cy.contains("h3", "Files");
  });

  context("Desktop - 990px+", () => {
    beforeEach(() => {
      cy.viewport(1200, 1080);
    });

    it("Shows files on desktop", () => {
      const start = `/workspaces/750a5fb7-47da-4a8c-8d2e-399ae8d24cde/folders/b337907b-439d-4d05-9937-4975fa00bed7`;
      cy.visit(start);
      cy.get("tr")
        .eq(1)
        .within(() => {
          cy.get("td").eq(0).get(`svg[width="28"]`);
          cy.get("td")
            .eq(0)
            .contains("London Region NHS England Safeguarding Annual Review");
          cy.get("td").eq(1).contains("6 Oct 2020 17:53");
          cy.get("td").eq(2).contains("a", "Download file");
        });
      cy.get("tr")
        .eq(2)
        .within(() => {
          cy.get("td").eq(0).get(`svg[width="28"]`);
          cy.get("td")
            .eq(0)
            .contains("Midlands & East Region Safeguarding Annual Report");
          cy.get("td").eq(1).contains("6 Oct 2020 17:53");
          cy.get("td").eq(2).contains("a", "Download file");
        });
    });
  });

  context("Mobile - 320px", () => {
    beforeEach(() => {
      cy.viewport(320, 720);
    });

    it("Shows files on mobile", () => {
      const start = `/workspaces/750a5fb7-47da-4a8c-8d2e-399ae8d24cde/folders/b337907b-439d-4d05-9937-4975fa00bed7`;
      cy.visit(start);
      cy.get("h3")
        .next("ul")
        .within(() => {
          cy.get("li")
            .eq(0)
            .within(() => {
              cy.get(`svg[width="28"]`);
              cy.contains(
                "a",
                "London Region NHS England Safeguarding Annual Review"
              );
              cy.contains("h4", "Last modified");
              cy.contains("p", "6 Oct 2020 17:53");
              cy.contains("a", "Download file");
            });
          cy.get("li")
            .eq(1)
            .within(() => {
              cy.get(`svg[width="28"]`);
              cy.contains(
                "a",
                "Midlands & East Region Safeguarding Annual Report"
              );
              cy.contains("h4", "Last modified");
              cy.contains("p", "6 Oct 2020 17:53");
              cy.contains("a", "Download file");
            });
        });
    });
  });

  it("Click through to file page", () => {
    const start = `/workspaces/750a5fb7-47da-4a8c-8d2e-399ae8d24cde/folders/b337907b-439d-4d05-9937-4975fa00bed7`;
    cy.visit(start);
    cy.get("tr")
      .eq(1)
      .within(() => {
        cy.get("td")
          .eq(0)
          .contains("London Region NHS England Safeguarding Annual Review")
          .click();
      });
    cy.contains("h1", "London Region NHS England Safeguarding Annual Review", {
      timeout: Cypress.config("pageLoadTimeout"),
    });
    cy.contains(
      "p",
      "London Region NHS England Safeguarding Annual Review.ppt"
    );

    cy.get("tr")
      .eq(1)
      .within(() => {
        cy.get("td").eq(0).get(`svg[width="28"]`);
        cy.get("td")
          .eq(0)
          .contains("London Region NHS England Safeguarding Annual Review");
        cy.get("td").eq(1).contains("6 Oct 2020 17:53");
        cy.get("td").eq(2).contains("a", "Download file");
      });

    cy.get("tr").should("have.length", 2);
  });
});
