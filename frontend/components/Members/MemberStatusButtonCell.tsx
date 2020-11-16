import React, { FC } from "react";

import { Button, ErrorMessage } from "nhsuk-react-components";
import styled from "styled-components";
import { CombinedError, OperationContext, OperationResult } from "urql";

import {
  User,
  WorkspaceMembership,
  MembershipChange,
  Exact,
  ChangeWorkspaceMembershipMutation,
} from "../../lib/generated/graphql";

type MutationError = {
  user: User;
  error?: CombinedError;
} | null;

type ButtonCellProps = {
  user: User;
  workspaceId: string;
  newRole: WorkspaceMembership;
  changeMembership: (
    variables?:
      | Exact<{
          input: MembershipChange;
        }>
      | undefined,
    context?: Partial<OperationContext> | undefined
  ) => Promise<OperationResult<ChangeWorkspaceMembershipMutation>>;
  mutationError: MutationError;
  setMutationError: React.Dispatch<React.SetStateAction<MutationError>>;
  isAdmin: boolean;
};

const StyledErrorMessage = styled(ErrorMessage)`
  margin-bottom: 0;
`;

const StyledButton = styled(Button)`
  margin-bottom: 20px;
`;

const unpackError = (
  error: CombinedError
): { problem: string; suggestion: string } => {
  const extensions = error.graphQLErrors[0]?.extensions;

  if (!extensions || !extensions.problem || !extensions.suggestion) {
    return {
      problem: "Something went wrong.",
      suggestion: "Try again.",
    };
  }
  return { problem: extensions.problem, suggestion: extensions.suggestion };
};

const RenderedError: FC<CombinedError> = (error) => {
  const { problem, suggestion } = unpackError(error);

  return (
    <>
      <StyledErrorMessage>
        {problem}
        <br />
        {suggestion}
      </StyledErrorMessage>
    </>
  );
};

export const MemberStatusButtonCell: FC<ButtonCellProps> = ({
  user,
  workspaceId,
  newRole,
  changeMembership,
  setMutationError,
  mutationError,
  isAdmin,
}) => (
  <>
    {isAdmin && (
      <StyledButton
        secondary
        onClick={async () => {
          const result = await changeMembership({
            input: {
              workspace: workspaceId,
              user: user.id,
              newRole,
            },
          });
          setMutationError({
            user,
            error: result.error,
          });
        }}
      >
        {newRole === WorkspaceMembership.Admin
          ? "Make Administrator"
          : "Make Member"}
      </StyledButton>
    )}
    {mutationError?.user.id === user.id && mutationError?.error && (
      <RenderedError {...mutationError.error} />
    )}
  </>
);
