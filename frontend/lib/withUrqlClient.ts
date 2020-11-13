import { devtoolsExchange } from "@urql/devtools";
import {
  cacheExchange,
  Cache,
  Data,
  UpdatesConfig,
} from "@urql/exchange-graphcache";
import { NextPage } from "next";
import { withUrqlClient as withUrqlClientImpl } from "next-urql";
import NextApp from "next/app";
import { dedupExchange, fetchExchange } from "urql";

import { User } from "./auth";
import {
  FilesByFolderDocument,
  FilesByFolderQuery,
  FoldersByWorkspaceDocument,
  FoldersByWorkspaceQuery,
  Mutation,
  MutationCreateFileArgs,
  MutationDeleteFileArgs,
  MutationCreateFolderArgs,
} from "./generated/graphql";
import { requireEnv } from "./server/requireEnv";

const isServerSide = typeof window === "undefined";
const workspaceAPIServerUrl = isServerSide
  ? requireEnv("WORKSPACE_SERVICE_GRAPHQL_ENDPOINT")
  : "/api/graphql";

/**
 * Creates an updater for a given Query type.
 *
 * This doesn't change any behaviour. It's purely a type cast.
 */
const updateWithNull = <T extends { __typename?: "Query" }>(
  handler: (data: (Data & T) | null) => (Data & T) | null
): ((data: Data | null) => Data | null) => handler as any;

/**
 * Creates an updater for a given Query type, which performs no update if the
 * currently cached query result is null.
 */
const update = <T extends { __typename?: "Query" }>(
  handler: (data: Data & T) => (Data & T) | null
) => updateWithNull<T>((data) => (data === null ? null : handler(data)));

type PartialChildren<T> = {
  [P in keyof T]: Partial<T[P]>;
};

type MutationUpdatesConfig = {
  [K in keyof Omit<Mutation, "__typename">]?: (
    result: Data & PartialChildren<Pick<Mutation, K>>,
    // TODO: Ideally the typescript generator could generate a map with the
    // same keys as `Mutation` where the values are the corresponding argument
    // type, e.g.:
    //
    // ```
    // type MutationArgs = {
    //   createFile: MutationCreateFileArgs,
    //   createFolder: MutationCreateFolderArgs,
    // };
    // ```
    //
    // As long as it doesn't we type this as `any`, which lets us add a type
    // to the function parameter.
    args: any,
    cache: Cache
  ) => void;
};

const mutationUpdateResolvers: MutationUpdatesConfig = {
  createFolder: (result, args: MutationCreateFolderArgs, cache) => {
    const { __typename, id, title } = result.createFolder;
    if (id === undefined || title === undefined) {
      cache.invalidate("Query", "foldersByWorkspace", {
        workspace: args.newFolder.workspace,
      });
    } else {
      cache.updateQuery(
        {
          query: FoldersByWorkspaceDocument,
          variables: { workspace: args.newFolder.workspace },
        },
        update<FoldersByWorkspaceQuery>((data) => {
          data.foldersByWorkspace.push({ __typename, id, title });
          return data;
        })
      );
    }
  },
  deleteFile: (_result, args: MutationDeleteFileArgs, cache) => {
    cache.invalidate({ __typename: "File", id: args.id });
  },
  createFile: (result, args: MutationCreateFileArgs, cache) => {
    const {
      __typename,
      id,
      folder,
      title,
      description,
      fileName,
      fileType,
      modifiedAt,
    } = result.createFile;
    if (
      id === undefined ||
      folder === undefined ||
      title === undefined ||
      description === undefined ||
      fileName === undefined ||
      fileType === undefined ||
      modifiedAt === undefined
    ) {
      cache.invalidate("Query", "filesByFolder", {
        folder: args.newFile.folder,
      });
    } else {
      cache.updateQuery(
        {
          query: FilesByFolderDocument,
          variables: { folder: args.newFile.folder },
        },
        update<FilesByFolderQuery>((data) => {
          data.filesByFolder.push({
            __typename,
            id,
            folder,
            title,
            description,
            fileName,
            fileType,
            modifiedAt,
          });
          return data;
        })
      );
    }
  },
};

export default function withUrqlClient(
  component: NextPage<any> | typeof NextApp
) {
  return withUrqlClientImpl(
    (ssrExchange, ctx) => {
      if (ctx && ctx.req && ctx.res) {
        // @ts-ignore
        const user: User = ctx.req.user;
        if (!user) {
          ctx.res.writeHead(302, {
            Location: `/auth/login?next=${ctx.req.url}`,
          });
          ctx.res.end();
        }
      }

      const exchanges = [
        dedupExchange,
        cacheExchange({
          keys: {},
          updates: {
            Mutation: mutationUpdateResolvers as UpdatesConfig["Mutation"],
          },
        }),
        ssrExchange,
        fetchExchange,
      ];

      if (process.env.NODE_ENV !== "production") {
        exchanges.unshift(devtoolsExchange);
      }
      const fetchOptions = isServerSide
        ? {
            headers: {
              // @ts-ignore
              "x-user-auth-id": ctx?.req?.user?.authId,
            },
          }
        : undefined;
      return {
        fetchOptions,
        exchanges,
        url: workspaceAPIServerUrl,
      };
    },
    { ssr: true }
  )(component);
}
