import React, { FC, useState } from "react";

import { GraphQLClient } from "graphql-request";
import { GetServerSideProps } from "next";
import { useRouter } from "next/dist/client/router";
import { Input, Form, Button } from "nhsuk-react-components";
import { useForm } from "react-hook-form";
import styled from "styled-components";

import { Header } from "../../../../components/Header";
import { MainHeading } from "../../../../components/MainHeading";
import { Navigation } from "../../../../components/Navigation";
import { PageLayout } from "../../../../components/PageLayout";
import { Textarea } from "../../../../components/Textarea";
import { requireAuthentication } from "../../../../lib/auth";
import { getSdk, Folder } from "../../../../lib/generated/graphql";

export const getServerSideProps: GetServerSideProps<Props> = requireAuthentication(
  async (context) => {
    const client = new GraphQLClient("http://localhost:3030/graphql");
    const sdk = getSdk(client);
    const workspaceId = (context.params?.workspaceId as string) || "";

    const { foldersByWorkspace } = await sdk.FoldersByWorkspace({
      workspace: workspaceId,
    });
    const { workspace } = await sdk.GetWorkspaceByID({ id: workspaceId });

    return {
      props: {
        workspaceFolders: foldersByWorkspace,
        workspace,
      },
    };
  }
);

const ContentWrapper = styled.div`
  display: flex;
`;

const FormField = styled.div`
  padding-bottom: 40px;
  #text {
    padding-bottom: 60px;
  }
`;

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

const MAX_CHARS: { [key: string]: number } = {
  title: 100,
  description: 250,
};

interface Workspace {
  id: string;
  title: string;
  description: string;
}

interface Props {
  workspaceFolders: Array<Pick<Folder, "id" | "title">>;
  workspace: Pick<Workspace, "id" | "title">;
}

const CreateFolder: FC<Props> = ({ workspaceFolders, workspace }) => {
  const [remainingChars, setRemainingChars] = useState({
    title: null,
    description: null,
  });

  const { errors, handleSubmit, register } = useForm();
  const router = useRouter();

  const backToPreviousPage = () => router.back();

  const onSubmit = async (data: Folder) => {
    try {
      const { id: workspaceId } = workspace;
      const newFolderDetails = { ...data, workspace: workspaceId };
      const client = new GraphQLClient("/api/graphql");
      const sdk = getSdk(client);
      const newFolder = await sdk.CreateFolderMutation(newFolderDetails);
      const {
        createFolder: { id: folderId },
      } = newFolder;
      router.push(`/workspaces/${workspaceId}/folders/${folderId}`);
    } catch (error) {
      console.log("Create folder failed", error);
      window.alert("Error creating folder, failed");
    }
  };

  const handleCharNumber = (
    event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    setRemainingChars({
      ...remainingChars,
      [event.currentTarget.name]:
        MAX_CHARS[event.currentTarget.name] - event.currentTarget.value.length,
    });
  };

  return (
    <>
      <PageLayout>
        <Header />
        <ContentWrapper>
          <Navigation
            folders={workspaceFolders}
            workspace={workspace}
            activeFolder={"active"}
          />
          <PageContent>
            <MainHeading withBorder>Create a folder</MainHeading>
            <h2>Folder details</h2>
            <p> Fields marked with * are mandatory.</p>
            <Form onSubmit={handleSubmit(onSubmit)}>
              <FormField>
                <Input
                  name="title"
                  onChange={handleCharNumber}
                  id="title"
                  label="Enter folder title*"
                  hint="The title of your folder should accurately reflect its content or audience"
                  inputRef={register({
                    required: true,
                    maxLength: MAX_CHARS.title,
                  })}
                  error={
                    errors.title &&
                    `Folder name is required and cannot be longer than ${MAX_CHARS.title} characters`
                  }
                />
                {`${
                  remainingChars.title || MAX_CHARS.title
                } characters remaining`}
              </FormField>

              <FormField>
                <Textarea
                  name="description"
                  onChange={handleCharNumber}
                  id="description"
                  label="Description"
                  error={
                    errors.description &&
                    `Description must be a maximum of ${MAX_CHARS.description} characters`
                  }
                  hint="This is the description as seen by users"
                  inputRef={register({
                    required: false,
                    maxLength: MAX_CHARS.description,
                  })}
                />
                {`${
                  remainingChars.description || MAX_CHARS.description
                } characters remaining`}
              </FormField>
              <Button type="submit">Save and complete</Button>
              <Button secondary type="button" onClick={backToPreviousPage}>
                Discard
              </Button>
            </Form>
          </PageContent>
        </ContentWrapper>
      </PageLayout>
    </>
  );
};

export default CreateFolder;
