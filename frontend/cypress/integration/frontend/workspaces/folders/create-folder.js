describe("Create folder page", () => {
  // Currently only works against Prod or if you manually create a Workspace named "Selenium Testing" in your local/dev cluster environment
  it("Shows title, renders form and submits", () => {
    cy.visit(
      `/workspaces/01bb9a4d-2977-4c43-b28c-2a72b4eda453/folders/create-folder`
    );
    cy.contains("h1", "Create a folder");

    cy.get("#title").type("New Folder title");
    cy.get("#description").type("New Folder description");
    cy.get("form").submit();
  });

  it("Shows title, form and discard to go back to previous page ", () => {
    cy.visit(
      `/workspaces/01bb9a4d-2977-4c43-b28c-2a72b4eda453/folders/create-folder`
    );
    cy.contains("h1", "Create a folder");

    cy.get("button").contains("Discard").click();
    cy.go("back");
  });
});
