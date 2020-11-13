describe("Edit file details", () => {
  const url = `/workspaces/53a41928-8997-4687-9455-749d1932254b/folders/8a02d598-1fde-40c8-8bb0-f328faf3dd61/files/6770b9db-d45b-450a-9655-2855f3e0c064`;
  it("Shows title, renders form and submits", () => {
    cy.visit(url);
    cy.contains("h1", "London Region NHS England Safeguarding Annual Review");

    cy.contains("Edit file details").should("not.exist");
    cy.get(`[data-cy="file-options"]`).click();
    cy.contains("Edit file details").should("exist");

    cy.get(`[data-cy="edit-file-details"]`).click();

    cy.contains(
      "h1",
      "London Region NHS England Safeguarding Annual Review"
    ).should("not.exist");
    cy.contains("h1", "Edit file details").should("exist");

    cy.get("label").contains("Edit file title*");
    cy.get("span").contains(
      "This is the file title as seen by members. Try to be as descriptive as possible. Avoid including underscores, hyphens, or document type details (.doc, .pdf...)"
    );
    cy.get("input").should(
      "have.value",
      "London Region NHS England Safeguarding Annual Review"
    );
    cy.get("textarea").should(
      "have.value",
      "London Region NHS England Safeguarding Annual Review.ppt"
    );

    cy.get("input").clear();
    cy.get("span").contains("Title is required").should("not.exist");
    cy.get(`[name="submitButton"]`).click();
    cy.get("span").contains("Title is required").should("exist");

    cy.get("input").type("Some test text");
    cy.get("span").contains("Title is required").should("not.exist");
  });

  it("Shows title, form and press discard to go back to previous page ", () => {
    cy.visit(url);

    cy.contains("h1", "London Region NHS England Safeguarding Annual Review");

    cy.get(`[data-cy="file-options"]`).click();
    cy.get(`[data-cy="edit-file-details"]`).click();

    cy.get("button.nhsuk-button--secondary").contains("Discard").click();
    cy.location("pathname").should("eq", url);
  });
});
