import React from "react";

import { render } from "@testing-library/react";
import { ThemeProvider } from "styled-components";

import theme from "../../lib/fixtures/theme.json";
import NavListItem from "./NavListItem";

describe("<NavListItem/>", () => {
  const folder = { id: "folder-id", title: "folder title" };

  it("renders matching snapshot when not active", () => {
    const container = render(
      <ThemeProvider theme={theme}>
        <NavListItem
          active={false}
          item={folder}
          href="test-link"
          imgSrc="test-svg"
        />
      </ThemeProvider>
    );

    expect(container.asFragment()).toMatchSnapshot();
  });

  it("renders matching snapshot when active", () => {
    const container = render(
      <ThemeProvider theme={theme}>
        <NavListItem
          active={true}
          item={folder}
          href="test-link"
          imgSrc="test-svg"
        />
      </ThemeProvider>
    );

    expect(container.asFragment()).toMatchSnapshot();
  });
});
