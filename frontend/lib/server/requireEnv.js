// @ts-check

// Hack to make typescript keep the list of valid names in sync with our
// healthcheck endpoint. If this was a typescript file, we would write:
//
//     const ENV_VAR_NAMES = [...] as const
//
// and type `name` as:
//
//     typeof ENV_VAR_NAMES[number]
//
// Unfortunately, `as const` is not valid javascript, and there isn't
// a JSDoc equivalent, so we have to use an object, and type `name` as:
//
//     keyof typeof ENV_VAR_NAMES
//
const ENV_VAR_NAMES = {
  EVENTGRID_TOPIC_ENDPOINT: 1,
  EVENTGRID_TOPIC_KEY: 1,
  ORIGIN: 1,
  PG_URL: 1,
  WORKSPACE_SERVICE_GRAPHQL_ENDPOINT: 1,
};

/**
 * @param {keyof typeof ENV_VAR_NAMES} name
 * @returns {string}
 */
const requireEnv = (name) => {
  const value = process.env[name];
  if (!value) {
    throw new Error(`Environment variable ${name} is required`);
  }

  return value;
};

module.exports = { ENV_VAR_NAMES, requireEnv };
