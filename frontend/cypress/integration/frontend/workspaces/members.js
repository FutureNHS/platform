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

    it("Shows list of admins and members on desktop", () => {
      cy.visit(`workspaces/${Cypress.env("TEST_WORKSPACE_ID")}/members`);
      cy.get("tr")
        .eq(1)
        .within(() => {
          cy.get("td").eq(0).contains("Lisa Pink");
          cy.get("td").eq(1).contains("a", "lisa.pink@example.com");
          cy.get("td").eq(2).get(`svg[width="24"]`);
        });
    });
  });
});
