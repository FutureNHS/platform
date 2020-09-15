import React from "react";

import { render } from "@testing-library/react";

import WorkspaceDirectoryItem from "./WorkspaceDirectoryItem";

test("snapshot of component", () => {
  const title = "workspace name";
  const id = "test123";

  const { asFragment } = render(
    <WorkspaceDirectoryItem title={title} id={id} />
  );

  expect(asFragment()).toMatchSnapshot();
});
