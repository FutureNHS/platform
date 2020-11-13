describe("Update folder page", () => {
  context("Desktop - 990px+", () => {
    beforeEach(() => {
      cy.viewport(1200, 1080);
    });
    it("Shows title, renders form, selects permissions, and submits", () => {
      cy.visit(
        `/workspaces/01bb9a4d-2977-4c43-b28c-2a72b4eda453/folders/b382ae4b-4dd2-489b-8afb-9ba8c37491af/update-folder`
      );
      cy.contains("h1", "Edit folder");

      cy.get("#title").type("Folder details");
      cy.get("#description").type(" Additional folder text.");
      cy.get(`[data-cy="permissions-button"]`).click();
      cy.get("#workspace-members").click();
      cy.get("form").submit();
    });

    it("Can be accessed by the nav bar, shows title, form and can be discarded", () => {
      const start = `/workspaces/01bb9a4d-2977-4c43-b28c-2a72b4eda453`;
      cy.visit(start);
      cy.get("ul>li")
        .eq(3)
        .within(() => {
          cy.get("button").click();
          cy.get("a").eq(2).click();
        });

      cy.contains("h1", "Edit folder");
      cy.get("button").contains("Discard").click();
      cy.location("pathname").should("eq", start);
    });

    it("Can access edit page from folder page tooltip", () => {
      cy.visit(
        `/workspaces/01bb9a4d-2977-4c43-b28c-2a72b4eda453/folders/b382ae4b-4dd2-489b-8afb-9ba8c37491af`
      );
      cy.get("section")
        .eq(1)
        .within(() => {
          cy.get("div>button").click();
          cy.get("a").eq(1).click();
        });

      cy.contains("h1", "Edit folder");
      cy.get("button").contains("Discard").click();
    });
  });

  context("Mobile - 320px", () => {
    beforeEach(() => {
      cy.viewport(320, 720);
    });
    it("Shows title, renders form, selects permissions, and submits", () => {
      cy.visit(
        `/workspaces/01bb9a4d-2977-4c43-b28c-2a72b4eda453/folders/b382ae4b-4dd2-489b-8afb-9ba8c37491af/update-folder`
      );
      cy.contains("h1", "Edit folder");

      cy.get("#title").type("Folder details");
      cy.get("#description").type(" Additional folder text.");
      cy.get(`[data-cy="permissions-button"]`).click();
      cy.get("#workspace-members").click();
      cy.get("form").submit();
    });
  });
});
