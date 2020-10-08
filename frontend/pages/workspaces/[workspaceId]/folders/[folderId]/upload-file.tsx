import React, { useState } from "react";

import { BlockBlobClient } from "@azure/storage-blob";
import { NextPage } from "next";
import { useRouter } from "next/dist/client/router";
import { Input, Form, Button, ErrorMessage } from "nhsuk-react-components";
import { useForm } from "react-hook-form/dist/index.ie11";
import styled from "styled-components";
import { Client } from "urql";

import { MainHeading } from "../../../../../components/MainHeading";
import { NavHeader } from "../../../../../components/NavHeader";
import { Navigation } from "../../../../../components/Navigation";
import { PageLayout } from "../../../../../components/PageLayout";
import { Textarea } from "../../../../../components/Textarea";
import {
  FileUploadUrlsDocument,
  useCreateFileMutation,
  useGetFolderByIdQuery,
  useGetWorkspaceByIdQuery,
} from "../../../../../lib/generated/graphql";
import withUrqlClient from "../../../../../lib/withUrqlClient";

const ContentWrapper = styled.div`
  display: flex;
`;

const StyledInput = styled(Input)`
  border: none;
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

const StyledFileInfoBox = styled.div`
  ${({ theme }) => `
background-color: ${theme.colorNhsukGrey4};
`}
  margin-bottom: 10px;
`;

const MAX_CHARS: { [key: string]: number } = {
  title: 50,
};

type FormData = {
  title: string;
  files: FileList;
};

const UploadFile: NextPage<any> = ({ urqlClient }: { urqlClient: Client }) => {
  const router = useRouter();
  const workspaceId = (router.query.workspaceId || "unknown").toString();
  const folderId = (router.query.folderId || "unknown").toString();

  const [remainingChars, setRemainingChars] = useState({
    title: null,
    description: null,
  });

  const { register, handleSubmit, errors, setError } = useForm<FormData>();

  const [workspace] = useGetWorkspaceByIdQuery({
    variables: { id: workspaceId },
  });
  const [folder] = useGetFolderByIdQuery({
    variables: { id: folderId },
  });
  const [, createFile] = useCreateFileMutation();

  if (workspace.error || folder.error)
    return (
      <p>
        {" "}
        Oh no... {workspace.error?.message} {folder.error?.message}{" "}
      </p>
    );

  const backToPreviousPage = () => router.back();

  const onSubmit = async ({ title, files }: FormData) => {
    console.log("**********FORMDATA", title);
    console.log("********FILES", files);
    try {
      const { error, data } = await urqlClient
        .query(FileUploadUrlsDocument, { count: files.length })
        .toPromise();
      if (error) {
        throw new Error(`Failed to get upload URL: ${error.toString()}`);
      }
      console.log("********DATA", data.fileUploadUrls);
      if (data) {
        data.fileUploadUrls.map(async (url, index) => {
          const blobClient = new BlockBlobClient(url);
          const uploadResponse = await blobClient.uploadBrowserData(
            files[index],
            {
              maxSingleShotSize: 4 * 1024 * 1024,
            }
          );

          console.log("********UPLOADRESPONSE", uploadResponse);
          if (uploadResponse.errorCode) {
            throw new Error(
              `Failed to upload file: ${uploadResponse.errorCode}`
            );
          }
          const { name: fileName, type: fileType } = files[index];
          const setMetaResponse = await blobClient.setMetadata({
            title,
            fileName,
          });
          console.log("********METARESPONSE", setMetaResponse);
          if (setMetaResponse.errorCode) {
            throw new Error(
              `Failed to set file metadata: ${setMetaResponse.errorCode}`
            );
          }

          const file = await createFile({
            newFile: {
              description: "TBD", // TODO
              fileName,
              fileType,
              folder: folderId,
              temporaryBlobStoragePath: url,
              title,
            },
          });

          console.log("******file", file);
          if (file.error) {
            throw new Error(`Failed to save file: ${file.error?.message}`);
          }
        });
        // need to show loading and files need to be uploaded before router.push
        router.push(`/workspaces/${workspaceId}/folders/${folderId}`);
      }
    } catch (error) {
      setError("files", {
        type: "server",
        message: error.toString(),
      });
    }
  };
  const FormField = styled.div`
    padding-bottom: 40px;
    #text {
      padding-bottom: 60px;
    }
  `;

  const handleCharNumber = (
    event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    setRemainingChars({
      ...remainingChars,
      [event.currentTarget.name]:
        MAX_CHARS[event.currentTarget.name] - event.currentTarget.value.length,
    });
  };

  const [names, setFileNames] = useState<string[]>([]);

  const handleFiles = (files: FileList) => {
    const filenames = Object.values(files).map((file) => file.name); //['name1', 'name2']
    setFileNames([...names, ...filenames]);
  };

  return (
    <PageLayout>
      <NavHeader />
      <ContentWrapper>
        <Navigation
          workspaceId={workspaceId}
          workspaceTitle={
            workspace.fetching
              ? "Loading..."
              : workspace.data?.workspace.title || "No title!"
          }
          activeFolder={folderId}
        />
        <PageContent>
          <MainHeading withBorder>
            {folder.fetching
              ? "Loading..."
              : folder.data?.folder.title || "No title!"}
          </MainHeading>
          <p> Fields marked with * are mandatory.</p>
          <Form id="filesUploadForm" onSubmit={handleSubmit(onSubmit)}>
            <StyledInput
              type="file"
              name="files"
              id="files"
              hint="Maximum 5 files"
              multiple
              onChange={(e) => handleFiles(e.currentTarget.files)}
              inputRef={register({
                required: {
                  value: true,
                  message: "Please select one or more files",
                },
              })}
              aria-invalid={errors.files ? "true" : "false"}
              error={errors.files?.message}
            />

            <p>
              All uploaded content must conform to the platform&apos;s{" "}
              <a href="#">Terms and Conditions</a>.
            </p>
            {names &&
              names.map((name, index) => {
                return (
                  <>
                    <StyledFileInfoBox>
                      <FormField>
                        <Input
                          name="title"
                          onChange={handleCharNumber}
                          value={name}
                          key={index}
                          label="Enter file title*"
                          hint="The title of your file should accurately reflect its content or audience"
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
                    </StyledFileInfoBox>
                  </>
                );
              })}
            <Button type="submit" name="submitButton">
              Uploads and continue
            </Button>
            <StyledButton secondary type="button" onClick={backToPreviousPage}>
              Discard
            </StyledButton>
            {errors.title && (
              <ErrorMessage>{errors.title.message}</ErrorMessage>
            )}
            {errors.files && (
              <ErrorMessage>{errors.files.message}</ErrorMessage>
            )}
          </Form>
        </PageContent>
      </ContentWrapper>
    </PageLayout>
  );
};

export default withUrqlClient(UploadFile);
