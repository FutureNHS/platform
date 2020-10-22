import React, { FC, useState } from "react";

import { BlockBlobClient } from "@azure/storage-blob";
import { useRouter } from "next/dist/client/router";
import { Input, Form, Button } from "nhsuk-react-components";
import { useForm, useFieldArray } from "react-hook-form/dist/index.ie11";
import styled from "styled-components";
import { Client } from "urql";

import { StatusTag } from "../../components/StatusTag";
import { Textarea } from "../../components/Textarea";
import {
  FileUploadUrlsDocument,
  useCreateFileMutation,
} from "../../lib/generated/graphql";

interface FormData {
  files: FileList;
  fileData: Array<{ title: string; description: string }>;
}
interface Props {
  workspaceId: string;
  folderId: string;
  urqlClient: Client;
}
const StyledInput = styled(Input)`
  border: none;
  margin-bottom: 16px;
`;

const StyledTag = styled.p`
  margin-bottom: 40px;
`;

const StyledButton = styled(Button)`
  margin-left: 12px;
`;

const StyledFileInfoBox = styled.div`
  ${({ theme }) => `
background-color: ${theme.colorNhsukGrey5};
`}
  margin-bottom: 24px;
`;

const FormField = styled.div`
  padding: 16px 20px;

  #text {
    padding-bottom: 60px;
  }
`;

const StyledHeadingSection = styled.div`
  display: flex;
  direction: column;
  justify-content: space-between;
`;

const StyledFileName = styled.h4`
  width: 225px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
`;

const UploadFileForm: FC<Props> = ({ workspaceId, folderId, urqlClient }) => {
  const {
    register,
    handleSubmit,
    errors,
    setError,
    control,
    setValue,
  } = useForm<FormData>();
  const [, createFile] = useCreateFileMutation();

  const router = useRouter();
  const backToPreviousPage = () => router.back();

  const [results, setResults] = useState<Array<Boolean>>([]);
  const [isDisabled, disableButton] = useState<boolean>(false);
  const [remainingChars, setRemainingChars] = useState<{
    [key: string]: number;
  }>({});

  const { fields } = useFieldArray({
    control,
    name: "fileData",
  });

  const handleRemainingChars = (fieldType: "title" | "description") => (
    event: React.FormEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const maxChars = MAX_CHARS[fieldType];
    setRemainingChars({
      ...remainingChars,
      [event.currentTarget.name]: maxChars - event.currentTarget.value.length,
    });
  };

  const MAX_CHARS: { [key: string]: number } = {
    title: 50,
    description: 250,
  };

  const handleFiles = (files: FileList | null) => {
    if (files === null) {
      return;
    }
    const filenames = Object.values(files).map((file) => file.name);

    setValue(
      "fileData",
      filenames.map((name) => ({
        title: name,
        description: "",
      }))
    );
  };

  const onSubmit = async (formData: FormData) => {
    disableButton(true);
    try {
      const { error, data } = await urqlClient
        .query(FileUploadUrlsDocument, { count: formData.files.length })
        .toPromise();
      if (error) {
        throw new Error(`Failed to get upload URL: ${error.toString()}`);
      }

      if (data) {
        const results: Array<boolean> = await Promise.all(
          data.fileUploadUrls.map(
            async (url: string, index: number): Promise<boolean> => {
              try {
                const blobClient = new BlockBlobClient(url);
                const uploadResponse = await blobClient.uploadBrowserData(
                  formData.files[index],
                  {
                    maxSingleShotSize: 4 * 1024 * 1024,
                  }
                );

                if (uploadResponse.errorCode) {
                  throw new Error(
                    `Failed to upload file: ${uploadResponse.errorCode}`
                  );
                }
                const { name: fileName, type: fileType } = formData.files[
                  index
                ];
                const newTitle = formData.fileData[index].title;

                const setMetaResponse = await blobClient.setMetadata({
                  newTitle,
                  fileName,
                });

                if (setMetaResponse.errorCode) {
                  throw new Error(
                    `Failed to set file metadata: ${setMetaResponse.errorCode}`
                  );
                }

                const file = await createFile({
                  newFile: {
                    description: formData.fileData[index].description,
                    fileName,
                    fileType,
                    folder: folderId,
                    temporaryBlobStoragePath: url,
                    title: newTitle,
                  },
                });

                if (file.error) {
                  throw new Error(
                    `Failed to save file: ${file.error?.message}`
                  );
                }
                return true;
              } catch (error) {
                setError(`fileData[${index}].title`, {
                  type: "server",
                  message: error.toString(),
                });
                return false;
              }
            }
          )
        );
        if (results.every(Boolean)) {
          router.push(`/workspaces/${workspaceId}/folders/${folderId}`);
        } else {
          setResults(results);
        }
      }
    } catch (error) {
      setError("files", {
        type: "server",
        message: error.toString(),
      });
    }
  };
  return (
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
      <StyledTag>
        All uploaded content must conform to the platform&apos;s{" "}
        <a href="#">Terms and Conditions</a>.
      </StyledTag>
      {fields.map((item, index) => {
        return (
          <StyledFileInfoBox key={item.id}>
            <FormField>
              <StyledHeadingSection>
                <StyledFileName>{item.title}</StyledFileName>
                {results.length > 0 && (
                  <StatusTag
                    successStatus={results[index]}
                    successMessage="uploaded"
                    failedMessage="failed"
                  />
                )}
              </StyledHeadingSection>
              <Input
                type="text"
                name={`fileData[${index}].title`}
                defaultValue={item.title}
                label="Enter file title*"
                hint="The title of your file should accurately reflect its content or audience"
                onChange={handleRemainingChars("title")}
                inputRef={register({
                  required: {
                    value: true,
                    message: "Title is required",
                  },
                  maxLength: {
                    value: MAX_CHARS.title,
                    message: `Title must be a maximum of ${MAX_CHARS.title} characters`,
                  },
                })}
                error={errors.fileData?.[index]?.title?.message}
              />
              {`${
                remainingChars[`fileData[${index}].title`] || MAX_CHARS.title
              } characters remaining`}
            </FormField>
            <FormField>
              <Textarea
                name={`fileData[${index}].description`}
                label="Enter description (optional)"
                error={errors.fileData?.[index]?.description?.message}
                hint="This is the description as seen by users"
                onChange={handleRemainingChars("description")}
                inputRef={register({
                  maxLength: {
                    value: MAX_CHARS.description,
                    message: `Description must be a maximum of ${MAX_CHARS.description} characters`,
                  },
                })}
              />
              {`${
                remainingChars[`fileData[${index}].description`] ||
                MAX_CHARS.description
              } characters remaining`}
            </FormField>
          </StyledFileInfoBox>
        );
      })}
      <Button type="submit" name="submitButton" disabled={isDisabled}>
        Upload and continue
      </Button>
      <StyledButton secondary type="button" onClick={backToPreviousPage}>
        Discard
      </StyledButton>
    </Form>
  );
};

export default UploadFileForm;
