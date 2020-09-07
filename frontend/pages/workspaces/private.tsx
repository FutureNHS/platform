import React from "react";
import { GetServerSideProps } from "next";
import { requireAuthentication, User } from "../../lib/auth";
import { MainHeading } from "../../components/MainHeading";

interface Props {
  user: User;
}

export const getServerSideProps: GetServerSideProps<Props> = requireAuthentication(
  async (_context, user) => {
    return {
      props: { user },
    };
  }
);

const PrivatePage = (props: Props) => {
  return (
    <>
      <MainHeading>Private Page</MainHeading>
      <div>User ID: {props.user.id}</div>
      <div>Name: {props.user.name}</div>
      <div>Emails: {props.user.emails.join(", ")}</div>
    </>
  );
};

export default PrivatePage;
