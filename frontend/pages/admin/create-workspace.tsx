import React from "react";

import { NextPage } from "next";
import { Input, Form, Button } from "nhsuk-react-components";
import { useForm } from "react-hook-form/dist/index.ie11";
import styled from "styled-components";

import { Error } from "../../components/Error";
import { Footer } from "../../components/Footer";
import { H2 } from "../../components/H2";
import { Head } from "../../components/Head";
import { MainHeading } from "../../components/MainHeading";
import { NavHeader } from "../../components/NavHeader";
import { PageLayout } from "../../components/PageLayout";
import { Textarea } from "../../components/Textarea";
import { User } from "../../lib/auth";
import {
  useCreateWorkspaceMutation,
  Workspace,
} from "../../lib/generated/graphql";
import { useMaxLengthHelper } from "../../lib/useMaxLengthHelper";
import withUrqlClient from "../../lib/withUrqlClient";

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

const FormField = styled.div`
  padding-bottom: 40px;
  #text {
    padding-bottom: 60px;
  }
`;

interface InitialProps {
  isPlatformAdmin: boolean;
}

export const CreateWorkspace: NextPage<InitialProps> = ({
  isPlatformAdmin,
}) => {
  const titleMaxLength = useMaxLengthHelper("Title", 100);
  const descriptionMaxLength = useMaxLengthHelper("Description", 250);
  const { errors, handleSubmit, register } = useForm();
  const [, createWorkspace] = useCreateWorkspaceMutation();

  const onSubmit = (data: Workspace) =>
    createWorkspace(data).then((result) => {
      if (result.data) {
        window.alert("Workspace created successfully");
      } else {
        console.log("Create workspace failed", result.error);
        window.alert("Error creating workspace, failed");
      }
    });

  return (
    <>
      <Head title="Admin - Create Workspace" />
      <PageLayout>
        <NavHeader />
        <PageContent>
          {isPlatformAdmin ? (
            <>
              <MainHeading>Create a workspace</MainHeading>
              <H2 title="Workspace details" />
              <p> Fields marked with * are required.</p>

              <Form onSubmit={handleSubmit(onSubmit)}>
                <FormField>
                  <Input
                    name="title"
                    onChange={titleMaxLength.onChange}
                    id="title"
                    label="Name of workspace*"
                    hint="This is the name of the workspace as seen by users of FutureNHS."
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
                    hint="This is the description as seen by users. Try to be as descriptive as possible."
                    inputRef={register(descriptionMaxLength.validation)}
                  />
                  {descriptionMaxLength.remainingText("description")}
                </FormField>
                <Button type="submit">Save and complete</Button>
              </Form>
            </>
          ) : (
            <>
              <Error
                title="You do not have permission to do this."
                description="Please contact your Platform Administrator to request a
                workspace."
              />
            </>
          )}
        </PageContent>
        <Footer />
      </PageLayout>
    </>
  );
};

CreateWorkspace.getInitialProps = async (context): Promise<InitialProps> => {
  // @ts-ignore
  const user: User | undefined = context?.req?.user;
  return { isPlatformAdmin: user?.isPlatformAdmin || false };
};

export default withUrqlClient(CreateWorkspace);
