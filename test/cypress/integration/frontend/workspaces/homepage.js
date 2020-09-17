describe("Workspace Homepage", () => {
  before(() => {
    cy.login(
      Cypress.env("TEST_LOGIN_EMAIL_ADDRESS"),
      Cypress.env("TEST_LOGIN_PASSWORD")
    );
    cy.server({ force404: true });
    cy.route("POST", "https://dc.services.visualstudio.com/v2/track", {});
  });

  it("Shows the title", () => {
    cy.visit(`/workspaces/${Cypress.env("TEST_WORKSPACE_ID")}`);
    cy.contains("h1", "Selenium Testing");
    cy.contains("h2", "Most recent items");
  });
});
