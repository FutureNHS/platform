import React, { FC, useState } from "react";

import { NextPage } from "next";
import { useRouter } from "next/router";
import styled from "styled-components";

import { Footer } from "../../../components/Footer";
import { Head } from "../../../components/Head";
import { MainHeading } from "../../../components/MainHeading";
import { MemberStatusButtonCell } from "../../../components/Members";
import { NavHeader } from "../../../components/NavHeader";
import { Navigation } from "../../../components/Navigation";
import { PageLayout } from "../../../components/PageLayout";
import { ResponsiveTable } from "../../../components/Table";
import {
  User,
  useGetWorkspaceWithMembersQuery,
  useChangeWorkspaceMembershipMutation,
  WorkspaceMembership,
} from "../../../lib/generated/graphql";
import withUrqlClient from "../../../lib/withUrqlClient";

const PageContent = styled.section`
  flex-grow: 3;
  min-height: 100vh;
  padding-top: 24px;
  padding-left: 10%;
  padding-right: 10%;
  ${({ theme }) => `
  background-color: ${theme.colorNhsukWhite};
  `}
`;

const ContentWrapper = styled.div`
  display: flex;
`;

const CountSentence = styled.p`
  margin: 0;
  padding-top: 16px;
`;

const nameCell: FC<User> = ({ name }) => <div>{name}</div>;
const emailAddressCell: FC<User> = ({ emailAddress }) => (
  <a
    href={`mailto:${encodeURI(emailAddress)}`}
    target="_blank"
    rel="noreferrer"
  >
    {emailAddress}
  </a>
);

const WorkspaceMembersPage: NextPage = () => {
  const router = useRouter();
  const { workspaceId } = router.query;
  const id = (workspaceId || "unknown").toString();

  const [{ data, fetching, error }] = useGetWorkspaceWithMembersQuery({
    variables: { id },
  });

  const [, changeMembership] = useChangeWorkspaceMembershipMutation();
  const [mutationError, setMutationError] = useState<{
    user: User;
    error?: string;
  } | null>(null);

  const buttonCellProps = {
    workspaceId: id,
    changeMembership,
    mutationError,
    setMutationError,
  };
  const makeAdminButtonCell = (user: User) =>
    MemberStatusButtonCell({
      ...buttonCellProps,
      user,
      newRole: WorkspaceMembership.Admin,
    });
  const makeNonAdminButtonCell = (user: User) =>
    MemberStatusButtonCell({
      ...buttonCellProps,
      user,
      newRole: WorkspaceMembership.NonAdmin,
    });

  const workspaceTitle = (!fetching && data?.workspace.title) || "Loading...";

  return (
    <>
      <Head title={workspaceTitle} />
      <PageLayout>
        <NavHeader />
        <ContentWrapper>
          <Navigation workspaceId={id} workspaceTitle={workspaceTitle} />
          <PageContent>
            <MainHeading>Workspace members</MainHeading>
            <p>This is a list of all workspace members.</p>
            {error && <p> Oh no... {error?.message} </p>}
            <>
              {data && (
                <>
                  <CountSentence>
                    Showing all administrators ({data.workspace.admins.length})
                  </CountSentence>
                  <ResponsiveTable
                    tableHeading="Administrators"
                    columns={[
                      { heading: "Name of user", content: nameCell },
                      { heading: "Email", content: emailAddressCell },
                    ]}
                    extraDetails={[
                      {
                        heading: "Permissions",
                        // eslint-disable-next-line react/display-name
                        content: () => <>Administrator</>,
                      },
                      {
                        // eslint-disable-next-line react/display-name
                        content: () => (
                          <>
                            Administrators can manager folder, members and
                            workspace details
                          </>
                        ),
                      },
                      {
                        content: makeNonAdminButtonCell,
                      },
                    ]}
                    data={data.workspace.admins as User[]}
                  />

                  <CountSentence>
                    Showing all members ({data.workspace.members.length})
                  </CountSentence>
                  <ResponsiveTable
                    tableHeading="Members"
                    columns={[
                      { heading: "Name of User", content: nameCell },
                      { heading: "Email", content: emailAddressCell },
                    ]}
                    extraDetails={[
                      {
                        heading: "Permissions",
                        // eslint-disable-next-line react/display-name
                        content: () => <>Member</>,
                      },
                      {
                        // eslint-disable-next-line react/display-name
                        content: () => (
                          <>
                            Administrators can manager folder, members and
                            workspace details
                          </>
                        ),
                      },
                      {
                        content: makeAdminButtonCell,
                      },
                    ]}
                    data={data.workspace.members as User[]}
                  />
                </>
              )}
            </>
          </PageContent>
        </ContentWrapper>
        <Footer />
      </PageLayout>
    </>
  );
};

export default withUrqlClient(WorkspaceMembersPage);
