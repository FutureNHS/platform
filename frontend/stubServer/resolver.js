const changeWorkspaceMembershipResponse = require("../cypress/fixtures/change-workspace-membership-response.json");
const createFolderResponse = require("../cypress/fixtures/create-folder-graphql-response.json");
const createWorkspaceResponse = require("../cypress/fixtures/create-workspace-graphql-response.json");
const deleteFileResponse = require("../cypress/fixtures/delete-file-graphql-response.json");
const fileResponse = require("../cypress/fixtures/file-graphql-response.json");
const fileUploadUrlsResponse = require("../cypress/fixtures/file-upload-url-graphql-response.json");
const filesByFolderResponse = require("../cypress/fixtures/files-by-folder-graphql-response.json");
const folderResponse = require("../cypress/fixtures/folder-graphql-response.json");
const foldersByWorkspaceResponse = require("../cypress/fixtures/folders-by-workspace-graphql-response.json");
const getOrCreateUserResponse = require("../cypress/fixtures/get-or-create-user-graphql-response.json");
const membersResponse = require("../cypress/fixtures/members-graphql-response.json");
const requestingUserWorkspaceRights = require("../cypress/fixtures/requesting-user-workspace-rights.json");
const updateFolderResponse = require("../cypress/fixtures/update-folder-graphql-response.json");
const workspaceResponse = require("../cypress/fixtures/workspace-graphql-response.json");
// Workspace
const workspacesResolver = {
  workspaces: async () => workspaceResponse.data.workspaces,
  workspace: async () => ({
    id: membersResponse.data.workspace.id,
    title: membersResponse.data.workspace.title,
    members: ({ filter }) =>
      filter === "ADMIN"
        ? membersResponse.data.workspace.admins
        : membersResponse.data.workspace.members,
  }),
  requestingUserWorkspaceRights: async () =>
    requestingUserWorkspaceRights.data.requestingUserWorkspaceRights,
};

const workspaceMutation = {
  createWorkspace: async () => createWorkspaceResponse.data.createWorkspace,
};

// File
const fileResolver = {
  filesByFolder: async () => filesByFolderResponse.data.filesByFolder,
  file: async () => fileResponse.data.file,
};

const fileMutation = {
  deleteFile: async () => deleteFileResponse.data.deleteFile,
};

const fileUploadUrlsMutation = {
  fileUploadUrls: async () => fileUploadUrlsResponse.data.fileUploadUrls,
};

// Folder
const folderResolver = {
  foldersByWorkspace: async () =>
    foldersByWorkspaceResponse.data.foldersByWorkspace,
  folder: async () => folderResponse.data.folder,
};

const folderMutation = {
  createFolder: async () => createFolderResponse.data.createFolder,
  updateFolder: async () => updateFolderResponse.data.updateFolder,
};

// User
const userMutation = {
  getOrCreateUser: async () => getOrCreateUserResponse.data.getOrCreateUser,
};

// Members
const membersMutation = {
  changeWorkspaceMembership: async () =>
    changeWorkspaceMembershipResponse.data.changeWorkspaceMembership,
};

module.exports = (schema) => {
  const federationResolver = {
    _service: async () => ({
      sdl: schema,
    }),
  };

  return {
    Query: {
      ...federationResolver,
      ...fileResolver,
      ...folderResolver,
      ...workspacesResolver,
    },
    Mutation: {
      ...fileMutation,
      ...fileUploadUrlsMutation,
      ...folderMutation,
      ...userMutation,
      ...workspaceMutation,
      ...membersMutation,
    },
  };
};
