import React from "react";

import { render } from "@testing-library/react";
import { GraphQLError } from "graphql";
import { ThemeProvider } from "styled-components";
import { CombinedError } from "urql";

import theme from "../../lib/fixtures/theme.json";
import { WorkspaceMembership } from "../../lib/generated/graphql";
import { MemberStatusButtonCell } from "./MemberStatusButtonCell";

const user = {
  id: "asdf",
  authId: "ghjkl",
  name: "Mr T",
  emailAddress: "t@t.com",
  isPlatformAdmin: false,
};

const buttonCellProps = {
  workspaceId: "id",
  changeMembership: (() => {}) as any,
  mutationError: null,
  setMutationError: (() => {}) as any,
  user,
  newRole: WorkspaceMembership.Admin,
  isAdmin: true,
};

const error: CombinedError = { name: "", message: "", graphQLErrors: [] };

test("renders make admin button", () => {
  const { asFragment } = render(
    <MemberStatusButtonCell
      {...buttonCellProps}
      newRole={WorkspaceMembership.Admin}
    />
  );

  expect(asFragment()).toMatchSnapshot();
});

test("renders make member button", () => {
  const { asFragment } = render(
    <ThemeProvider theme={theme}>
      <MemberStatusButtonCell
        {...buttonCellProps}
        newRole={WorkspaceMembership.NonAdmin}
      />
    </ThemeProvider>
  );

  expect(asFragment()).toMatchSnapshot();
});

test("does not render make admin button when isAdmin status is false", () => {
  const { asFragment } = render(
    <MemberStatusButtonCell
      {...buttonCellProps}
      newRole={WorkspaceMembership.Admin}
      isAdmin={false}
    />
  );

  expect(asFragment()).toMatchSnapshot();
});

test("does not render make member button when isAdmin status is false", () => {
  const { asFragment } = render(
    <MemberStatusButtonCell
      {...buttonCellProps}
      newRole={WorkspaceMembership.NonAdmin}
      isAdmin={false}
    />
  );

  expect(asFragment()).toMatchSnapshot();
});

test("renders error message when user matches one in error", () => {
  const { asFragment } = render(
    <ThemeProvider theme={theme}>
      <MemberStatusButtonCell
        {...buttonCellProps}
        newRole={WorkspaceMembership.Admin}
        mutationError={{ user, error }}
      />
    </ThemeProvider>
  );

  expect(asFragment()).toMatchSnapshot();
});

test("specific error message when it exists", () => {
  const specificError: CombinedError = {
    ...error,
    graphQLErrors: [
      ({
        extensions: { problem: "I don't like you.", suggestion: "Go away." },
      } as unknown) as GraphQLError,
    ],
  };
  const { asFragment } = render(
    <ThemeProvider theme={theme}>
      <MemberStatusButtonCell
        {...buttonCellProps}
        newRole={WorkspaceMembership.Admin}
        mutationError={{ user, error: specificError }}
      />
    </ThemeProvider>
  );

  expect(asFragment()).toMatchSnapshot();
});
test("renders no error message when user does not match error", () => {
  const { asFragment } = render(
    <ThemeProvider theme={theme}>
      <MemberStatusButtonCell
        {...buttonCellProps}
        newRole={WorkspaceMembership.Admin}
        mutationError={{
          user: { ...user, id: "someone-else" },
          error,
        }}
      />
    </ThemeProvider>
  );

  expect(asFragment()).toMatchSnapshot();
});
