import React from "react";

import { render } from "@testing-library/react";

import EmailLink from "./EmailLink";

describe(EmailLink, () => {
  it("renders correctly", () => {
    const container = render(
      <EmailLink emailAddress="my-email-address@email.com" />
    );

    expect(container.asFragment()).toMatchSnapshot();
  });
});
