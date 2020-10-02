describe("Create folder page", () => {
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
    const start = `/workspaces/01bb9a4d-2977-4c43-b28c-2a72b4eda453`;
    cy.visit(start);
    cy.contains("a", "Create new folder").click();
    cy.contains("h1", "Create a folder");

    cy.get("button").contains("Discard").click();
    cy.location("pathname").should("eq", start);
  });
});
