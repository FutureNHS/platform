import React from "react";

import { NextPage } from "next";
import { useRouter } from "next/dist/client/router";
import { Input, Form, Button, ErrorMessage } from "nhsuk-react-components";
import { useForm } from "react-hook-form/dist/index.ie11";
import styled from "styled-components";

import { Footer } from "../../../../components/Footer";
import { H2 } from "../../../../components/H2";
import { MainHeading } from "../../../../components/MainHeading";
import { NavHeader } from "../../../../components/NavHeader";
import { Navigation } from "../../../../components/Navigation";
import { PageLayout } from "../../../../components/PageLayout";
import { Permissions } from "../../../../components/Permissions";
import { Textarea } from "../../../../components/Textarea";
import {
  RoleRequired,
  useCreateFolderMutation,
  useGetWorkspaceByIdQuery,
} from "../../../../lib/generated/graphql";
import { useMaxLengthHelper } from "../../../../lib/useMaxLengthHelper";
import withUrqlClient from "../../../../lib/withUrqlClient";

const ContentWrapper = styled.div`
  display: flex;
`;

const FormField = styled.div`
  padding-bottom: 40px;
  #text {
    padding-bottom: 60px;
  }
`;

const PageContent = styled.div`
  ${({ theme }) => `
  background-color: ${theme.colorNhsukWhite};
  min-height: 100vh;
  padding-top: 24px;
  padding-left: 10%;
  padding-right: 10%;
  .nhsuk-form-group {
    margin-bottom: 8px;
  }
  `}
`;

interface FolderInputs {
  title: string;
  description: string;
  roleRequired: RoleRequired;
  server?: never;
}

const StyledButton = styled(Button)`
  margin-left: 12px;
`;

const CreateFolder: NextPage = () => {
  const router = useRouter();
  const workspaceId = (router.query.workspaceId || "unknown").toString();

  const titleMaxLength = useMaxLengthHelper("Title", 100);
  const descriptionMaxLength = useMaxLengthHelper("Description", 250);

  const { errors, handleSubmit, register, setError } = useForm<FolderInputs>();

  const [{ data, fetching, error }] = useGetWorkspaceByIdQuery({
    variables: { id: workspaceId },
  });

  const [, createFolder] = useCreateFolderMutation();

  if (error) return <p> Oh no... {error?.message} </p>;
  if (fetching || !data) return <p>Loading...</p>;

  const backToPreviousPage = () => router.back();

  const onSubmit = async (newFolder: FolderInputs) => {
    try {
      const result = await createFolder({
        ...newFolder,
        workspace: workspaceId,
      });
      if (result.data) {
        router.push(
          `/workspaces/${workspaceId}/folders/${result.data.createFolder.id}`
        );
      } else {
        setError("server", {
          type: "server",
          message: "Error creating folder",
        });
      }
    } catch (err) {
      setError("server", {
        type: "server",
        message: err,
      });
    }
  };

  return (
    <PageLayout>
      <NavHeader />
      <ContentWrapper>
        <Navigation
          workspaceId={workspaceId}
          workspaceTitle={data.workspace.title}
          activeFolder={"active"}
        />
        <PageContent>
          <MainHeading>Create a folder</MainHeading>
          <H2 title="Folder details" />
          <p> Fields marked with * are mandatory.</p>
          <Form onSubmit={handleSubmit(onSubmit)}>
            <FormField>
              <Input
                name="title"
                onChange={titleMaxLength.onChange}
                id="title"
                label="Enter folder title*"
                hint="The title of your folder should accurately reflect its content or audience"
                inputRef={register({
                  required: {
                    value: true,
                    message: "Title is required",
                  },
                  ...titleMaxLength.validation,
                })}
                error={errors.title?.message}
              />
              {titleMaxLength.remainingText("title")}
            </FormField>

            <FormField>
              <Textarea
                name="description"
                onChange={descriptionMaxLength.onChange}
                id="description"
                label="Description"
                error={errors.description?.message}
                hint="This is the description as seen by users"
                inputRef={register(descriptionMaxLength.validation)}
              />
              {descriptionMaxLength.remainingText("description")}
            </FormField>
            <FormField>
              <Permissions inputRef={register()} />
            </FormField>
            <Button type="submit" name="submitButton">
              Save and complete
            </Button>
            <StyledButton secondary type="button" onClick={backToPreviousPage}>
              Discard
            </StyledButton>
            {errors.server && (
              <ErrorMessage>{errors.server.message}</ErrorMessage>
            )}
          </Form>
        </PageContent>
      </ContentWrapper>
      <Footer />
    </PageLayout>
  );
};

export default withUrqlClient(CreateFolder);
