import React, { createRef, useEffect } from "react";
import { PageLayout } from "../../components/PageLayout";
import { Header } from "../../components/Header";
import Head from "next/head";
import Link from "next/link";

const AccessibilityTest = () => {
  const contentRef: any = createRef();
  useEffect(() => {
    if (contentRef && contentRef.current) {
      contentRef.current.focus();
    }
  }, []);

  return (
    <PageLayout>
      <Header
        imageRight={require("../../public/NHS.png")}
        imageRightURL="https://www.nhs.co.uk"
        imageRightAltText="NHS logo"
        aria-hidden="true"
      />
      <Head>
        <title>FutureNHS Accessibility Page 2</title>
      </Head>
      <nav>
        Nav Bar:
        <Link href="/accessible/page1">
          <a>Page 1</a>
        </Link>
      </nav>
      <header tabIndex={-1}>
        <h1 ref={contentRef} tabIndex={0}>
          This is a Header - NextJS
        </h1>
      </header>

      <main>This is NEXTJS page TWO</main>
    </PageLayout>
  );
};

export default AccessibilityTest;
