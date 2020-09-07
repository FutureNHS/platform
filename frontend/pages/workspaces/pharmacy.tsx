import React from "react";
import { GetServerSideProps } from "next";
import { requireAuthentication } from "../../lib/auth";
import { Header } from "../../components/Header";
import Link from "next/link";
import { MainHeading } from "../../components/MainHeading";

export const getServerSideProps: GetServerSideProps = requireAuthentication(
  async () => {
    return {
      props: {},
    };
  }
);

const PrivatePage = () => {
  return (
    <>
      <Header />
      <MainHeading>Pharmacy Page</MainHeading>
      <p>This is a different private page</p>
      <Link href="/workspaces/private">
        <a>Private workspace</a>
      </Link>
    </>
  );
};

export default PrivatePage;
