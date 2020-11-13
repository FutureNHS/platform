import React from "react";

import { render } from "@testing-library/react";

import Error from "./Error";

describe(Error, () => {
  it("renders matching snapshot", () => {
    const props = {
      title: "Insufficient Permissions",
      description: "Please contact your administrator",
    };
    const container = render(<Error {...props} />);

    expect(container.asFragment()).toMatchSnapshot();
  });
});
