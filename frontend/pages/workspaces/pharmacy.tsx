import React from "react";
import { GetServerSideProps } from "next";
import { requireAuthentication } from "../../utils/pages/auth";

export const getServerSideProps: GetServerSideProps = async (context) => {
  await requireAuthentication(context);
  return {
    props: {},
  };
};

const PrivatePage = () => {
  return (
    <>
      <div>Private Pharmacy Page</div>
      <div>This is a different private page</div>
    </>
  );
};

export default PrivatePage;
