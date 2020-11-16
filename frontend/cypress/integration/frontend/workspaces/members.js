describe("Workspace Members page", () => {
  it("Shows the title", () => {
    cy.visit(`/workspaces/${Cypress.env("TEST_WORKSPACE_ID")}`);
    cy.contains("h1", "Selenium Testing");

    cy.get("a").contains("View members").click();
    cy.contains("h1", "Workspace members");
    cy.contains("p", "This is a list of all workspace members.");
  });

  context("Desktop - 990px+", () => {
    beforeEach(() => {
      cy.viewport(1200, 1080);
    });

    it("Shows members and moves admin to member on desktop", () => {
      cy.visit(`workspaces/${Cypress.env("TEST_WORKSPACE_ID")}/members`);
      cy.get("tr")
        .eq(1)
        .within(() => {
          cy.get("td").eq(0).contains("Lisa Pink");
          cy.get("td").eq(1).contains("a", "lisa.pink@example.com");
          cy.get("td").eq(2).get(`svg[width="24"]`).click();
        });
      cy.get("tr")
        .eq(2)
        .within(() => {
          cy.get("td").eq(0).contains("Permissions");
        });
      cy.get(".nhsuk-button").click();
    });

    it("Makes a user admin on desktop", () => {
      cy.visit(`workspaces/${Cypress.env("TEST_WORKSPACE_ID")}/members`);
      cy.get("tr")
        .eq(5)
        .within(() => {
          cy.get("td").eq(0).contains("Dorian Black");
          cy.get("td").eq(1).contains("a", "dorian.black@example.com");
          cy.get("td").eq(2).get(`svg[width="24"]`).click();
        });
      cy.get("tr")
        .eq(6)
        .within(() => {
          cy.get("td").eq(0).contains("Permissions");
          cy.get("td").eq(1).contains("Member");
        });

      cy.get(".nhsuk-button").click();
    });
  });

  context("Mobile - 320px", () => {
    beforeEach(() => {
      cy.viewport(320, 720);
    });

    it("Shows members and moves admin to member on mobile", () => {
      cy.visit(`workspaces/${Cypress.env("TEST_WORKSPACE_ID")}/members`);
      cy.get("h3")
        .next("ul")
        .within(() => {
          cy.get("li")
            .eq(0)
            .within(() => {
              cy.contains("h4", "Name of User");
              cy.contains("div", "Lisa Pink");
              cy.contains("h4", "Email");
              cy.contains("a", "lisa.pink@example.com");
              cy.get(`svg[width="24"]`).click();
              cy.contains("h4", "Permissions");
              cy.contains("div", "Administrator");
              cy.get("button").eq(1).click();
            });
        });
    });
    it("Makes a user admin on mobile", () => {
      cy.visit(`workspaces/${Cypress.env("TEST_WORKSPACE_ID")}/members`);
      cy.get("h3")
        .next("ul")
        .within(() => {
          cy.get("li")
            .eq(3)
            .within(() => {
              cy.contains("h4", "Name of User");
              cy.contains("div", "Dorian Black");
              cy.contains("h4", "Email");
              cy.contains("a", "dorian.black@example.com");
              cy.get(`svg[width="24"]`).click();
              cy.contains("h4", "Permissions");
              cy.contains("div", "Member");
              cy.get("button").eq(1).click();
            });
        });
    });
  });
});
