import React from "react";
import Head from "next/head";
import styles from "./layout.module.css";
import utilStyles from "../styles/utils.module.css";
import Link from "next/link";

export default function Layout({ children }: { children: React.ReactNode }) {
  return <div>{children}</div>;
}
