import React from "react";

import { GraphQLClient } from "graphql-request";
import { GetServerSideProps } from "next";
import Link from "next/link";

import { Header } from "../../components/Header";
import { PageLayout } from "../../components/PageLayout";
import { requireAuthentication } from "../../lib/auth";
import { getSdk } from "../../lib/generated/graphql";


export const getServerSideProps: GetServerSideProps = requireAuthentication(
  async () => {
    const client = new GraphQLClient(
      "http://workspace-service.workspace-service/graphql"
    );

    const sdk = getSdk(client);

    const { workspaces } = await sdk.GetWorkspaces();

    // const workspaces = directory.workspaces;

    // directory = {
    //   workspaces: [
    //     { title: "something" },
    //     { title: "Hackney" },
    //     { title: "Islington" },
    //   ],
    // };

    return {
      props: {
        workspaces,
      },
    };
  }
);

interface Props {
  workspaces: [{ title: string; id: string }];
}

const WorkspaceDirectory = ({ workspaces }: Props) => {
  return (
    <PageLayout>
      <Header />
      <h1>My workspaces</h1>
      {workspaces.map((workspace) => {
        return (
          <>
            <Link href="/workspaces/[id]" as={`/workspaces/${workspace.id}`}>
              <a>{workspace.title}</a>
            </Link>
            <div>{workspace.id}</div>
          </>
        );
      })}
    </PageLayout>
  );
};

export default WorkspaceDirectory;

// 1. shell script to post test data via GraphQL
// 2. Write graphQL `query GetWorkspaces` and then run yarn generate
// 3. Use graphQL methods in getServerSideProps to fetch data
// 4. Pass down data as props and print out onto page
// 5. Check designs with Kim
