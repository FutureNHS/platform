import React, { FC, useState } from "react";

import { useRouter } from "next/dist/client/router";
import { Input, Form, Button } from "nhsuk-react-components";
import { useForm, useFieldArray } from "react-hook-form/dist/index.ie11";

import { StatusTag } from "../../components/StatusTag";
import { Textarea } from "../../components/Textarea";
import {
  useCreateFileMutation,
  useFileUploadUrlsMutation,
} from "../../lib/generated/graphql";
import { uploadBlob } from "../../lib/uploadBlob";
import { useMaxLengthHelper } from "../../lib/useMaxLengthHelper";
import {
  FormField,
  StyledButton,
  StyledFileInfoBox,
  StyledFileName,
  StyledHeadingSection,
  StyledInput,
  StyledTag,
} from "./styles";

interface FormData {
  files: FileList;
  fileData: Array<{ title: string; description: string }>;
}

interface Props {
  folderId: string;
  workspaceId: string;
}

const CreateFileForm: FC<Props> = ({ workspaceId, folderId }) => {
  const {
    control,
    errors,
    handleSubmit,
    register,
    setError,
    setValue,
  } = useForm<FormData>();
  const [, createFile] = useCreateFileMutation();
  const [, fileUploadUrls] = useFileUploadUrlsMutation();

  const router = useRouter();
  const backToPreviousPage = () => router.back();

  const [results, setResults] = useState<Array<Boolean>>([]);
  const [isDisabled, disableButton] = useState<boolean>(false);

  const titleMaxLength = useMaxLengthHelper("Title", 50);
  const descriptionMaxLength = useMaxLengthHelper("Description", 250);

  const { fields } = useFieldArray({
    control,
    name: "fileData",
  });

  const handleFiles = (files: FileList | null) => {
    if (files === null) {
      return;
    }

    const filenames = Object.values(files).map((file) =>
      file.name.replaceAll("_", " ").split(".").slice(0, -1).join(".")
    );
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
      const { error, data } = await fileUploadUrls({
        count: formData.files.length,
      });
      if (error) {
        throw new Error(`Failed to get upload URL: ${error.toString()}`);
      }

      if (data) {
        const results: Array<boolean> = await Promise.all(
          data.fileUploadUrls.map(
            async (url: string, index: number): Promise<boolean> => {
              try {
                await uploadBlob(url, formData.files[index]);

                const file = await createFile({
                  newFile: {
                    description: formData.fileData[index].description,
                    fileName: formData.files[index].name,
                    fileType: formData.files[index].type,
                    folder: folderId,
                    temporaryBlobStoragePath: url,
                    title: formData.fileData[index].title,
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
                onChange={titleMaxLength.onChange}
                inputRef={register({
                  required: {
                    value: true,
                    message: "Title is required",
                  },
                  ...titleMaxLength.validation,
                })}
                error={errors.fileData?.[index]?.title?.message}
              />
              {titleMaxLength.remainingText(`fileData[${index}].title`)}
            </FormField>
            <FormField>
              <Textarea
                name={`fileData[${index}].description`}
                label="Enter description (optional)"
                error={errors.fileData?.[index]?.description?.message}
                hint="This is the description as seen by users"
                onChange={descriptionMaxLength.onChange}
                inputRef={register(descriptionMaxLength.validation)}
              />
              {descriptionMaxLength.remainingText(
                `fileData[${index}].description`
              )}
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

export default CreateFileForm;
