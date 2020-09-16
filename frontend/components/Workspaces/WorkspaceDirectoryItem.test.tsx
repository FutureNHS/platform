import React from "react";

import { render } from "@testing-library/react";
import { ThemeProvider } from "styled-components";

import theme from "../../lib/fixtures/theme.json";
import WorkspaceDirectoryItem from "./WorkspaceDirectoryItem";

test("snapshot of component", () => {
  const title = "workspace name";
  const id = "test123";

  const { asFragment } = render(
    <ThemeProvider theme={theme}>
      <WorkspaceDirectoryItem title={title} id={id} />
    </ThemeProvider>
  );

  expect(asFragment()).toMatchSnapshot();
});
