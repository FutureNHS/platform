import React, { useState } from "react";

import { GraphQLClient } from "graphql-request";
import { GetServerSideProps } from "next";
import { Input, Form, Button } from "nhsuk-react-components";
import { useForm } from "react-hook-form";
import styled from "styled-components";

import { requireAuthentication } from "../../lib/auth";
import { getSdk } from "../../lib/generated/graphql";
import { MainHeading } from "../MainHeading";
import { Textarea } from "../Textarea";

export const getServerSideProps: GetServerSideProps = requireAuthentication(
  async () => {
    return {
      props: {},
    };
  }
);

const MAX_CHARS: { [key: string]: number } = {
  title: 100,
  description: 250,
};

interface Workspace {
  id: string;
  title: string;
  description: string;
}

const FormField = styled.div`
  padding-bottom: 40px;
  #text {
    padding-bottom: 60px;
  }
`;

const CreateFolder = () => {
  const [remainingChars, setRemainingChars] = useState({
    title: null,
    description: null,
  });

  const { errors, handleSubmit, register } = useForm();

  const onSubmit = async (data: Workspace) => {
    try {
      const client = new GraphQLClient("/api/graphql");
      const sdk = getSdk(client);
      await sdk.CreateWorkspaceMutation(data);
      window.alert("Workspace created successfully");
    } catch (error) {
      console.log("Create workspace failed", error);
      window.alert("Error creating workspace, failed");
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
      <MainHeading>Create a folder</MainHeading>
      <h2>Folder details</h2>
      <p> Fields marked with * are required.</p>
      <Form onSubmit={handleSubmit(onSubmit)}>
        <FormField>
          <Input
            name="title"
            onChange={handleCharNumber}
            id="title"
            label="Name of workspace*"
            hint="This is the name of the workspace as seen by users of FutureNHS."
            inputRef={register({
              required: true,
              maxLength: MAX_CHARS.title,
            })}
            error={
              errors.title &&
              `Workspace name is required and cannot be longer than ${MAX_CHARS.title} characters`
            }
          />
          {`${remainingChars.title || MAX_CHARS.title} characters remaining`}
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
            hint="This is the description as seen by users. Try to be as descriptive as possible."
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
      </Form>
    </>
  );
};

export default CreateFolder;
