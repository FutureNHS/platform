import React, { FC } from "react";

import { Button } from "nhsuk-react-components";
import { OperationContext, OperationResult } from "urql";

import {
  User,
  WorkspaceMembership,
  MembershipChange,
  Exact,
  ChangeWorkspaceMembershipMutation,
} from "../../lib/generated/graphql";

type MutationError = {
  user: User;
  error?: string | undefined;
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
};

export const MemberStatusButtonCell: FC<ButtonCellProps> = ({
  user,
  workspaceId,
  newRole,
  changeMembership,
  setMutationError,
  mutationError,
}) => (
  <>
    <Button
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
          error: result.error?.message,
        });
      }}
    >
      {newRole === WorkspaceMembership.Admin
        ? "Make Administrator"
        : "Make Member"}
    </Button>
    {mutationError?.user.id === user.id && mutationError?.error && (
      <p> Oh no... {mutationError.error} </p>
    )}
  </>
);
