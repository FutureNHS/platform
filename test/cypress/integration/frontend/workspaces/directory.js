describe("Workspace Directory", () => {
  before(() => {
    cy.login(
      Cypress.env("TEST_LOGIN_EMAIL_ADDRESS"),
      Cypress.env("TEST_LOGIN_PASSWORD")
    );
    cy.server({ force404: true });
    cy.route("POST", "https://dc.services.visualstudio.com/v2/track", {});
  });

  // Currently only works against Prod or if you manually create a Workspace named "Selenium Testing" in your local/dev cluster environment
  it("Shows title and loads directory items", () => {
    cy.visit(`/workspaces/directory`);
    cy.contains("h1", "My workspaces");
    cy.contains("h3", Cypress.env("TEST_WORKSPACE_NAME"));
  });

  it("Navigates to workspace page", () => {
    cy.visit(`/workspaces/directory`);

    cy.get("h3:first")
      .click()
      .then(($h3) => {
        const text = $h3.text();
        cy.contains("h1", text);
        cy.contains("h2", "Most recent items");
      });
  });
});
