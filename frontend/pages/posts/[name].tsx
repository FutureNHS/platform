import React, { useState } from "react";
import Link from "next/link";

import { getGreeting } from "../../lib/posts";
import { GetServerSideProps } from "next";

export const getServerSideProps: GetServerSideProps = async (context) => {
  const greeting = await getGreeting(context?.params?.name);

  return {
    props: {
      greeting,
    },
  };
};

const FirstPost = ({ greeting }: { greeting: object }) => {
  const [inputValue, setInputValue] = useState("");
  return (
    <>
      <section>
        <div>Enter a name</div>

        <div>{greeting}</div>
        <input
          value={inputValue}
          onChange={(e) => setInputValue(e.target.value)}
        />
        <button
          onClick={() => {
            window.location.href = `/posts/${inputValue}`;
          }}
        >
          greet
        </button>
      </section>
      <section>
        <Link href="/">
          <a>home</a>
        </Link>
      </section>
    </>
  );
};

export default FirstPost;
