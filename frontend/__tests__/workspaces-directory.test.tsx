import React from "react";

import { render } from "@testing-library/react";
import { ThemeProvider } from "styled-components";

import theme from "../lib/fixtures/theme.json";
import WorkspaceDirectory from "../pages/workspaces/directory";

test("takes a snapshot of the component", () => {
  const workspaces = [
    { title: "hospital", id: "1" },
    { title: "pharmacy", id: "2" },
    { title: "ambulance", id: "3" },
  ];
  const { asFragment } = render(
    <ThemeProvider theme={theme}>
      <WorkspaceDirectory workspaces={workspaces} />
    </ThemeProvider>
  );
  expect(asFragment()).toMatchSnapshot();
});
