import React, { FC } from "react";

import { useRouter } from "next/dist/client/router";
import { Input, Form, Button } from "nhsuk-react-components";
import { useForm } from "react-hook-form/dist/index.ie11";

import { Textarea } from "../../components/Textarea";
import { useCreateFileVersionMutation } from "../../lib/generated/graphql";
import { useMaxLengthHelper } from "../../lib/useMaxLengthHelper";
import { FormFieldNoSidePadding, StyledButton } from "./styles";

interface FormData {
  fileData: { title: string; description: string };
}

interface Props {
  fileDescription: string;
  fileId: string;
  fileTitle: string;
  folderId: string;
  latestVersionId: string;
  workspaceId: string;
}

const UpdateDetailsFileForm: FC<Props> = ({
  fileDescription,
  fileId,
  fileTitle,
  folderId,
  latestVersionId,
  workspaceId,
}) => {
  const { register, handleSubmit, errors, setError } = useForm<FormData>();
  const [, createFileVersion] = useCreateFileVersionMutation();

  const router = useRouter();
  const backToPreviousPage = () => router.back();

  const titleMaxLength = useMaxLengthHelper("Title", 50);
  const descriptionMaxLength = useMaxLengthHelper("Description", 250);

  const onSubmit = async (formData: FormData) => {
    try {
      const file = await createFileVersion({
        newFileVersion: {
          description: formData.fileData.description,
          file: fileId,
          latestVersion: latestVersionId,
          title: formData.fileData.title,
        },
      });

      if (file.error) {
        throw new Error(`Failed to save file: ${file.error?.message}`);
      }

      router.push(
        `/workspaces/${workspaceId}/folders/${folderId}/files/${fileId}`
      );
    } catch (error) {
      setError("fileData.title", {
        type: "server",
        message: error.toString(),
      });
    }
  };

  return (
    <Form id="filesUploadForm" onSubmit={handleSubmit(onSubmit)}>
      <FormFieldNoSidePadding>
        <Input
          type="text"
          name={`fileData.title`}
          defaultValue={fileTitle}
          label="Edit file title*"
          hint="This is the file title as seen by members. Try to be as descriptive as possible. Avoid including
              underscores, hyphens, or document type details (.doc, .pdf...)"
          onChange={titleMaxLength.onChange}
          inputRef={register({
            required: {
              value: true,
              message: "Title is required",
            },
            ...titleMaxLength.validation,
          })}
          error={errors.fileData?.title?.message}
        />
        {titleMaxLength.remainingText(`fileData.title`)}
      </FormFieldNoSidePadding>
      <FormFieldNoSidePadding>
        <Textarea
          name={`fileData.description`}
          defaultValue={fileDescription}
          label="Edit description"
          error={errors.fileData?.description?.message}
          hint="This is the description of the content that members will see. Be as clear and descriptive as possible."
          onChange={descriptionMaxLength.onChange}
          inputRef={register(descriptionMaxLength.validation)}
        />
        {descriptionMaxLength.remainingText(`fileData.description`)}
      </FormFieldNoSidePadding>
      <Button type="submit" name="submitButton">
        Save and complete
      </Button>
      <StyledButton secondary type="button" onClick={backToPreviousPage}>
        Discard
      </StyledButton>
    </Form>
  );
};

export default UpdateDetailsFileForm;
