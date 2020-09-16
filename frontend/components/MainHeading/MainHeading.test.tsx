import React from "react";

import { render } from "@testing-library/react";
import { ThemeProvider } from "styled-components";

import theme from "../../lib/fixtures/theme.json";
import MainHeading from "./MainHeading";

test("takes a snapshot of the component", () => {
  const children = "example heading";
  const { asFragment } = render(<MainHeading>{children}</MainHeading>);

  expect(asFragment()).toMatchSnapshot();
});

test("takes a snapshot with the prop", () => {
  const children = "example heading";
  const { asFragment } = render(
    <ThemeProvider theme={theme}>
      <MainHeading withBorder>{children}</MainHeading>
    </ThemeProvider>
  );

  expect(asFragment()).toMatchSnapshot();
});
