import React from "react";

import { render } from "@testing-library/react";

import MainHeading from "./MainHeading";

test("takes a snapshot of the component", () => {
  const children = "example heading";
  const { asFragment } = render(<MainHeading>{children}</MainHeading>);

  expect(asFragment()).toMatchSnapshot();
});

test("takes a snapshot with the prop", () => {
  const children = "example heading";
  const { asFragment } = render(
    <MainHeading withBorder>{children}</MainHeading>
  );

  expect(asFragment()).toMatchSnapshot();
});
