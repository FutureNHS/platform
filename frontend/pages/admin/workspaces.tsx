import React, { useState } from "react";
import { useForm } from "react-hook-form";
import { GraphQLClient } from "graphql-request";
import { getSdk } from "../../lib/generated/graphql";
import { Head } from "../../components/Head";
import { Header } from "../../components/Header";
import { PageLayout } from "../../components/PageLayout";
import { Textarea } from "../../components/Textarea";
import styled from "styled-components";
import { Input, Form, Button } from "nhsuk-react-components";
import { GetServerSideProps } from "next";
import { requireAuthentication } from "../../lib/auth";
import { MainHeading } from "../../components/MainHeading";

export const getServerSideProps: GetServerSideProps = requireAuthentication(
  async () => {
    return {
      props: {},
    };
  }
);

const MAX_CHARS: { [key: string]: number } = {
  title: 100,
  longDescription: 250,
};

interface Workspace {
  id: string;
  title: string;
  longDescription: string;
}

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

const H2 = styled.h2`
  ${({ theme }) => `
  border-top: 1px solid ${theme.colorNhsukGrey1};
  padding-top: 24px;
  margin-bottom: 8px;
  color: ${theme.colorNhsukGrey1}
  `}
`;

const FormField = styled.div`
  padding-bottom: 40px;
  #text {
    padding-bottom: 60px;
  }
`;

const CreateWorkspace = () => {
  const [remainingChars, setRemainingChars] = useState({
    title: null,
    longDescription: null,
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
      <Head title="Admin - Create Workspace" />
      <PageLayout>
        <Header />
        <PageContent>
          <MainHeading>Create a workspace</MainHeading>
          <H2>Workspace details</H2>
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
              {`${
                remainingChars.title || MAX_CHARS.title
              } characters remaining`}
            </FormField>

            <FormField>
              <Textarea
                name="longDescription"
                onChange={handleCharNumber}
                id="longDescription"
                label="Description"
                error={
                  errors.longDescription &&
                  `Description must be a maximum of ${MAX_CHARS.longDescription} characters`
                }
                hint="This is the description as seen by users. Try to be as descriptive as possible."
                inputRef={register({
                  required: false,
                  maxLength: MAX_CHARS.longDescription,
                })}
              />
              {`${
                remainingChars.longDescription || MAX_CHARS.longDescription
              } characters remaining`}
            </FormField>
            <Button type="submit">Save and complete</Button>
          </Form>
        </PageContent>
      </PageLayout>
    </>
  );
};

export default CreateWorkspace;
