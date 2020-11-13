import React, { useEffect } from "react";

import { NextPage } from "next";
import { useRouter } from "next/dist/client/router";
import { Input, Form, Button, ErrorMessage } from "nhsuk-react-components";
import { useForm } from "react-hook-form/dist/index.ie11";
import styled from "styled-components";

import { Error as ErrorComponent } from "../../../../../components/Error";
import { Footer } from "../../../../../components/Footer";
import { H2 } from "../../../../../components/H2";
import { MainHeading } from "../../../../../components/MainHeading";
import { NavHeader } from "../../../../../components/NavHeader";
import { Navigation } from "../../../../../components/Navigation";
import { PageLayout } from "../../../../../components/PageLayout";
import { Permissions } from "../../../../../components/Permissions";
import { Textarea } from "../../../../../components/Textarea";
import {
  useUpdateFolderMutation,
  useGetFolderByIdQuery,
  useGetWorkspaceByIdQuery,
  RoleRequired,
} from "../../../../../lib/generated/graphql";
import { useMaxLengthHelper } from "../../../../../lib/useMaxLengthHelper";
import withUrqlClient from "../../../../../lib/withUrqlClient";

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

const StyledButton = styled(Button)`
  margin-left: 12px;
`;

interface FolderInputs {
  title: string;
  description: string;
  roleRequired: RoleRequired;
  server?: never;
}

const UpdateFolder: NextPage = () => {
  const router = useRouter();
  const { workspaceId, folderId } = router.query;

  if (folderId === undefined || Array.isArray(folderId)) {
    throw new Error("folderId required in URL");
  }

  if (workspaceId === undefined || Array.isArray(workspaceId)) {
    throw new Error("workspaceId required in URL");
  }

  const [folder] = useGetFolderByIdQuery({
    variables: { id: folderId },
  });

  const [, updateFolder] = useUpdateFolderMutation();

  const accessPermitted =
    folder.error?.graphQLErrors[0]?.extensions?.details === "ACCESS_DENIED"
      ? false
      : true;

  const titleMaxLength = useMaxLengthHelper("Title", 100);
  const descriptionMaxLength = useMaxLengthHelper("Description", 250);

  const { errors, handleSubmit, register, reset, setError } = useForm<
    FolderInputs
  >();

  useEffect(() => {
    if (folder.data) {
      reset({
        title: folder.data.folder.title,
        description: folder.data.folder.description,
        roleRequired: folder.data.folder.roleRequired,
      });
    }
  }, [folder]);

  const [{ data, fetching, error }] = useGetWorkspaceByIdQuery({
    variables: { id: workspaceId },
  });

  if (error) return <p> Oh no... {error?.message} </p>;
  if (fetching || !data) return <p>Loading...</p>;

  const backToPreviousPage = () => router.back();

  const onSubmit = async (folder: FolderInputs) => {
    try {
      const result = await updateFolder({ ...folder, id: folderId });
      if (result.data) {
        router.push(
          `/workspaces/${workspaceId}/folders/${result.data.updateFolder.id}`
        );
      } else {
        setError("server", {
          type: "server",
          message: "Error updating folder",
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
          activeFolder={folderId}
        />
        <PageContent>
          {accessPermitted && folder.data ? (
            <>
              <MainHeading>Edit folder</MainHeading>
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
                <StyledButton
                  secondary
                  type="button"
                  onClick={backToPreviousPage}
                >
                  Discard
                </StyledButton>
                {errors.server && (
                  <ErrorMessage>{errors.server.message}</ErrorMessage>
                )}
              </Form>
            </>
          ) : (
            <>
              <ErrorComponent
                title="You do not have permission to do this."
                description="Please contact a Workspace Administrator to request access
                to this folder."
              />
            </>
          )}
        </PageContent>
      </ContentWrapper>
      <Footer />
    </PageLayout>
  );
};

export default withUrqlClient(UpdateFolder);
